mod driver;
pub mod tcan4550;
mod tranceiver;
mod rx_buffer;

#[cfg(feature="usb-ftdi")]
pub use tranceiver::usb_ftdi::TCAN455xTranceiver;

#[cfg(feature="raspberrypi")]
pub use tranceiver::raspberrypi::TCAN455xTranceiver;
