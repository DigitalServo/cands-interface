#[cfg(feature="usb-ftdi")]
pub mod ftdi;

#[cfg(feature="raspberrypi")]
pub mod raspberrypi;

#[cfg(feature="raspberrypi_cm")]
pub mod raspberrypi_cm;

use std::io::Error as IoError;
type IoResult<T> = Result<T, IoError>;

pub const GPI_MAX_POINT: usize = 64;

#[allow(dead_code)]
pub(crate) trait TCAN455xDriver {
    fn tcan455x_write(&mut self, data: &[u8]) -> IoResult<usize>;
    fn tcan455x_read(&mut self, buffer: &mut [u8]) -> IoResult<usize>;
    fn tcan455x_transfer(&mut self, data: &[u8], buffer: &mut [u8]) -> IoResult<usize>;
    fn tcan455x_transfer_in_place(&mut self, data: &mut [u8]) -> IoResult<usize>;
    fn tcan455x_reset(&mut self) -> IoResult<()>;
}

#[allow(dead_code)]
pub(crate) trait ADCDriver {
    fn adc_reset(&mut self) -> IoResult<()>;
    fn adc_write(&mut self, data: &[u8]) -> IoResult<usize>;
    fn adc_read(&mut self, buffer: &mut [u8]) -> IoResult<usize>;
    fn adc_transfer(&mut self, data: &[u8], buffer: &mut [u8]) -> IoResult<usize>;
    fn adc_transfer_in_place(&mut self, data: &mut [u8]) -> IoResult<usize>;
}

#[allow(dead_code)]
pub(crate) trait WS2812Driver {
    fn ws2812_write(&mut self, data: &[u8]) -> IoResult<usize>;
}

#[allow(dead_code)]
pub(crate) trait GpioDriver {
    fn gpio_out(&mut self, state: u8) -> IoResult<()>;
    fn gpio_read(&mut self, channel: usize) -> IoResult<bool>;
    fn gpio_read_all(&mut self) -> IoResult<[bool; GPI_MAX_POINT]>;
}

#[allow(dead_code)]
pub(crate) trait DeviceDriver: TCAN455xDriver {}

#[allow(dead_code)]
pub(crate) trait RaspiDeviceDriver: TCAN455xDriver + GpioDriver + ADCDriver + WS2812Driver {}


