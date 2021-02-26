use super::defs::*;
use super::regs;
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
            address: regs::DEVICE_ADDRESS,
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
        let buffer: [u8; 5];
        match self.band_limits {
            BandLimits::EuropeUS => {
                if self.frequency < 87.5 {
                    self.frequency = 87.5
                }
                else if self.frequency > 108.0 {
                    self.frequency = 108.0;
                }
            }
            BandLimits::Japanese => {
                if self.frequency < 76.0 {
                    self.frequency = 76.0
                }
                else if self.frequency > 91.0 {
                    self.frequency = 91.0;
                }
            }
        }

        Ok(())
    }
}

fn to_decimal_pll(injection_side: InjectionSide, clock_frequency: ClockFrequency,
                  frequency: f32) -> Option<u32> {
    let mut numerator: u32 = 0 ;
    match injection_side {
        InjectionSide::HighSide => {
            numerator = (4.0 * (frequency * 1_000_000.0 + 225_000.0)) as u32;
        }
        InjectionSide::LowSide => {
            numerator = (4.0 * (frequency * 1_000_000.0 - 225_000.0)) as u32;
        }
    }
    let mut f_ref: u32 = 0;
    match clock_frequency {
        ClockFrequency::Clk32_768Khz => f_ref = 32_768,
        ClockFrequency::Clk6_5MHz => f_ref = 6_500_000,
        ClockFrequency::Clk13Mhz => f_ref = 13_000_000,
    }

    Some(numerator / f_ref)
}

fn to_register_format_pll(decimal: u32) -> Option<u32> {
    // use bit_field del this
    Some(decimal)
}

#[cfg(test)]
mod tests {
    use crate::device::to_decimal_pll;
    use crate::defs::{InjectionSide, ClockFrequency};

    #[test]
    fn test_to_decimal_pll() {
        assert_eq!(to_decimal_pll(InjectionSide::HighSide,
                                  ClockFrequency::Clk32_768Khz,
        89.9).unwrap(), 11001);
    }
}