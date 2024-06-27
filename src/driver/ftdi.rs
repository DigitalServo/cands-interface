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
    DeviceInfo,
    list_devices as list_ftdi_devices
};

//Error handling
use std::error::Error as StdError;
use std::io::{Error as IoError, ErrorKind as IoErrorKind};
use ftdi_embedded_hal::Error as FtdiError;

type IoResult<T> = Result<T, IoError>;

fn emap<E: StdError>() -> impl FnOnce(FtdiError<E>) -> IoError { |err| match err {
    FtdiError::Hal(_) => IoError::new(IoErrorKind::Other, "FTDI HAL ERROR"),
    FtdiError::Io(e) => e,
    FtdiError::Backend(e) => IoError::new(IoErrorKind::Other, e.to_string()),
} }

use super::{GpioDriver, SpiDriver, TCAN4550Driver, GPI_MAX_POINT};

pub struct FtdiDriver<DEVICE, E>
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
    #[allow(dead_code)]
    pub fn list_devices() -> Result<Vec<DeviceInfo>, Box<dyn StdError>> {
        match list_ftdi_devices() {
            Ok(list) => Ok(list),
            Err(e) => Err(e.into())
        }
    }

    pub fn find_device() -> Result<DEVICE, Box<dyn StdError>> {
        let device: Ftdi = Ftdi::new()?;
        let device: DEVICE = device.try_into()?;
        Ok(device)
    }

    pub fn new(spi_clk_freq: u32, spi_clk_polarity: u8) -> IoResult<Self> {

        let device: DEVICE = match Self::find_device() {
            Ok(device) => device,
            Err(_) => return Err(IoError::new(IoErrorKind::NotConnected, "Device Not Found."))
        };

        let hal: FtHal<DEVICE> = FtHal::init_freq(device, spi_clk_freq).map_err(emap::<E>())?;

        const SPI_CS_INDEX: u8 = 3;
        let spi_clk_polarity: Polarity = if spi_clk_polarity == 0 { Polarity::IdleLow } else {Polarity::IdleHigh};
        let mut spi: SpiDevice<DEVICE> = hal.spi_device(SPI_CS_INDEX).map_err(emap::<E>())?;
        spi.set_clock_polarity(spi_clk_polarity);
        
        let pin_1: OutputPin<DEVICE> = hal.ad4().map_err(emap::<E>())?;
        let pin_2: OutputPin<DEVICE> = hal.ad5().map_err(emap::<E>())?;
        let pin_3: OutputPin<DEVICE> = hal.ad6().map_err(emap::<E>())?;
        let pin_4: OutputPin<DEVICE> = hal.ad7().map_err(emap::<E>())?;
        
        let pins: [OutputPin<DEVICE>; 4] = [pin_1, pin_2, pin_3, pin_4];

        Ok(Self { spi, pins })
    }
}

impl <DEVICE, E> SpiDriver for FtdiDriver <DEVICE, E>
where
    DEVICE: MpsseCmdExecutor<Error = E> + TryFrom<Ftdi>,
    <DEVICE as TryFrom<Ftdi>>::Error: StdError + 'static,
    E: StdError,
    FtdiError<E>: From<E>,
{
    fn spi_write(&mut self, data: &[u8]) -> IoResult<usize> {
        (&self.spi).write(data).map_err(emap::<E>())?;
        Ok(data.len())
    }

    fn spi_read(&mut self, _buffer: &mut [u8]) -> IoResult<usize> {
        Ok(0)
    }

    fn spi_transfer(&mut self, data: &[u8], buffer: &mut [u8]) -> IoResult<usize> {
        (&self.spi).transfer(buffer, data).map_err(emap::<E>())?;
        Ok(data.len())
    }

    fn spi_transfer_in_place(&mut self, data: &mut [u8]) -> IoResult<usize> {
        (&self.spi).transfer_in_place(data).map_err(emap::<E>())?;
        Ok(data.len())
    }
}


impl <DEVICE, E> GpioDriver for FtdiDriver <DEVICE, E>
where
    DEVICE: MpsseCmdExecutor<Error = E> + TryFrom<Ftdi>,
    <DEVICE as TryFrom<Ftdi>>::Error: StdError + 'static,
    E: StdError,
    FtdiError<E>: From<E>,
{
    fn gpio_out(&mut self, state: u8) -> IoResult<()> {
        for i in 0..4 {
            if state & (0x01 << i) == 1 {
                self.pins[i].set_high().map_err(emap::<E>())?;
            } else {
                self.pins[i].set_low().map_err(emap::<E>())?;
            }
        }
        Ok(())
    }

    fn gpio_read(&mut self, _channel: usize) -> IoResult<bool> {
        Ok(false)
    }

    fn gpio_read_all(&mut self) -> IoResult<[bool; GPI_MAX_POINT]> {
        let ret: [bool; GPI_MAX_POINT] = [false; GPI_MAX_POINT];
        Ok(ret)
    }
}


impl <DEVICE, E> TCAN4550Driver for FtdiDriver <DEVICE, E>
where
    DEVICE: MpsseCmdExecutor<Error = E> + TryFrom<Ftdi>,
    <DEVICE as TryFrom<Ftdi>>::Error: StdError + 'static,
    E: StdError,
    FtdiError<E>: From<E>,
{
    fn reset_tcan4550(&mut self) -> super::IoResult<()> {

        const RESET_WAIT_TIME: u64 = 5;
        
        self.gpio_out(0x01)?;
        std::thread::sleep(std::time::Duration::from_millis(RESET_WAIT_TIME));
        self.gpio_out(0x00)?;
        std::thread::sleep(std::time::Duration::from_millis(RESET_WAIT_TIME));
        Ok(())
    }
}