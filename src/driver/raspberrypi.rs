use rppal::gpio::{Gpio, InputPin, OutputPin};
use rppal::spi::{Bus, Mode, SlaveSelect, Spi};

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
    
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {

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

    pub fn reset_tcan455x(&mut self) {
        self.reset_pin.set_high();
        std::thread::sleep(std::time::Duration::from_millis(100));
        self.reset_pin.set_low();
        std::thread::sleep(std::time::Duration::from_millis(100));
    }

    pub fn gpi_read(&mut self, channel: usize) -> bool {
        self.input_pins[channel].is_high()
    }

    pub fn gpi_read_all(&mut self) -> [bool; GPIO_INPUT_PIN_NUM] {
        let mut ret: [bool; GPIO_INPUT_PIN_NUM] = [false; GPIO_INPUT_PIN_NUM];
        for i in 0..GPIO_INPUT_PIN_NUM {
            ret[i] = self.input_pins[i].is_high();
        }
        ret
    }

    pub fn spi_read(&mut self, buffer: &mut [u8]) -> rppal::spi::Result<usize> {
        self.spi.read(buffer)
    } 

    pub fn spi_write(&mut self, buffer: &[u8]) -> rppal::spi::Result<usize> {
        self.spi.write(buffer)
    } 

    pub fn spi_transfer(&mut self, rx_buffer: &mut [u8], tx_buffer: &[u8]) -> rppal::spi::Result<usize> {
        self.spi.transfer(rx_buffer, tx_buffer)
    }

}