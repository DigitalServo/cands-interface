use ftdi_embedded_hal::{
    libftd2xx::Ftdi,
    ftdi_mpsse::MpsseCmdExecutor,
    FtHal,
    SpiDevice,
    OutputPin,
};

//Import trait: determine functions
use ftdi_embedded_hal::eh1::{
    digital::OutputPin as OutputPinTrait,
    spi::SpiDevice as SpiDeviceTrait,
    spi::Polarity
};

//Re-export device type
pub use ftdi_embedded_hal::libftd2xx::{
    Ft232h,
    TimeoutError,
    DeviceInfo,
    list_devices as list_ftdi_devices
};

//Error handling
use std::error::Error as StdError;
use std::io::{ Error as IoError, ErrorKind as IoErrorKind};
use ftdi_embedded_hal::Error as FtdiError;

pub fn list_devices() -> Result<Vec<DeviceInfo>, Box<dyn StdError>> {
    match list_ftdi_devices() {
        Ok(list) => Ok(list),
        Err(e) => Err(e.into())
    }
}

pub struct FtdiDriver <DEVICE, E>
where
    DEVICE: MpsseCmdExecutor<Error = E>,
    E: StdError,
{
    spi: SpiDevice<DEVICE>,
    pins: [OutputPin<DEVICE>; 4]
}

impl <DEVICE, E> FtdiDriver <DEVICE, E>
where
    DEVICE: MpsseCmdExecutor<Error = E> + TryFrom<Ftdi>,
    <DEVICE as TryFrom<Ftdi>>::Error: StdError + 'static,
    E: StdError,
    FtdiError<E>: From<E>,
{
    pub fn find_device() -> Result<DEVICE, Box<dyn StdError>> {
        let device: Ftdi = Ftdi::new()?;
        let device: DEVICE = device.try_into()?;
        Ok(device)
    }

    pub fn new(spi_clk_freq: u32, spi_clk_polarity: u8) -> Result<Self, FtdiError<E>> {

        let device: DEVICE = match Self::find_device() {
            Ok(device) => device,
            Err(_) => {
                let err: IoError = IoError::new(IoErrorKind::NotConnected, "Device Not Found.");
                return Err(FtdiError::Io(err));
            }
        };

        let hal: FtHal<DEVICE> = FtHal::init_freq(device, spi_clk_freq)?;

        const SPI_CS_INDEX: u8 = 3;
        let spi_clk_polarity: Polarity = if spi_clk_polarity == 0 { Polarity::IdleLow } else {Polarity::IdleHigh};
        let mut spi: SpiDevice<DEVICE> = hal.spi_device(SPI_CS_INDEX)?;
        spi.set_clock_polarity(spi_clk_polarity);
        
        let pin_1: OutputPin<DEVICE> = hal.ad4()?;
        let pin_2: OutputPin<DEVICE> = hal.ad5()?;
        let pin_3: OutputPin<DEVICE> = hal.ad6()?;
        let pin_4: OutputPin<DEVICE> = hal.ad7()?;
        
        let pins: [OutputPin<DEVICE>; 4] = [pin_1, pin_2, pin_3, pin_4];

        Ok(Self { spi, pins })
    }

    pub fn spi_write(&mut self, data: &[u8]) -> Result<(), FtdiError<E>>{
        (&self.spi).write(data)
    }

    pub fn spi_transfer(&mut self, data: &[u8], buffer: &mut [u8]) -> Result<(), FtdiError<E>>{
        (&self.spi).transfer(buffer, data)
    }

    pub fn spi_transfer_in_place(&mut self, data: &mut [u8]) -> Result<(), FtdiError<E>>{
        (&self.spi).transfer_in_place(data)
    }

    pub fn set_pin(&mut self, state: u8) -> Result<(), FtdiError<E>> {
        for i in 0..4 {
            if state & (0x01 << i) == 1 {
                self.pins[i].set_high()?;
            } else {
                self.pins[i].set_low()?;
            }
        }
        Ok(())
    }

}
