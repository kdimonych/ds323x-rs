//! Functions exclusive of DS3232
maybe_async_cfg::content! {
#![maybe_async_cfg::default(
    idents(I2cInterface, Ds323x),
)]

#[maybe_async_cfg::maybe(
    sync(not(feature = "async")),
    async(feature = "async")
)]
use crate::{
    ic,BitFlags,interface::I2cInterface, Ds323x, Error, TempConvRate, CONTROL_POR_VALUE,
};
use core::marker::PhantomData;

#[cfg(not(feature = "async"))]
use embedded_hal::i2c;
#[cfg(feature = "async")]
use embedded_hal_async::i2c;

#[maybe_async_cfg::maybe(
    sync(not(feature = "async")),
    async(feature = "async")
)]
impl<I2C, E> Ds323x<I2cInterface<I2C>, ic::DS3232>
where
    I2C: i2c::I2c<Error = E>,
{
    /// Create a new instance of the DS3232 device.
    pub fn new_ds3232(i2c: I2C) -> Self {
        const STATUS_POR_VALUE: u8 = BitFlags::OSC_STOP | BitFlags::BB32KHZ | BitFlags::EN32KHZ;
        Ds323x {
            iface: I2cInterface { i2c },
            control: CONTROL_POR_VALUE,
            status: STATUS_POR_VALUE,
            _ic: PhantomData,
        }
    }

    /// Destroy driver instance, return I²C bus instance.
    pub fn destroy_ds3232(self) -> I2C {
        self.iface.i2c
    }

    /// Enable the 32kHz output when battery-powered. (enabled per default)
    ///
    /// Additionally, the 32kHz output needs to be enabled. See
    /// [`enable_32khz_output()`](#method.enable_32khz_output).
    ///
    /// Note: This is only available for DS3232 and DS3234 devices.
    pub async fn enable_32khz_output_on_battery(&mut self) -> Result<(), Error<E>> {
        let status = self.status | BitFlags::BB32KHZ;
        self.write_status_without_clearing_alarm(status).await
    }

    /// Disable the 32kHz output when battery-powered.
    ///
    /// The 32kHz output will still generate a wave when not battery-powered if
    /// it enabled. See [`enable_32khz_output()`](#method.enable_32khz_output).
    ///
    /// Note: This is only available for DS3232 and DS3234 devices.
    pub async fn disable_32khz_output_on_battery(&mut self) -> Result<(), Error<E>> {
        let status = self.status & !BitFlags::BB32KHZ;
        self.write_status_without_clearing_alarm(status).await
    }

    /// Set the temperature conversion rate.
    ///
    /// Set how often the temperature is measured and applies compensation to
    /// the oscillator. This can be used to reduce power consumption but sudden
    /// temperature changes will not be compensated for.
    ///
    /// Note: This is only available for DS3232 and DS3234 devices.
    pub async fn set_temperature_conversion_rate(&mut self, rate: TempConvRate) -> Result<(), Error<E>> {
        let status = match rate {
            TempConvRate::_64s => self.status & !BitFlags::CRATE1 & !BitFlags::CRATE0,
            TempConvRate::_128s => self.status & !BitFlags::CRATE1 | BitFlags::CRATE0,
            TempConvRate::_256s => self.status | BitFlags::CRATE1 & !BitFlags::CRATE0,
            TempConvRate::_512s => self.status | BitFlags::CRATE1 | BitFlags::CRATE0,
        };
        self.write_status_without_clearing_alarm(status).await
    }
}
}
