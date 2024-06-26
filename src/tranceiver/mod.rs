
#[cfg(feature="usb-ftdi")]
use crate::driver::ftdi::{FtdiDriver, Ft232h, TimeoutError};

#[cfg(feature="raspberrypi")]
use crate::driver::raspberrypi::{RaspiIF, GPIO_INPUT_PIN_NUM};

pub mod usb_ftdi;
pub mod raspberrypi;


pub struct TCAN455xTranceiver {
    #[cfg(feature="usb-ftdi")]
    driver: FtdiDriver<Ft232h, TimeoutError>,

    #[cfg(feature="raspberrypi")]
    driver: RaspiIF,
    #[cfg(feature="raspberrypi")]
    rx_buf: [u8; BUFSIZE],
}