//! I2C/SPI interfaces
maybe_async_cfg::content! {
#![maybe_async_cfg::default(
    idents(ReadData, WriteData, Ds323x),
)]

use crate::{private, Error, DEVICE_ADDRESS};

#[cfg(not(feature = "async"))]
use embedded_hal::{i2c, spi};
#[cfg(feature = "async")]
use embedded_hal_async::{i2c, spi};

/// I2C interface
#[maybe_async_cfg::maybe(
    sync(not(feature = "async")),
    async(feature = "async")
)]
#[derive(Debug, Default)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct I2cInterface<I2C> {
    pub(crate) i2c: I2C,
}

/// SPI interface
#[maybe_async_cfg::maybe(
    sync(not(feature = "async")),
    async(feature = "async")
)]
#[derive(Debug, Default)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct SpiInterface<SPI> {
    pub(crate) spi: SPI,
}

/// Write data
#[maybe_async_cfg::maybe(
    sync(not(feature = "async")),
    async(feature = "async")
)]
pub trait WriteData: private::Sealed {
    /// Error type
    type Error;
    /// Write to an u8 register
    async fn write_register(&mut self, register: u8, data: u8) -> Result<(), Self::Error>;
    /// Write data. The first element corresponds to the starting address.
    async fn write_data(&mut self, payload: &mut [u8]) -> Result<(), Self::Error>;
}

#[maybe_async_cfg::maybe(
    sync(not(feature = "async")),
    async(feature = "async")
)]
impl<I2C, E> WriteData for I2cInterface<I2C>
where
    I2C: i2c::I2c<Error = E>,
{
    type Error = Error<E>;
    async fn write_register(&mut self, register: u8, data: u8) -> Result<(), Self::Error> {
        let payload: [u8; 2] = [register, data];
        self.i2c
            .write(DEVICE_ADDRESS, &payload)
            .await
            .map_err(Error::Comm)
    }

    async fn write_data(&mut self, payload: &mut [u8]) -> Result<(), Self::Error> {
        self.i2c
            .write(DEVICE_ADDRESS, payload)
            .await
            .map_err(Error::Comm)
    }
}

#[maybe_async_cfg::maybe(
    sync(not(feature = "async")),
    async(feature = "async")
)]
impl<SPI, E> WriteData for SpiInterface<SPI>
where
    SPI: spi::SpiDevice<u8, Error = E>,
{
    type Error = Error<E>;
    async fn write_register(&mut self, register: u8, data: u8) -> Result<(), Self::Error> {
        let payload: [u8; 2] = [register + 0x80, data];
        self.spi.write(&payload).await.map_err(Error::Comm)
    }

    async fn write_data(&mut self, payload: &mut [u8]) -> Result<(), Self::Error> {
        payload[0] += 0x80;
        self.spi.write(payload).await.map_err(Error::Comm)
    }
}

/// Read data
#[maybe_async_cfg::maybe(
    sync(not(feature = "async")),
    async(feature = "async")
)]
pub trait ReadData: private::Sealed {
    /// Error type
    type Error;
    /// Read an u8 register
    async fn read_register(&mut self, register: u8) -> Result<u8, Self::Error>;
    /// Read some data. The first element corresponds to the starting address.
    async fn read_data(&mut self, payload: &mut [u8]) -> Result<(), Self::Error>;
}

#[maybe_async_cfg::maybe(
    sync(not(feature = "async")),
    async(feature = "async")
)]
impl<I2C, E> ReadData for I2cInterface<I2C>
where
    I2C: i2c::I2c<Error = E>,
{
    type Error = Error<E>;
    async fn read_register(&mut self, register: u8) -> Result<u8, Self::Error> {
        let mut data = [0];
        self.i2c
            .write_read(DEVICE_ADDRESS, &[register], &mut data)
            .await
            .map_err(Error::Comm)
            .and(Ok(data[0]))
    }

    async fn read_data(&mut self, payload: &mut [u8]) -> Result<(), Self::Error> {
        let len = payload.len();
        self.i2c
            .write_read(DEVICE_ADDRESS, &[payload[0]], &mut payload[1..len])
            .await
            .map_err(Error::Comm)
    }
}

#[maybe_async_cfg::maybe(
    sync(not(feature = "async")),
    async(feature = "async")
)]
impl<SPI, E> ReadData for SpiInterface<SPI>
where
    SPI: spi::SpiDevice<u8, Error = E>,
{
    type Error = Error<E>;
    async fn read_register(&mut self, register: u8) -> Result<u8, Self::Error> {
        let mut data = [register, 0];
        let result = self
            .spi
            .transfer_in_place(&mut data)
            .await
            .map_err(Error::Comm);
        result.and(Ok(data[1]))
    }

    async fn read_data(&mut self, payload: &mut [u8]) -> Result<(), Self::Error> {
        self.spi
            .transfer_in_place(payload)
            .await
            .map_err(Error::Comm)
    }
}
}
