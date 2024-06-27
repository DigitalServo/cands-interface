#[cfg(feature="usb-ftdi")]
pub mod ftdi;

#[cfg(feature="raspberrypi")]
pub mod raspberrypi;

use std::io::Error as IoError;
type IoResult<T> = Result<T, IoError>;

pub const GPI_MAX_POINT: usize = 64;

#[allow(dead_code)]
pub(crate) trait SpiDriver {
    fn spi_write(&mut self, data: &[u8]) -> IoResult<usize>;
    fn spi_read(&mut self, buffer: &mut [u8]) -> IoResult<usize>;
    fn spi_transfer(&mut self, data: &[u8], buffer: &mut [u8]) -> IoResult<usize>;
    fn spi_transfer_in_place(&mut self, data: &mut [u8]) -> IoResult<usize>;
}

#[allow(dead_code)]
pub(crate) trait GpioDriver {
    fn gpio_out(&mut self, state: u8) -> IoResult<()>;
    fn gpio_read(&mut self, channel: usize) -> IoResult<bool>;
    fn gpio_read_all(&mut self) -> IoResult<[bool; GPI_MAX_POINT]>;
}

#[allow(dead_code)]
pub(crate) trait TCAN4550Driver: SpiDriver + GpioDriver {
    fn reset_tcan4550(&mut self) -> IoResult<()>;
}