use rppal::gpio::{Gpio, InputPin, OutputPin};
use rppal::spi::{Bus, Mode, SlaveSelect, Spi};

//Error handling
use std::error::Error as StdError;
use std::io::{Error as IoError, ErrorKind as IoErrorKind};
use rppal::spi::Error as RaspiError;

type IoResult<T> = Result<T, IoError>;

fn emap() -> impl FnOnce(RaspiError) -> IoError { |err| match err {
    RaspiError::Io(e) => e,
    RaspiError::BitsPerWordNotSupported(_u8) => IoError::new(IoErrorKind::Other, "Bits Per Word Not Supported"),
    RaspiError::BitOrderNotSupported(_bit_order) => IoError::new(IoErrorKind::Other, "Bit Order Not Supported"),
    RaspiError::ClockSpeedNotSupported(_u32) => IoError::new(IoErrorKind::Other, "Clock Speed Not Supported"),
    RaspiError::ModeNotSupported(_mode) => IoError::new(IoErrorKind::Other, "Mode Not Supported"),
    RaspiError::PolarityNotSupported(_polarity) => IoError::new(IoErrorKind::Other, "Polarity Not Supported"),
} }

use super::{GpioDriver, ADCDriver, TCAN455xDriver, WS2812Driver, RaspiDeviceDriver, GPI_MAX_POINT};

const GPIO_RESET_PIN_BCM: u8 = 5;
const ADC_RESET_PIN_BCM: u8 = 26;


pub const GPIO_INPUT_PIN_NUM: usize = 10;
pub const GPIO_OUTPUT_PIN_NUM: usize = 2;

const GPIO_INPUT_PIN_BCM: [u8; GPIO_INPUT_PIN_NUM] = [15, 24, 2, 3, 4, 17, 27, 22, 25, 7];
const GPIO_OUTPUT_PIN_BCM: [u8; GPIO_OUTPUT_PIN_NUM] = [23, 18];

/// Mode = 0 -> CPOL: 0, CPHA: 0
/// Mode = 1 -> CPOL: 0, CPHA: 1
/// Mode = 2 -> CPOL: 1, CPHA: 0
/// Mode = 3 -> CPOL: 1, CPHA: 1

/// SPI0
/// use BCM 8, 9, 10, 11
const SPI0_BUS: Bus = Bus::Spi0;
const SPI0_SS: SlaveSelect = SlaveSelect::Ss0;
const SPI0_CLK_FREQ: u32 = 18_000_000;
const SPI0_MODE: Mode = Mode::Mode0;

/// SPI1
/// use BCM 18, 19, 20, 21
const SPI1_BUS: Bus = Bus::Spi1;
const SPI1_SS: SlaveSelect = SlaveSelect::Ss0;
const SPI1_CLK_FREQ: u32 = 8_000_000;
const SPI1_MODE: Mode = Mode::Mode0;

/// SPI5
/// use BCM 12, 13, 14, 16
const SPI5_BUS: Bus = Bus::Spi5;
const SPI5_SS: SlaveSelect = SlaveSelect::Ss0;
const SPI5_CLK_FREQ: u32 = 5_000_000;
const SPI5_MODE: Mode = Mode::Mode0;

pub struct RaspiIF {
    pub spi0: Spi,
    pub spi1: Spi,
    pub spi5: Spi,
    pub tcan_reset_pin: OutputPin,
    pub adc_reset_pin: OutputPin,
    pub input_pins: [InputPin; GPIO_INPUT_PIN_NUM],
    pub output_pins: [OutputPin; GPIO_OUTPUT_PIN_NUM]
}

impl  RaspiIF {
    
    pub fn new() -> Result<Self, Box<dyn StdError>> {

        let gpio: Gpio = match Gpio::new() {
            Ok(x) => x,
            Err(e) => return Err(Box::new(e)),
        };

        //TCAN455x spi
        let spi0: Spi = Spi::new(SPI0_BUS, SPI0_SS,  SPI0_CLK_FREQ, SPI0_MODE)?;

        //ADC spi
        let spi1: Spi = Spi::new(SPI1_BUS, SPI1_SS, SPI1_CLK_FREQ, SPI1_MODE)?;

        //WS2812 spi
        let spi5: Spi = Spi::new(SPI5_BUS, SPI5_SS, SPI5_CLK_FREQ, SPI5_MODE)?;


        //TCAN455x reset pin
        let tcan_reset_pin: OutputPin = gpio.get(GPIO_RESET_PIN_BCM).map(|x| x.into_output()).map_err(|e| Box::new(e))?;

        //ADC reset pin
        let adc_reset_pin: OutputPin = gpio.get(ADC_RESET_PIN_BCM).map(|x| x.into_output()).map_err(|e| Box::new(e))?;

        //Input pins
        let input_pins: [InputPin; GPIO_INPUT_PIN_NUM] = [
            gpio.get(GPIO_INPUT_PIN_BCM[0]).map(|x| x.into_input()).map_err(|e| Box::new(e))?,
            gpio.get(GPIO_INPUT_PIN_BCM[1]).map(|x| x.into_input()).map_err(|e| Box::new(e))?,
            gpio.get(GPIO_INPUT_PIN_BCM[2]).map(|x| x.into_input()).map_err(|e| Box::new(e))?,
            gpio.get(GPIO_INPUT_PIN_BCM[3]).map(|x| x.into_input()).map_err(|e| Box::new(e))?,
            gpio.get(GPIO_INPUT_PIN_BCM[4]).map(|x| x.into_input()).map_err(|e| Box::new(e))?,
            gpio.get(GPIO_INPUT_PIN_BCM[5]).map(|x| x.into_input()).map_err(|e| Box::new(e))?,
            gpio.get(GPIO_INPUT_PIN_BCM[6]).map(|x| x.into_input()).map_err(|e| Box::new(e))?,
            gpio.get(GPIO_INPUT_PIN_BCM[7]).map(|x| x.into_input()).map_err(|e| Box::new(e))?,
            gpio.get(GPIO_INPUT_PIN_BCM[8]).map(|x| x.into_input()).map_err(|e| Box::new(e))?,
            gpio.get(GPIO_INPUT_PIN_BCM[9]).map(|x| x.into_input()).map_err(|e| Box::new(e))?,
        ];

        //Output pins
        let output_pins: [OutputPin; GPIO_OUTPUT_PIN_NUM] = [
            gpio.get(GPIO_OUTPUT_PIN_BCM[0]).map(|x| x.into_output()).map_err(|e| Box::new(e))?,
            gpio.get(GPIO_OUTPUT_PIN_BCM[1]).map(|x| x.into_output()).map_err(|e| Box::new(e))?,
        ];

        Ok(Self { spi0, spi1, spi5, tcan_reset_pin, adc_reset_pin, input_pins, output_pins })
    }
}

impl GpioDriver for RaspiIF {
    fn gpio_out(&mut self, state: u8) -> IoResult<()> {
        for i in 0..GPIO_OUTPUT_PIN_NUM {
            if (state & (0x01 << i)) != 0 {
                self.output_pins[i].set_high();
            } else {
                self.output_pins[i].set_low();
            }
        }
        Ok(())
    }

    fn gpio_read(&mut self, channel: usize) -> IoResult<bool> {
        Ok(self.input_pins[channel].is_high())
    }

    fn gpio_read_all(&mut self) -> IoResult<[bool; GPI_MAX_POINT]> {
        let mut ret: [bool; GPI_MAX_POINT] = [false; GPI_MAX_POINT];
        for i in 0..GPIO_INPUT_PIN_NUM {
            ret[i] = self.input_pins[i].is_high();
        }
        Ok(ret)
    }
}

impl TCAN455xDriver for RaspiIF {

    fn tcan455x_read(&mut self, buffer: &mut [u8]) -> IoResult<usize> {
        self.spi0.read(buffer).map_err(emap())
    }

    fn tcan455x_write(&mut self, buffer: &[u8]) -> IoResult<usize> {
        self.spi0.write(buffer).map_err(emap())
    } 

    fn tcan455x_transfer(&mut self, tx_buffer: &[u8], rx_buffer: &mut [u8]) -> IoResult<usize> {
        self.spi0.transfer(rx_buffer, tx_buffer).map_err(emap())
    }

    fn tcan455x_transfer_in_place(&mut self, data: &mut [u8]) -> IoResult<usize> {
        let mut rx_buffer: [u8; 512] = [0u8; 512];
        let size: usize = self.spi0.transfer(&mut rx_buffer, data).map_err(emap())?;
        for i in 0..size {
            data[i] = rx_buffer[i];
        }
        Ok(size)
    }

    fn tcan455x_reset(&mut self) -> super::IoResult<()> {

        const RESET_WAIT_TIME: u64 = 5;
        
        self.tcan_reset_pin.set_high();
        std::thread::sleep(std::time::Duration::from_millis(RESET_WAIT_TIME));
        self.tcan_reset_pin.set_low();
        std::thread::sleep(std::time::Duration::from_millis(RESET_WAIT_TIME));
        Ok(())
    }
}

impl ADCDriver for RaspiIF {

    fn adc_reset(&mut self) -> IoResult<()> {

        const RESET_WAIT_TIME: u64 = 100;
        
        self.adc_reset_pin.set_high();
        std::thread::sleep(std::time::Duration::from_millis(RESET_WAIT_TIME));
        self.adc_reset_pin.set_low();
        std::thread::sleep(std::time::Duration::from_millis(RESET_WAIT_TIME));
        Ok(())
    }

    fn adc_read(&mut self, buffer: &mut [u8]) -> IoResult<usize> {
        self.spi1.read(buffer).map_err(emap())
    }

    fn adc_write(&mut self, buffer: &[u8]) -> IoResult<usize> {
        self.spi1.write(buffer).map_err(emap())
    } 

    fn adc_transfer(&mut self, tx_buffer: &[u8], rx_buffer: &mut [u8]) -> IoResult<usize> {
        self.spi1.transfer(rx_buffer, tx_buffer).map_err(emap())
    }

    fn adc_transfer_in_place(&mut self, data: &mut [u8]) -> IoResult<usize> {
        let mut rx_buffer: [u8; 512] = [0u8; 512];
        let size: usize = self.spi1.transfer(&mut rx_buffer, data).map_err(emap())?;
        for i in 0..size {
            data[i] = rx_buffer[i];
        }
        Ok(size)
    }
}

impl WS2812Driver for RaspiIF {
    fn ws2812_write(&mut self, buffer: &[u8]) -> IoResult<usize> {
        self.spi5.write(buffer).map_err(emap())
    } 
}

impl RaspiDeviceDriver for RaspiIF {}