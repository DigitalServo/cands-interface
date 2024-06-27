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

use super::{GpioDriver, SpiDriver, TCAN4550Driver, GPI_MAX_POINT};

const GPIO_RESET_PIN_BCM: u8 = 5;

pub const GPIO_INPUT_PIN_NUM: usize = 8;
const GPIO_INPUT_PIN_BCM: [u8; GPIO_INPUT_PIN_NUM] = [ 3, 2, 4, 14, 18, 15, 23, 17 ];

const SPI_BUS: Bus = Bus::Spi0;
const SPI_SS: SlaveSelect = SlaveSelect::Ss0;
const SPI_CLK_FREQ: u32 = 18_000_000;
const SPI_MODE: Mode = Mode::Mode0;

pub struct RaspiIF {
    pub spi: Spi,
    pub reset_pin: OutputPin,
    pub input_pins: [InputPin; GPIO_INPUT_PIN_NUM]
}

impl  RaspiIF {
    
    pub fn new() -> Result<Self, Box<dyn StdError>> {

        let gpio: Gpio = match Gpio::new() {
            Ok(x) => x,
            Err(e) => return Err(Box::new(e)),
        };

        //TCAN4550 spi
        let spi: Spi = match Spi::new(SPI_BUS, SPI_SS, SPI_CLK_FREQ, SPI_MODE) {
            Ok(x) => x,
            Err(e) => return Err(Box::new(e)),
        };

        //TCAN4550 reset pin
        let reset_pin: OutputPin = match gpio.get(GPIO_RESET_PIN_BCM) {
            Ok(x) => x.into_output(),
            Err(e) => return Err(Box::new(e)),
        };

        //Input pins
        let input_pin_0: InputPin = match gpio.get(GPIO_INPUT_PIN_BCM[0]) {
            Ok(x) => x.into_input(),
            Err(e) => return Err(Box::new(e)),
        };

        let input_pin_1: InputPin = match gpio.get(GPIO_INPUT_PIN_BCM[1]) {
            Ok(x) => x.into_input(),
            Err(e) => return Err(Box::new(e)),
        };

        let input_pin_2: InputPin = match gpio.get(GPIO_INPUT_PIN_BCM[2]) {
            Ok(x) => x.into_input(),
            Err(e) => return Err(Box::new(e)),
        };

        let input_pin_3: InputPin = match gpio.get(GPIO_INPUT_PIN_BCM[3]) {
            Ok(x) => x.into_input(),
            Err(e) => return Err(Box::new(e)),
        };

        let input_pin_4: InputPin = match gpio.get(GPIO_INPUT_PIN_BCM[4]) {
            Ok(x) => x.into_input(),
            Err(e) => return Err(Box::new(e)),
        };

        let input_pin_5: InputPin = match gpio.get(GPIO_INPUT_PIN_BCM[5]) {
            Ok(x) => x.into_input(),
            Err(e) => return Err(Box::new(e)),
        };

        let input_pin_6: InputPin = match gpio.get(GPIO_INPUT_PIN_BCM[6]) {
            Ok(x) => x.into_input(),
            Err(e) => return Err(Box::new(e)),
        };

        let input_pin_7: InputPin = match gpio.get(GPIO_INPUT_PIN_BCM[7]) {
            Ok(x) => x.into_input(),
            Err(e) => return Err(Box::new(e)),
        };

        let input_pins: [InputPin; GPIO_INPUT_PIN_NUM] = [
            input_pin_0,
            input_pin_1,
            input_pin_2,
            input_pin_3,
            input_pin_4,
            input_pin_5,
            input_pin_6,
            input_pin_7,
        ];

        Ok(Self { spi, reset_pin, input_pins })
    }
}

impl SpiDriver for RaspiIF {
    fn spi_read(&mut self, buffer: &mut [u8]) -> IoResult<usize> {
        self.spi.read(buffer).map_err(emap())
    }

    fn spi_write(&mut self, buffer: &[u8]) -> IoResult<usize> {
        self.spi.write(buffer).map_err(emap())
    } 

    fn spi_transfer(&mut self, tx_buffer: &[u8], rx_buffer: &mut [u8]) -> IoResult<usize> {
        self.spi.transfer(rx_buffer, tx_buffer).map_err(emap())
    }

    fn spi_transfer_in_place(&mut self, data: &mut [u8]) -> IoResult<usize> {
        let mut rx_buffer: [u8; 512] = [0u8; 512];
        let size: usize = self.spi.transfer(&mut rx_buffer, data).map_err(emap())?;
        for i in 0..size {
            data[i] = rx_buffer[i];
        }
        Ok(size)
    }
}

impl GpioDriver for RaspiIF {
    fn gpio_out(&mut self, _state: u8) -> IoResult<()> {
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

impl TCAN4550Driver for RaspiIF {
    fn reset_tcan4550(&mut self) -> super::IoResult<()> {

        const RESET_WAIT_TIME: u64 = 5;
        
        self.reset_pin.set_high();
        std::thread::sleep(std::time::Duration::from_millis(RESET_WAIT_TIME));
        self.reset_pin.set_low();
        std::thread::sleep(std::time::Duration::from_millis(RESET_WAIT_TIME));
        Ok(())
    }
}