use super::defs::*;
use super::regs::*;
use super::regs::DEVICE_ADDRESS;
use embedded_hal::blocking::i2c;
use bit_field::{BitField, BitArray};
use core::any::Any;

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
            software_programmable_port1: false,
            software_programmable_port2: false,
            search_indicator: false,
            deemphasis_time: DeemphasisTime::Dtc75
        };
        Ok(tea5767)
    }

    /// TEA5767 destructor
    pub fn destroy(self) -> Option<I2C> {
        Some(self.i2c)
    }

    /// Write preconfigured values to the device registers
    fn upload(&mut self) -> Result<(), E> {
        let mut write_bytes: [u8; 5] =  [0; 5];
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

        let pll = to_register_format_pll(
            to_decimal_pll(self.injection_side, self.clock_frequency, self.frequency)
                .unwrap()
        ).unwrap();

        write_bytes[0] = pll[0];
        write_bytes[1] = pll[1];

        match self.mute {
            MuteChannel::Both => write_bytes.get_mut(0).unwrap()
                .set_bit(WM_DB1_MUTE, true),
            _ => write_bytes.get(0).unwrap()
        };

        match self.search_mode {
            true => write_bytes.get_mut(0).unwrap()
                .set_bit(WM_DB1_SM, true),
            _ => write_bytes.get(0).unwrap()
        };

        match self.search_mode_dir {
            SearchModeDirection::Up => write_bytes.get_mut(2).unwrap()
                .set_bit(WM_DB3_SUD, true),
            _ => write_bytes.get(2).unwrap()
        };

        match self.search_adc_level {
            SearchAdcLevel::Low => write_bytes.get_mut(2).unwrap()
                .set_bits(WM_DB3_SSL, 0b01), // TODO names of bits
            SearchAdcLevel::Mid => write_bytes.get_mut(2).unwrap()
                .set_bits(WM_DB3_SSL, 0b10),
            SearchAdcLevel::High => write_bytes.get_mut(2).unwrap()
                .set_bits(WM_DB3_SSL, 0b11),
        };

        match self.injection_side {
            InjectionSide::HighSide => write_bytes.get_mut(2).unwrap()
                .set_bit(WM_DB3_HLSI, true),
            _ => write_bytes.get(2).unwrap()
        };

        match self.sound_mode {
            SoundMode::Mono => write_bytes.get_mut(2).unwrap()
                .set_bit(WM_DB3_MS, true),
            _ => write_bytes.get(2).unwrap()
        };

        match self.mute {
            MuteChannel::Right => write_bytes.get_mut(2).unwrap()
                .set_bit(WM_DB3_MR, true),
            MuteChannel::Left => write_bytes.get_mut(2).unwrap()
                .set_bit(WM_DB3_ML, true),
            _ => write_bytes.get(2).unwrap()
        };

        match self.software_programmable_port1 {
            true => write_bytes.get_mut(2).unwrap()
                .set_bit(WM_DB3_SWP1, true),
            _ => write_bytes.get(2).unwrap()
        };

        match self.software_programmable_port2 {
            true => write_bytes.get_mut(3).unwrap()
                .set_bit(WM_DB4_SWP2, true),
            _ => write_bytes.get(3).unwrap()
        };

        match self.standby {
            true => write_bytes.get_mut(3).unwrap()
                .set_bit(WM_DB4_STBY, true),
            _ => write_bytes.get(3).unwrap()
        };

        match self.high_cut_control {
            true => write_bytes.get_mut(3).unwrap()
                .set_bit(WM_DB4_HCC, true),
            _ => write_bytes.get(3).unwrap()
        };

        match self.stereo_noise_canceling {
            true => write_bytes.get_mut(3).unwrap()
                .set_bit(WM_DB4_SNC, true),
            _ => write_bytes.get(3).unwrap()
        };

        match self.search_indicator {
            true => write_bytes.get_mut(3).unwrap()
                .set_bit(WM_DB4_SI, true),
            _ => write_bytes.get(3).unwrap()
        };

        match self.clock_frequency {
            ClockFrequency::Clk32_768Khz => write_bytes.get_mut(3).unwrap()
                    .set_bit(WM_DB4_XTAL, true),
            ClockFrequency::Clk6_5MHz => write_bytes.get_mut(4).unwrap()
                .set_bit(WM_DB5_PLLREF, true),
            _ => write_bytes.get(3).unwrap()
        };

        match self.deemphasis_time {
            DeemphasisTime::Dtc75 => write_bytes.get_mut(4).unwrap()
                .set_bit(WM_DB5_DTC, true),
            _ => write_bytes.get(4).unwrap()
        };

        write_data(&mut self.i2c, write_bytes);
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

fn to_register_format_pll(decimal: u32) -> Option<[u8; 2]> {
    // use bit_field del this
    let pll_binary = [decimal.get_bits(8..14) as u8,
        decimal.get_bits(0..8) as u8];
    Some(pll_binary)
}

#[cfg(test)]
mod tests {
    use crate::device::*;
    use crate::defs::{InjectionSide, ClockFrequency};

    #[test]
    fn test_to_decimal_pll1() {
        assert_eq!(to_decimal_pll(InjectionSide::HighSide,
                                  ClockFrequency::Clk32_768Khz,
        89.9).unwrap(), 11001);
    }
    #[test]
    fn test_to_decimal_pll2() {
        assert_eq!(to_decimal_pll(InjectionSide::LowSide,
                                  ClockFrequency::Clk6_5MHz,
                                  89.9).unwrap(), 55);
    }
    #[test]
    fn test_to_decimal_pll3() {
        assert_eq!(to_decimal_pll(InjectionSide::LowSide,
                                  ClockFrequency::Clk13Mhz,
                                  89.9).unwrap(), 27);
    }
    #[test]
    fn test_to_register_format_pll() {
        assert_eq!(to_register_format_pll(11001).unwrap(),[0b0010_1010, 0b1111_1001]);
    }
}