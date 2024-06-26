use std::{io, time::Duration};
use futures_lite::FutureExt;
use async_io::{block_on, Timer};

use crate::tcan4550::{
    filter::{SIDFCONFIG, XIDFCONFIG},
    register::*,
    configuration::mram::*,
    request::TCAN455xRequest,
};

use crate::rx_buffer::RxData;

use crate::driver::raspberrypi::{RaspiIF, GPIO_INPUT_PIN_NUM};

const BUFSIZE: usize = 512;

pub struct TCAN455xTranceiver {
    driver: RaspiIF,
    rx_buf: [u8; BUFSIZE],
}

impl TCAN455xTranceiver {
    pub fn new () -> Result<Self, Box<dyn std::error::Error>> {
        
        let driver = match RaspiIF::new() {
            Ok(x) => x,
            Err(e) => return Err(e)
        };

        Ok(Self {
            driver,
            rx_buf: [0; BUFSIZE]
        })
    }


    pub fn gpi_read(&mut self, channel: usize) -> bool {
        self.driver.gpi_read(channel)
    }

    pub fn gpi_read_all(&mut self) -> [bool; GPIO_INPUT_PIN_NUM] {
        self.driver.gpi_read_all()
    }


    pub fn write(&mut self, data: &[u8]) -> io::Result<usize> {
        match self.driver.spi_write(data) {
            Ok(size) => Ok(size),
            Err(_) => Err(io::ErrorKind::ConnectionRefused.into())
        }
    }

    pub fn read(&mut self, addr: u16, len: u8) -> io::Result<usize> {
        let req: Vec<u8> = TCAN455xRequest::get_read_command(addr, len);
        match self.driver.spi_transfer(&mut self.rx_buf, &req) {
            Ok(size) => Ok(size),
            Err(_) => Err(io::ErrorKind::ConnectionRefused.into())
        }
    }

    pub fn read_bytes(&mut self, addr: u16, len: u8) -> io::Result<Vec<u8>> {
        match self.read(addr, len) {
            Ok(size) => {
                let v = self.rx_buf[4..size]
                    .chunks(4)
                    .map(|x| vec![x[3], x[2], x[1], x[0]])
                    .flatten()
                    .collect();
                Ok(v)
            },
            Err(_) => Err(io::ErrorKind::InvalidData.into())
        }
    }

    pub fn read_device(&mut self, addr: u16) -> io::Result<u32> {
        let status: u32 = match self.read(addr, 1) {
            Ok(size) => {
                match &self.rx_buf[4..size].try_into() {
                    Ok(x) => u32::from_be_bytes(*x),
                    Err(_) => return Err(io::ErrorKind::InvalidData.into())
                }
            },
            Err(_) => return Err(io::ErrorKind::InvalidData.into())
        };
        Ok(status)
    }

    pub fn setup(&mut self, sidf: &[SIDFCONFIG], xidf: &[XIDFCONFIG]) -> Result<(), String>{

        Self::reset(self);
        Self::switch_standby_mode(self).expect("Cannot shift into standby mode");
    
        // Clear SPI error
        Self::clear_spi_error(self).expect("Cannot clear SPI error");
    
        // Clear device interrupt flags
        let dev_ir: u32 = Self::read_device_irq(self).expect("Cannot read device irq");
        if (dev_ir >> 20) & 0x01 == 0x01 {
            Self::clear_device_irq_flags(self, dev_ir).expect("Cannot clear device irq flags");
        }

        // Unprotect config registers
        Self::lock_mcan_cccr(self).expect("Cannot lock MCAN CCCR");
    
        // Configuration::enable CAN FD
        Self::configure_mcan_cccr(self).expect("Cannot configure MCAN CCCR");
        // Configuration::global filter
        Self::configure_global_filter(self).expect("Cannot configure global filter");
        // Configuration::bit timing
        Self::configure_bit_timing(self).expect("Cannot configure bit timing");
        // Configuration::clear MRAM
        Self::clear_mram(self).expect("Cannot clear MRAM");
        // Configuration::MRAM
        Self::configure_mram(self).expect("Cannot configure MRAM");
        // Configuration::SID and XID filter
        Self::configure_filter(self, sidf, xidf).expect("Cannot configure SID/XID filter");
    
        // Protect config registers
        Self::unlock_mcan_cccr(self).expect("Cannot unlock MCAN CCCR ");
    
        // Test mode: Only enable when REG_MCAN_CCCR[7] = 1 & REG_MCAN_CCCR[5] = 1
        Self::switch_test_mode(self).expect("Cannot shift into test mode");
    
        // Configure MCAN IRQ
        Self::configure_mcan_irq(self).expect("Cannot configure MCAN IRQ");
    
        // Configure device
        Self::configure_mode_and_pins(self).expect("Cannot configure mode and pins");
    
        Self::switch_normal_mode(self).expect("Cannot shift into normal mode");
    
        // Clear all MCAN interrupt flags
        Self::clear_mcan_irq_flags(self).expect("Cannot clear MCAN IRQ");

        Ok(())
    }

    pub fn reset(&mut self) {
        self.driver.reset_tcan455x();
    }

    pub fn close(&mut self) -> io::Result<()> {
        Self::switch_sleep_mode(self)
    }

    async fn timeout<T>() -> std::io::Result<T> {
        Timer::after(Duration::from_millis(500)).await;
        Err(io::ErrorKind::TimedOut.into())
    }

    pub fn get_device_id(&mut self) -> io::Result<String>{
        let fut = async {
            let info: Vec<u8> = match self.read_bytes(REG_SPI_DEVICE_ID0, 2) {
                Ok(val) => val,
                Err(_) => return Err(io::ErrorKind::InvalidData.into())
            };
            let info: String = String::from_utf8_lossy(&info).to_string();
            Ok(info)
        };
        block_on(fut.or(Self::timeout()))
    }
    
    pub fn read_spi_status(&mut self) -> io::Result<u32>{
        let fut = async {
            match self.read_device(REG_SPI_STATUS) {
                Ok(val) => Ok(val),
                Err(_) => Err(io::ErrorKind::InvalidData.into())
            }
        };
        block_on(fut.or(Self::timeout()))
    }
    
    pub fn read_device_irq(&mut self) -> io::Result<u32>{
        let fut = async {
            match self.read_device(REG_DEV_IR) {
                Ok(val) => Ok(val),
                Err(_) => Err(io::ErrorKind::InvalidData.into())
            }
        };
        block_on(fut.or(Self::timeout()))
    }

    pub fn clear_spi_error(&mut self) -> io::Result<()> {
        let fut = async {
            let cmd: Vec<u8> = TCAN455xRequest::get_write_command(REG_SPI_STATUS, vec![0xFFFFFFFF]);
            self.write(&cmd)?;
            Ok(())
        };
        block_on(fut.or(Self::timeout()))
    }

    pub fn clear_device_irq_flags(&mut self, dev_ir: u32) -> io::Result<()> {
        let fut = async {
            let cmd: Vec<u8> = TCAN455xRequest::get_write_command(REG_DEV_IR, vec![dev_ir]);
            self.write(&cmd)?;
            Ok(())
        };
        block_on(fut.or(Self::timeout()))
    }

    pub fn clear_mcan_irq_flags(&mut self) -> io::Result<()> {
        let fut = async {
            let cmd: Vec<u8> = TCAN455xRequest::get_write_command(REG_MCAN_IR, vec![0xFFFFFFFF]);
            self.write(&cmd)?;
            Ok(())
        };
        block_on(fut.or(Self::timeout()))
    }

    pub fn clear_mram(&mut self) -> io::Result<()> {
        let fut = async {
            const MRAM_SIZE: u16 = 2048;
            for addr in (REG_MRAM..(REG_MRAM + MRAM_SIZE)).step_by(4) {
                let cmd: Vec<u8> = TCAN455xRequest::get_write_command(addr, vec![0]);
                self.write(&cmd)?;
            }
            Ok(())
        };
        block_on(fut.or(Self::timeout()))
    }
    
    pub fn lock_mcan_cccr(&mut self) -> io::Result<()> {
        let fut = async {
            let ccr: u32 = match self.read_device(REG_MCAN_CCCR) {
                Ok(val) => val,
                Err(_) => return Err(io::ErrorKind::InvalidData.into())
            };
            let cce: u32 = TCAN455xRequest::unprotect_register(ccr);
            let cmd: Vec<u8> = TCAN455xRequest::get_write_command(REG_MCAN_CCCR, vec![cce]);
            self.write(&cmd)?;
            Ok(())
        };
        block_on(fut.or(Self::timeout()))
    }

    pub fn unlock_mcan_cccr(&mut self) -> io::Result<()> {
        let fut = async {
            let ccr: u32 = match self.read_device(REG_MCAN_CCCR) {
                Ok(val) => val,
                Err(_) => return Err(io::ErrorKind::InvalidData.into())
            };
            let ccd: u32 = TCAN455xRequest::protect_register(ccr);
            let cmd: Vec<u8> = TCAN455xRequest::get_write_command(REG_MCAN_CCCR, vec![ccd]);
            self.write(&cmd)?;
            Ok(())
        };
        block_on(fut.or(Self::timeout()))
    }

    pub fn configure_global_filter(&mut self) -> io::Result<()> {
        let fut = async {
            const ANFS: u32 = 1;   // Incoming message doesn't match a filter is to accept into RXFIO0 for SID messages (11 bit IDs)
            const ANFE: u32 = 1;   // Incoming message doesn't match a filter is to accept into RXFIO0 for XID messages (29 bit IDs)
            const RRFS: u32 = 0;   // Reject remote frames (TCAN4x5x doesn't support this)
            const RRFE: u32 = 0;   // Reject remote frames (TCAN4x5x doesn't support this)
            const PAYLOAD: u32 = ((ANFS << 4) | (ANFE << 2) | (RRFS << 1) | (RRFE << 0)) & REG_BITS_MCAN_GFC_MASK;
            let cmd: Vec<u8> = TCAN455xRequest::get_write_command(REG_MCAN_GFC, vec![PAYLOAD]);
            self.write(&cmd)?;
            Ok(())
        };
        block_on(fut.or(Self::timeout()))
    }

    pub fn configure_mcan_cccr(&mut self) -> io::Result<()> {
        let fut = async {
            self.write(&TCAN455xRequest::set_mcan_cccr())?;
            Ok(())
        };
        block_on(fut.or(Self::timeout())) 
    }

    pub fn configure_bit_timing(&mut self) -> io::Result<()> {
        let fut = async {
            self.write(&TCAN455xRequest::set_dbtp())?;
            self.write(&TCAN455xRequest::set_nbtp())?;
            self.write(&TCAN455xRequest::set_tdcr())?;
            self.write(&TCAN455xRequest::set_tscc())?;
            Ok(())
        };
        block_on(fut.or(Self::timeout())) 
    }

    pub fn configure_mram(&mut self) -> io::Result<()> {
    // Following registers cannot change unless Configuration Change Enable (CCE) = HIGH
        let fut = async {
            self.write(&TCAN455xRequest::set_sidfc())?;
            self.write(&TCAN455xRequest::set_xidfc())?;
            self.write(&TCAN455xRequest::set_rxf0c())?;
            self.write(&TCAN455xRequest::set_rxf1c())?;
            self.write(&TCAN455xRequest::set_rxbc())?;
            self.write(&TCAN455xRequest::set_rxesc())?;
            self.write(&TCAN455xRequest::set_txefc())?;
            self.write(&TCAN455xRequest::set_txbc())?;
            self.write(&TCAN455xRequest::set_txesc())?;
            Ok(())
        };
        block_on(fut.or(Self::timeout())) 
    }

    pub fn configure_mode_and_pins(&mut self) -> io::Result<()> {
        let fut = async {
            self.write(&TCAN455xRequest::set_device_modes_and_pins())?;
            Ok(())
        };
        block_on(fut.or(Self::timeout()))
    }

    pub fn configure_mcan_irq(&mut self) -> io::Result<()> {
        let fut = async {
            self.write(&TCAN455xRequest::set_mcan_ie())?;
            self.write(&TCAN455xRequest::set_mcan_ile())?;
            Ok(())
        };
        block_on(fut.or(Self::timeout())) 
    }

    pub fn configure_filter(&mut self, sidf: &[SIDFCONFIG], xidf: &[XIDFCONFIG]) -> io::Result<()>{
        let fut = async {
            self.write(&TCAN455xRequest::set_sid(sidf))?;
            self.write(&TCAN455xRequest::set_xid(xidf))?;
            Ok(())
        };
        block_on(fut.or(Self::timeout()))
    }

    pub fn switch_operation_mode(&mut self, mode: u8) -> io::Result<()> {
        let fut = async {
            let config: u32 = match self.read_device(REG_DEV_MODES_AND_PINS) {
                Ok(val) => val,
                Err(_) => return Err(io::ErrorKind::InvalidData.into())
            };

            let config_masked: u32 = config & !REG_BITS_DEVICE_MODE_DEVICEMODE_MASK;
            let payload: u32 = match mode {
                0 => config_masked | REG_BITS_DEVICE_MODE_DEVICEMODE_NORMAL,
                1 => config_masked | REG_BITS_DEVICE_MODE_DEVICEMODE_STANDBY,
                2 => config_masked | REG_BITS_DEVICE_MODE_DEVICEMODE_SLEEP,
                _ => config_masked
            };
            let cmd: Vec<u8> = TCAN455xRequest::get_write_command(REG_DEV_MODES_AND_PINS, vec![payload]);
            self.write(&cmd)?;
            Ok(())
        };
        block_on(fut.or(Self::timeout()))
    }

    pub fn switch_normal_mode(&mut self) -> io::Result<()> {
        Self::switch_operation_mode(self, 0)
    }

    pub fn switch_standby_mode(&mut self) -> io::Result<()> {
        Self::switch_operation_mode(self, 1)
    }

    pub fn switch_sleep_mode(&mut self) -> io::Result<()> {
        Self::switch_operation_mode(self, 2)
    }

    pub fn switch_test_mode(&mut self) -> io::Result<()> {
        // This register can be written only when REG_MCAN_CCCR[21] = 1;
        let fut = async {
            let cccr: u32 = match self.read_device(REG_MCAN_CCCR) {
                Ok(val) => val,
                Err(_) => return Err(io::ErrorKind::InvalidData.into())
            };

            let is_test_ena: bool = ((cccr & REG_BITS_MCAN_CCCR_TEST) > 0) & ((cccr & REG_BITS_MCAN_CCCR_MON) > 0);
            if !is_test_ena { return Ok(()); }

            let test: u32 = match self.read_device(REG_MCAN_TEST) {
                Ok(val) => val,
                Err(_) => return Err(io::ErrorKind::InvalidData.into())
            };
            let payload: u32 = test | REG_BITS_MCAN_TEST_LOOP_BACK;
            let cmd: Vec<u8> = TCAN455xRequest::get_write_command(REG_MCAN_TEST, vec![payload]);
            self.write(&cmd)?;
            Ok(())
        };
        block_on(fut.or(Self::timeout()))
    }


    pub const CAN_DLC_TO_DLEN: [u8;16] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 12, 16, 20, 24, 32, 48, 64];
    pub const CAN_DLEN_TO_DLC: [u8; 65] = [
        0,  1,  2,  3,  4,  5,  6,  7,  8,                               // 0-8
        9,  9,  9,  9,                                                   // 9-12
        10, 10, 10, 10,                                                  // 13-16
        11, 11, 11, 11,                                                  // 17-20
        12, 12, 12, 12,                                                  // 21-24
        13, 13, 13, 13, 13, 13, 13, 13,                                  // 25-32
        14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14,  // 33-48
        15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15,  // 49-64
    ];

    pub fn transmit(&mut self, xid: u32, data: &[u8], size: usize) -> io::Result<()> {
        let fut = async {
            
            let payloads: Vec<Vec<u8>> = data
                .chunks(TXFIFODATASIZE.size as usize)
                .map(|x| Vec::from(x))
                .collect();

            for payload in payloads {

                let byte_array: Vec<u32> = payload
                    .chunks(4)
                    .map(|x| u32::from_le_bytes([x[0], x[1], x[2], x[3]]))
                    .collect();

                let tx_fqs: u32 = match self.read_device(REG_MCAN_TXFQS) {
                    Ok(val) => val,
                    Err(_) => return Err(io::ErrorKind::InvalidData.into())
                };
                let tx_free_level: u32 = tx_fqs & & 0x000000FF;
                let tx_put_index: u16 = ((tx_fqs & 0x001F0000) >> 16) as u16;
            
                if tx_free_level == 0 { return Err(io::ErrorKind::Interrupted.into()) }
                
                let dlc: u32 = Self::CAN_DLEN_TO_DLC[size] as u32;

                const MM: u32 = 1;
                const EFC: u32 = 1;
                const FDF: u32 = 1;
                const BRS: u32 = 1;
                
                let addr: u16 = TCAN455xRequest::get_txdata_start_addr(tx_put_index);
                let xid: u32 = (1u32 << 30) |  xid;
                let header: u32 = (MM << 24) |(EFC << 23) | (FDF << 21) | (BRS << 20) | (dlc << 16);

                let mut payload: Vec<u32> = Vec::<u32>::new();
                payload.push(xid);
                payload.push(header);
                payload.extend(&byte_array);

                let cmd: Vec<u8> = TCAN455xRequest::get_write_command(addr, payload); 
                self.write(&cmd)?;

                let add_req: u32 = 1 << tx_put_index;
                let cmd: Vec<u8> = TCAN455xRequest::get_write_command(REG_MCAN_TXBAR, vec![add_req]); 
                self.write(&cmd)?;
            }
            Ok(())
        };
    
        block_on(fut.or(Self::timeout()))
    }

    pub fn receive(&mut self) -> io::Result<Option<RxData>> {
        let fut = async {

            let dev_ir: u32 = match Self::read_device_irq(self) {
                Ok(x) => x,
                Err(err) => return Err(err),
            };
            
            let mcan_int: bool =  (dev_ir & REG_BITS_DEVICE_IR_M_CAN_INT) >> 1 != 0;
            if !mcan_int {
                return Ok(None);
            }

            let mcan_ir: u32 = self.read_device(REG_MCAN_IR)?;

            let rx_fifo0_new_message: bool = (mcan_ir & REG_BITS_MCAN_IR_RF0N) != 0;
            let rx_fifo1_new_message: bool = (mcan_ir & REG_BITS_MCAN_IR_RF1N) != 0;

            let rx_fifo_new_message: [bool; 2] = [rx_fifo0_new_message, rx_fifo1_new_message];
            const RXFIFO_STATUS_ADDR: [u16; 2] = [REG_MCAN_RXF0S, REG_MCAN_RXF1S];
            const RXFIFO_ACK_ADDR: [u16; 2] = [REG_MCAN_RXF0A, REG_MCAN_RXF1A];

            let mut rx_buffer: RxData = RxData::new();

            if rx_fifo0_new_message | rx_fifo1_new_message {

                let cmd: Vec<u8> = TCAN455xRequest::get_write_command(REG_MCAN_IR, vec![mcan_ir]); 
                self.write(&cmd)?;

                for ch in 0..2 {
                    if rx_fifo_new_message[ch] {
                        let rx_fifo_status: u32 = self.read_device(RXFIFO_STATUS_ADDR[ch])?;

                        let rx_fifo_put_index: u32 = (rx_fifo_status >> 16) & 0x3f;
                        let rx_fifo_get_index: u32 = (rx_fifo_status >> 8) & 0x3f;
                        let rx_fifo_unread: u32 = rx_fifo_status & 0x7f;
            
                        let mut rx_data: Vec<u8> = Vec::<u8>::new();
                        
                        let rx_fifo_overwrapped: bool = rx_fifo_put_index <= rx_fifo_get_index;
                        if !rx_fifo_overwrapped {
                            let addr: u16 = TCAN455xRequest::get_rxdata_start_addr(ch as u16, rx_fifo_get_index as u16);
                            let len: u32 =  (RXDATA_BLOCKSIZE[ch] * rx_fifo_unread) / 4;
                            match self.read_bytes(addr, len as u8) {
                                Ok(vec) => rx_data.extend(vec),
                                Err(_) => {}
                            };
                        } else {
                            let addr: u16 = TCAN455xRequest::get_rxdata_start_addr(ch as u16, rx_fifo_get_index as u16);
                            let len: u32 =  (RXDATA_BLOCKSIZE[ch] * (rx_fifo_unread - rx_fifo_put_index)) / 4;
                            match self.read_bytes(addr, len as u8) {
                                Ok(vec) => rx_data.extend(vec),
                                Err(_) => {}
                            };
                            let addr: u16 = TCAN455xRequest::get_rxdata_start_addr(ch as u16, 0);
                            let len: u32 =  (RXDATA_BLOCKSIZE[ch] * (rx_fifo_put_index)) / 4;
                            match self.read_bytes(addr, len as u8) {
                                Ok(vec) => rx_data.extend(vec),
                                Err(_) => {}
                            };
                        }

                        match ch {
                            0 => rx_buffer.fifo0 = rx_data,
                            1 => rx_buffer.fifo1 = rx_data,
                            _ => {}
                        }

                        let rx_fifo_ack_index: u32 = (rx_fifo_put_index + RXDATA_FIFOSIZE[ch] - 1) % RXDATA_FIFOSIZE[ch];
                        let cmd: Vec<u8> = TCAN455xRequest::get_write_command(RXFIFO_ACK_ADDR[ch], vec![rx_fifo_ack_index]); 
                        self.write(&cmd)?;
                    }
                }

            }

            let _spi_staus = self.read_spi_status();
            let _dev_ir = self.read_device_irq();

            Ok(Some(rx_buffer))
        };

        block_on(fut.or(Self::timeout()))
    }

}