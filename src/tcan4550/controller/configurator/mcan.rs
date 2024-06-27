use crate::tcan4550::register::*;

// CC control register
const NISO: u32 = 0;   // Non ISO Operation, 0: CAN FD Frame format according to ISO 11898-1:2015, 1: CAN FD Frame format according to Bosch CAN FD Specification V1.0
const TXP: u32 = 0;    // Transmitter Pause is 0: Disabled, 1: Enabled
const EFBI: u32 = 0;   // Edge Filtering during Bus Integration 0: Edge Filtering Disabled, 1: Two Consecutive Dominant tq required to detect an edge for hard synchronization
const PXHD: u32 = 0;   // Protocol Exception Handling is 0: Enabled, 1: Disabled
const BRSE: u32 = 1;   // Bit Rate Switch is 0: Disable, 1: Enabled
const FDOE: u32 = 1;   // FD Operation Enable is 0: Disabled, 1: Enabled
const TEST: u32 = 0;   // **Test Mode Enable, 0: Normal Mode of Operation, Register TEST Holds Reset Value, 1: Test Mode, Write Access to Register TEST Enabled
const DAR: u32 = 0;    // Automatic Retransmission is 0: Enabled, 1: Disabled
const MON: u32 = 0;    // **Bus Monitoring Mode is 0: Disabled, 1: Enabled
const CSR: u32 = 0;    // Clock Stop Request, 0: No clock Stop is requested, 1: Clock Stop Requested.
const CSA: u32 = 0;    // Clock Stop Acknowledge, 0: No Clock Stop Requested, 1: m_can may be set in power down by stopping m_can-hclk and m_can_cclk
const ASM: u32 = 0;    // Restricted Operation Mode, 0: Normal CAN Operation, 1: Restricted Operation Mode Active
const CCE: u32 = 0;    // Configure change enable
const INIT: u32 = 0;   // Initialization, 0: Normal operation, 1: Initilization started

// Nominal timing: BitRate = FCLK / NBPRS / (NTSEG1 + NTSEG2 + 1)
const NBPRS: u32 = 2;
const NTSEG1: u32 = 31;
const NTSEG2: u32 = 8;

// Data timing: BitRate = FCLK / DBRPRS / (DTSEG1 + DTSEG2 + 1)
const DBRPRS: u32 = 2;
const DTSEG1: u32 = 5;
const DTSEG2: u32 = 4;
 
const NBTP_NBPRS: u32 = (NBPRS - 1) << 16;
const NBTP_NTSEG1: u32 = (NTSEG1 - 1) << 8;
const NBTP_NTSEG2: u32 = (NTSEG2 - 1) << 0;
const NBTP_NSJW: u32 = (NTSEG1 - 1) << 25;

const DBTP_TDC: u32 = REG_BITS_MCAN_DBTP_TDC_EN;
const DBTP_DBRPRS: u32 = (DBRPRS - 1) << 16;
const DBTP_DTSEG1: u32 = (DTSEG1 - 1) << 8;
const DBTP_DTSEG2: u32 = (DTSEG2 - 1) << 4;
const DBTP_DSJW: u32 = (DTSEG2 - 1) << 0;

const TDCO: u32 = (DTSEG1 - 2) << 8;

// Interrupt
const MCANIRQ_ARAE: u32 = 0;  //IE[29] ARAE: Access to reserved address
const MCANIRQ_PEDE: u32 = 0;  //IE[28] PEDE: Protocol error in data phase (data bit time is used)
const MCANIRQ_PEAE: u32 = 0;  //IE[27] PEAE Protocol Error in arbitration phase (nominal bit time used)
const MCANIRQ_WDIE: u32 = 0;  //IE[26] WDIE: MRAM Watchdog Interrupt
const MCANIRQ_BOE: u32 = 0;   //IE[25] BOE: Bus_off status changed
const MCANIRQ_EWE: u32 = 0;   //IE[24] EWE: Error_warning status changed
const MCANIRQ_EPE: u32 = 0;   //IE[23] EPE: Error_passive status changed
const MCANIRQ_ELOE: u32 = 0;  //IE[22] ELOE: Error logging overflow
const MCANIRQ_BEUE: u32 = 0;   //IE[21] BEUE: MRAM Bit error uncorrected
const MCANIRQ_BECE: u32 = 0;  //IE[20] BECE: MRAM Bit error corrected
const MCANIRQ_DRXE: u32 = 0;  //IE[19] DRXE: Message stored to dedicated RX buffer
const MCANIRQ_TOOE: u32 = 0;  //IE[18] TOOE: Time out occured
const MCANIRQ_MRAFE: u32 = 0; //IE[17] MRAFE: Message RAM access failure
const MCANIRQ_TSWE: u32 = 0;  //IE[16] TSWE: Timestamp wraparound
const MCANIRQ_TEFLE: u32 = 0; //IE[15] TEFLE: Tx Event FIFO element lost
const MCANIRQ_TEFFE: u32 = 0; //IE[14] TEFFE: Tx Event FIFO full
const MCANIRQ_TEFWE: u32 = 0; //IE[13] TEFWE: Tx Event FIFO watermark reached
const MCANIRQ_TEFNE: u32 = 0; //IE[12] TEFNE: Tx Event FIFO generate_write_command entry
const MCANIRQ_TFEE: u32 = 0;  //IE[11] TFEE: Tx FIFO Empty
const MCANIRQ_TCFE: u32 = 0;  //IE[10] TCFE: Transmission cancellation finished
const MCANIRQ_TCE: u32 = 0;   //IE[9] TCE: Transmission completed
const MCANIRQ_HPME: u32 = 0;  //IE[8] HPME: High priority message
const MCANIRQ_RF1LE: u32 = 0; //IE[7] RF1LE: Rx FIFO 1 message lost
const MCANIRQ_RF1FE: u32 = 0; //IE[6] RF1FE: Rx FIFO 1 full
const MCANIRQ_RF1WE: u32 = 0; //IE[5]  RF1WE: RX FIFO 1 watermark reached
const MCANIRQ_RF1NE: u32 = 1; //IE[4] RF1NE: Rx FIFO 1 generate_write_command message
const MCANIRQ_RF0LE: u32 = 0; //IE[3] RF0LE: Rx FIFO 0 message lost
const MCANIRQ_RF0FE: u32 = 0; //IE[2] RF0FE: Rx FIFO 0 full
const MCANIRQ_RF0WE: u32 = 0; //IE[1] RF0WE: Rx FIFO 0 watermark reached
const MCANIRQ_RF0NE: u32 = 1; //IE[0] RF0NE: Rx FIFO 0 generate_write_command message

const MCANIRQ_INT1_EN: u32 = 1;
const MCANIRQ_INT0_EN: u32 = 1;
  
impl super::super::TCAN455xController {
    pub fn protect_register(data: u32) -> u32 {
        data & !(REG_BITS_MCAN_CCCR_CSA | REG_BITS_MCAN_CCCR_CSR | REG_BITS_MCAN_CCCR_INIT | REG_BITS_MCAN_CCCR_CCE)
    }

    pub fn unprotect_register(data: u32) -> u32 {
        data & !(REG_BITS_MCAN_CCCR_CSA | REG_BITS_MCAN_CCCR_CSR) | (REG_BITS_MCAN_CCCR_INIT | REG_BITS_MCAN_CCCR_CCE)
    }

    pub fn set_mcan_cccr() -> Vec<u8> {
        let addr: u16 = REG_MCAN_CCCR;
        let data: u32 = (NISO << 15)
                | (TXP << 14)
                | (EFBI << 13)
                | (PXHD << 12)
                | (BRSE << 9)
                | (FDOE << 8)
                | (TEST << 7)
                | (DAR << 6)
                | (MON << 5)
                | (CSR << 4)
                | (CSA << 3)
                | (ASM << 2)
                | (CCE << 1)
                | (INIT << 0);
        let data: u32 = Self::unprotect_register(data);
        Self::generate_write_command(addr, vec![data])
    }

    pub fn set_dbtp() -> Vec<u8> {
        let addr: u16 = REG_MCAN_DBTP;
        let data: u32 = DBTP_TDC | DBTP_DSJW | DBTP_DBRPRS | DBTP_DTSEG1 |  DBTP_DTSEG2;
        Self::generate_write_command(addr, vec![data])
    }

    pub fn set_nbtp() -> Vec<u8> {
        let addr: u16 = REG_MCAN_NBTP;
        let data: u32 = NBTP_NSJW | NBTP_NBPRS | NBTP_NTSEG1 |  NBTP_NTSEG2;
        Self::generate_write_command(addr, vec![data])
    }

    pub fn set_tdcr() -> Vec<u8> {
        let addr: u16 = REG_MCAN_TDCR;
        let data: u32 = TDCO;
        Self::generate_write_command(addr, vec![data])
    }
    
    pub fn set_tscc() -> Vec<u8> {
        let addr: u16 = REG_MCAN_TSCC;
        let data: u32 = REG_BITS_MCAN_TSCC_COUNTER_EXTERNAL;
        Self::generate_write_command(addr, vec![data])
    }

    pub fn set_mcan_ie() -> Vec<u8> {

        let addr: u16 = REG_MCAN_IE;
        let data: u32 = (MCANIRQ_ARAE << 29)
            | (MCANIRQ_PEDE << 28)
            | (MCANIRQ_PEAE << 27)
            | (MCANIRQ_WDIE << 26)
            | (MCANIRQ_BOE << 25)
            | (MCANIRQ_EWE << 24)
            | (MCANIRQ_EPE << 23)
            | (MCANIRQ_ELOE << 22)
            | (MCANIRQ_BEUE << 21)
            | (MCANIRQ_BECE << 20)
            | (MCANIRQ_DRXE << 19)
            | (MCANIRQ_TOOE << 18)
            | (MCANIRQ_MRAFE << 17)
            | (MCANIRQ_TSWE << 16)
            | (MCANIRQ_TEFLE << 15)
            | (MCANIRQ_TEFFE << 14)
            | (MCANIRQ_TEFWE << 13)
            | (MCANIRQ_TEFNE << 12)
            | (MCANIRQ_TFEE << 11)
            | (MCANIRQ_TCFE << 10)
            | (MCANIRQ_TCE << 9)
            | (MCANIRQ_HPME << 8)
            | (MCANIRQ_RF1LE << 7)
            | (MCANIRQ_RF1FE << 6)
            | (MCANIRQ_RF1WE << 5)
            | (MCANIRQ_RF1NE << 4)
            | (MCANIRQ_RF0LE << 3)
            | (MCANIRQ_RF0FE << 2)
            | (MCANIRQ_RF0WE << 1)
            | (MCANIRQ_RF0NE << 0);

        Self::generate_write_command(addr, vec![data])
        
    }
    
    pub fn set_mcan_ile() -> Vec<u8> {
        let addr: u16 = REG_MCAN_ILE;
        let data: u32 = (MCANIRQ_INT1_EN << 1) | (MCANIRQ_INT0_EN << 0);
        Self::generate_write_command(addr, vec![data])
    }

}