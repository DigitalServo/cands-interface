
#[cfg(feature="usb-ftdi")]
use crate::driver::ftdi::{FtdiDriver, Ft232h, TimeoutError};

#[cfg(feature="raspberrypi")]
use crate::driver::raspberrypi::RaspiIF;

pub mod usb_ftdi;
pub mod raspberrypi;

pub struct TCAN455xTranceiver {
    #[cfg(feature="usb-ftdi")]
    driver: FtdiDriver<Ft232h, TimeoutError>,

    #[cfg(feature="raspberrypi")]
    driver: RaspiIF,
}