//! Device configuration
maybe_async_cfg::content! {
#![maybe_async_cfg::default(
    idents(ReadData, WriteData, Ds323x),
)]

#[maybe_async_cfg::maybe(
    sync(not(feature = "async")),
    async(feature = "async")
)]
use crate::interface::{ReadData, WriteData};
#[maybe_async_cfg::maybe(
    sync(not(feature = "async")),
    async(feature = "async")
)]
use crate::{BitFlags, Ds323x, Error, Register, SqWFreq};



#[maybe_async_cfg::maybe(
    sync(not(feature = "async")),
    async(feature = "async")
)]
impl<DI, IC, E> Ds323x<DI, IC>
where
    DI: ReadData<Error = Error<E>> + WriteData<Error = Error<E>>,
{
    /// Enable the oscillator (set the clock running) (default).
    pub async fn enable(&mut self) -> Result<(), Error<E>> {
        let control = self.control;
        self.write_control(control & !BitFlags::EOSC).await
    }

    /// Disable the oscillator (stops the clock).
    pub async fn disable(&mut self) -> Result<(), Error<E>> {
        let control = self.control;
        self.write_control(control | BitFlags::EOSC).await
    }

    /// Force a temperature conversion and time compensation with TXCO algorithm.
    ///
    /// The *busy* status should be checked before doing this. See [`busy()`](#method.busy)
    pub async fn convert_temperature(&mut self) -> Result<(), Error<E>> {
        let control = self.iface.read_register(Register::CONTROL).await?;
        // do not overwrite if a conversion is in progress
        if (control & BitFlags::TEMP_CONV) == 0 {
            self.iface
                .write_register(Register::CONTROL, control | BitFlags::TEMP_CONV)
                .await?;
        }
        Ok(())
    }

    /// Enable the 32kHz output. (enabled per default)
    pub async fn enable_32khz_output(&mut self) -> Result<(), Error<E>> {
        let status = self.status | BitFlags::EN32KHZ;
        self.write_status_without_clearing_alarm(status).await
    }

    /// Disable the 32kHz output.
    pub async fn disable_32khz_output(&mut self) -> Result<(), Error<E>> {
        let status = self.status & !BitFlags::EN32KHZ;
        self.write_status_without_clearing_alarm(status).await
    }

    /// Set the aging offset.
    pub async fn set_aging_offset(&mut self, offset: i8) -> Result<(), Error<E>> {
        self.iface
            .write_register(Register::AGING_OFFSET, offset as u8)
            .await
    }

    /// Read the aging offset.
    pub async fn aging_offset(&mut self) -> Result<i8, Error<E>> {
        let offset = self.iface.read_register(Register::AGING_OFFSET).await?;
        Ok(offset as i8)
    }

    /// Set the interrupt/square-wave output to be used as interrupt output.
    pub async fn use_int_sqw_output_as_interrupt(&mut self) -> Result<(), Error<E>> {
        let control = self.control;
        self.write_control(control | BitFlags::INTCN).await
    }

    /// Set the interrupt/square-wave output to be used as square-wave output. (default)
    pub async fn use_int_sqw_output_as_square_wave(&mut self) -> Result<(), Error<E>> {
        let control = self.control;
        self.write_control(control & !BitFlags::INTCN).await
    }

    /// Enable battery-backed square wave generation.
    ///
    pub async fn enable_square_wave(&mut self) -> Result<(), Error<E>> {
        let control = self.control;
        self.write_control(control | BitFlags::BBSQW).await
    }

    /// Disable battery-backed square wave generation.
    pub async fn disable_square_wave(&mut self) -> Result<(), Error<E>> {
        let control = self.control;
        self.write_control(control & !BitFlags::BBSQW).await
    }

    /// Set the square-wave output frequency.
    pub async fn set_square_wave_frequency(&mut self, freq: SqWFreq) -> Result<(), Error<E>> {
        let new_control = match freq {
            SqWFreq::_1Hz => self.control & !BitFlags::RS2 & !BitFlags::RS1,
            SqWFreq::_1_024Hz => self.control & !BitFlags::RS2 | BitFlags::RS1,
            SqWFreq::_4_096Hz => self.control | BitFlags::RS2 & !BitFlags::RS1,
            SqWFreq::_8_192Hz => self.control | BitFlags::RS2 | BitFlags::RS1,
        };
        self.write_control(new_control).await
    }

    /// Enable Alarm1 interrupts.
    pub async fn enable_alarm1_interrupts(&mut self) -> Result<(), Error<E>> {
        let control = self.control;
        self.write_control(control | BitFlags::ALARM1_INT_EN).await
    }

    /// Disable Alarm1 interrupts.
    pub async fn disable_alarm1_interrupts(&mut self) -> Result<(), Error<E>> {
        let control = self.control;
        self.write_control(control & !BitFlags::ALARM1_INT_EN).await
    }

    /// Enable Alarm2 interrupts.
    pub async fn enable_alarm2_interrupts(&mut self) -> Result<(), Error<E>> {
        let control = self.control;
        self.write_control(control | BitFlags::ALARM2_INT_EN).await
    }

    /// Disable Alarm2 interrupts.
    pub async fn disable_alarm2_interrupts(&mut self) -> Result<(), Error<E>> {
        let control = self.control;
        self.write_control(control & !BitFlags::ALARM2_INT_EN).await
    }

    async fn write_control(&mut self, control: u8) -> Result<(), Error<E>> {
        self.iface
            .write_register(Register::CONTROL, control)
            .await?;
        self.control = control;
        Ok(())
    }

    pub(crate) async fn write_status_without_clearing_alarm(
        &mut self,
        status: u8,
    ) -> Result<(), Error<E>> {
        // avoid clearing alarm flags
        let new_status = status | BitFlags::ALARM2F | BitFlags::ALARM1F;
        self.iface
            .write_register(Register::STATUS, new_status)
            .await?;
        self.status = status;
        Ok(())
    }
}
}
