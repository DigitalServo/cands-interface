use crate::device_driver::raspberrypi_cm::{RaspiIF, GPIO_INPUT_PIN_NUM};

impl super::TCAN455xTranceiver {
    pub fn new () -> Result<Self, Box<dyn std::error::Error>> {
        let driver: RaspiIF = RaspiIF::new()?;
        Ok(Self { driver: Box::new(driver) })
    }

    pub fn gpi_read(&mut self, channel: usize) -> bool {
        self.driver.gpio_read(channel).unwrap()
    }

    pub fn gpi_read_all(&mut self) -> [bool; GPIO_INPUT_PIN_NUM] {
        let mut ret: [bool; GPIO_INPUT_PIN_NUM] = [false; GPIO_INPUT_PIN_NUM];
        let gpi = self.driver.gpio_read_all().unwrap();
        for i in 0..GPIO_INPUT_PIN_NUM {
            ret[i] = gpi[i];
        }
        ret
    }

    pub fn ws2812_write(&mut self, buffer: &[u8]) -> Result<(), Box<dyn std::error::Error>> {
        self.driver.ws2812_write(buffer)?;
        Ok(())
    }

    pub fn adc_reset(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.driver.adc_reset()?;
        Ok(())
    }

    pub fn adc_read(&mut self) -> Result<[u8; 3], Box<dyn std::error::Error>> {
        let mut buf: [u8; 3] = [0u8; 3];
        self.driver.adc_read(&mut buf)?;
        Ok(buf)
    }
}