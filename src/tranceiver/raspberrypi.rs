#[cfg(feature="raspberrypi")]
use crate::driver::raspberrypi::{RaspiIF, GPIO_INPUT_PIN_NUM};

#[cfg(feature="raspberrypi")]
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
}