#[cfg(feature="usb-ftdi")]
use crate::driver::ftdi::{FtdiDriver, Ft232h};

#[cfg(feature="usb-ftdi")]
impl super::TCAN455xTranceiver {
    pub fn new () -> Result<Self, Box<dyn std::error::Error>> {
        const SPI_CLK_FREQ: u32 = 15_000_000;
        const SPI_CLK_POLARITY: u8 = 0;
        let driver: FtdiDriver<Ft232h, _> = FtdiDriver::new(SPI_CLK_FREQ, SPI_CLK_POLARITY)?;
        Ok(Self { driver: Box::new(driver) })
    }
}