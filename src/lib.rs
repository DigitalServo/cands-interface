mod device_driver;
mod tcan4550;
mod tranceiver;

pub use tcan4550::id_filter::{SIDConfig, XIDConfig};
pub use tcan4550::register as tcan4550_register;

pub use tranceiver::TCAN455xTranceiver;

#[cfg(feature="raspberrypi")]
pub use device_driver::raspberrypi::GPIO_INPUT_PIN_NUM;

#[cfg(feature="raspberrypi")]
pub use device_driver::raspberrypi::GPIO_OUTPUT_PIN_NUM;

pub use tranceiver::rx_buffer::RxData;