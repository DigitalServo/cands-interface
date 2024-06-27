#[cfg(feature="raspberrypi")]
use crate::driver::raspberrypi::{RaspiIF, GPIO_INPUT_PIN_NUM};

#[cfg(feature="raspberrypi")]
impl super::TCAN455xTranceiver {
    pub fn new () -> Result<Self, Box<dyn std::error::Error>> {
        let driver = RaspiIF::new()?;
        Ok(Self { driver })
    }

    pub fn gpi_read(&mut self, channel: usize) -> bool {
        self.driver.gpi_read(channel)
    }

    pub fn gpi_read_all(&mut self) -> [bool; GPIO_INPUT_PIN_NUM] {
        self.driver.gpi_read_all()
    }
}