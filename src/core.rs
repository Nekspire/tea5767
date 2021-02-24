use super::defs::*;
use embedded_hal::blocking::i2c;

impl<I2C, E> TEA5767<I2C>
where
    I2C: i2c::Write<Error = E> + i2c::Read<Error = E>
{
    /// TEA5767 constructor
    pub fn new(i2c: I2C, frequency: f32, band_limits: BandLimits,
               sound_mode: SoundMode) -> Result<Self, E> {
        let tea5767 = TEA5767 {
            i2c,
            address: DEVICE_ADDRESS,
            frequency,
            band_limits,
            standby: false,
            mute: MuteChannel::None,
            soft_mute: false,
            search_mode: false,
            search_mode_dir: SearchModeDirection::Up,
            search_adc_level: SearchAdcLevel::Low,
            injection_side: InjectionSide::LowSide,
            sound_mode,
            high_cut_control: false,
            stereo_noise_canceling: true,
            clock_frequency: ClockFrequency::Clk13Mhz,
            ready_flag: false,
            band_limit_flag: false,
            stereo_indication: false,
        };
        Ok(tea5767)
    }

    /// TEA5767 destructor
    pub fn destroy(self) -> Option<I2C> {
        Some(self.i2c)
    }

    /// Write all data to registers
    fn upload(&mut self) -> Result<(), E> {
        // todo
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_upload() {

    }
}