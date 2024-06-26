mod driver;
pub mod tcan4550;
mod tranceiver;
mod rx_buffer;

/// CAN Tranceiver
pub use tranceiver::TCAN455xTranceiver;

/// Receive data buffer on user space
pub use rx_buffer::RxData;