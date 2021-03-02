use super::defs::*;
use super::regs::*;
use super::regs::DEVICE_ADDRESS;
use embedded_hal::blocking::i2c;
use bit_field::{BitField, BitArray};

#[derive(Debug)]
struct TEA5767Flags {
    ready_flag: bool,
    band_limit_flag: bool,
    pll: [u8; 2],
    sound_mode_flag: SoundMode,
    adc_level_flag: u8,
    founded_frequency: f32,
}

impl<I2C, E> TEA5767<I2C>
where
    I2C: i2c::Write<Error = E> + i2c::Read<Error = E>
{
    /// TEA5767 constructor
    pub fn new(i2c: I2C, frequency: f32, band_limits: BandLimits,
               sound_mode: SoundMode) -> Result<Self, E> {
        let mut tea5767 = TEA5767 {
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
            clock_frequency: ClockFrequency::Clk32_768Khz,
            ready_flag: false,
            band_limit_flag: false,
            stereo_indication: false,
            software_programmable_port1: false,
            software_programmable_port2: false,
            search_indicator: false,
            deemphasis_time: DeemphasisTime::Dtc75
        };

        tea5767.upload()?;
        Ok(tea5767)
    }

    /// TEA5767 destructor
    pub fn destroy(self) -> Option<I2C> {
        Some(self.i2c)
    }

    /// Mute left and right channels
    pub fn mute(&mut self) -> Result<(), E> {
        self.mute = MuteChannel::Both;
        self.upload()
    }

    /// Mute left channel
    pub fn mute_left(&mut self) -> Result<(), E> {
        self.mute = MuteChannel::Left;
        self.upload()
    }

    /// Mute right channel
    pub fn mute_right(&mut self) -> Result<(), E> {
        self.mute = MuteChannel::Right;
        self.upload()
    }

    /// Unmute left and right channels
    pub fn unmute(&mut self) -> Result<(), E> {
        self.mute = MuteChannel::None;
        self.upload()
    }

    /// Unmute right channel
    pub fn unmute_right(&mut self) -> Result<(), E> {
        if self.mute == MuteChannel::Both {
            self.mute = MuteChannel::Left;
        } else if self.mute == MuteChannel::Left {
            self.mute = MuteChannel::Left;
        } else {
            self.mute = MuteChannel::None;
        }
        self.upload()
    }

    /// Unmute left channel
    pub fn unmute_left(&mut self) -> Result<(), E> {
        if self.mute == MuteChannel::Both {
            self.mute = MuteChannel::Right;
        } else if self.mute == MuteChannel::Right {
            self.mute = MuteChannel::Right;
        } else {
            self.mute = MuteChannel::None;
        }
        self.upload()
    }

    /// Enable standby mode
    pub fn set_standby(&mut self) -> Result<(), E> {
        self.standby = true;
        self.upload()
    }

    /// Disable standby mode
    pub fn reset_standby(&mut self) -> Result<(), E> {
        self.standby = false;
        self.upload()
    }

    /// Set band: Europe/US or Japanese
    pub fn set_band(&mut self, band: BandLimits) -> Result<(), E> {
        self.band_limits = band;
        self.upload()
    }

    /// Enable soft mute mode
    pub fn set_soft_mute(&mut self) -> Result<(), E> {
        self.soft_mute = true;
        self.upload()
    }

    /// Disable soft mute mode
    pub fn reset_soft_mute(&mut self) -> Result<(), E> {
        self.soft_mute = false;
        self.upload()
    }

    /// Set specified clock frequency
    pub fn set_clock_frequency(&mut self, clock_frequency: ClockFrequency)
        -> Result<(), E> {
        self.clock_frequency = clock_frequency;
        self.upload()
    }

    /// Set high cut mode
    pub fn set_high_cut_control(&mut self) -> Result<(), E> {
        self.high_cut_control = true;
        self.upload()
    }

    /// Reset high cut mode
    pub fn reset_high_cut_control(&mut self) -> Result<(), E> {
        self.high_cut_control = false;
        self.upload()
    }

    /// Set stereo noise canceling
    pub fn set_stereo_noise_canceling(&mut self) -> Result<(), E> {
        self.stereo_noise_canceling = true;
        self.upload()
    }

    /// Reset stereo noise canceling
    pub fn reset_stereo_noise_canceling(&mut self) -> Result<(), E> {
        self.stereo_noise_canceling = false;
        self.upload()
    }

    /// The de-emphasis time constant is 75 μs or 50 μs
    pub fn set_deemphasis_time(&mut self, deemphasis_time: DeemphasisTime)
        -> Result<(), E> {
        self.deemphasis_time = deemphasis_time;
        self.upload()
    }

    /// Set specified radio frequency
    pub fn set_frequency(&mut self, frequency: f32) -> Result<(), E> {
        self.frequency = frequency;
        self.upload()
    }

    pub fn set_stereo(&mut self) -> Result<(), E> {
        self.sound_mode = SoundMode::Stereo;
        self.upload()
    }

    pub fn set_mono(&mut self) -> Result<(), E> {
        self.sound_mode = SoundMode::Mono;
        self.upload()
    }

    /// Start searching for radio station up
    pub fn search_up(&mut self, signal_level: SearchAdcLevel) -> Result<SearchStatus, E> {
        self.search_adc_level = signal_level;
        self.search_mode_dir = SearchModeDirection::Up;
        self.search_mode = true;
        self.upload();

        let mut  status: SearchStatus;
        loop {
            let flags = self.download()?;

            if flags.band_limit_flag {
                status = SearchStatus::Failure;
                break;
            }
            if flags.ready_flag {
                // TODO
                status = SearchStatus::Success;
                break;
            }
        }
        Ok(status)

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

        if self.search_mode {
            self.search_mode = false;
        }

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
            MuteChannel::Left => write_bytes.get_mut(2).unwrap()
                .set_bit(WM_DB3_MR, true),
            MuteChannel::Right => write_bytes.get_mut(2).unwrap()
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

        write_data(&mut self.i2c, write_bytes)?;
        Ok(())
    }

    /// Read actual values from the device registers
    fn download(&mut self) -> Result<TEA5767Flags, E> {
        let mut read_bytes = read_data(&mut self.i2c)?;

        let mut flags = TEA5767Flags {
            ready_flag: false,
            band_limit_flag: false,
            pll: [0, 0],
            sound_mode_flag: SoundMode::Mono,
            adc_level_flag: 0,
            founded_frequency: 0.0
        };

        if read_bytes.get(0).unwrap().get_bit(RM_DB1_RF) {
            flags.ready_flag = true;
        }

        if read_bytes.get(0).unwrap().get_bit(RM_DB1_BLF) {
            flags.band_limit_flag = true;
        }

        let mut pll = [0_u8; 2];
        pll[0] = read_bytes[0];
        pll[1] = read_bytes[1];

        flags.founded_frequency = from_decimal_pll(
            self.injection_side,
            self.clock_frequency,
            from_register_format_pll(pll)
                .unwrap())
            .unwrap();

        if read_bytes.get(2).unwrap().get_bit(RM_DB3_STEREO) {
            flags.sound_mode_flag = SoundMode::Stereo;
        }

        flags.adc_level_flag = read_bytes.get(3).unwrap().get_bits(4..8);

        Ok(flags)
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

    let f_ref = match clock_frequency {
        ClockFrequency::Clk32_768Khz => 32_768_u32,
        ClockFrequency::Clk6_5MHz => 6_500_000_u32,
        ClockFrequency::Clk13Mhz => 13_000_000_u32,
    };

    Some(numerator / f_ref)
}

fn from_decimal_pll(injection_side: InjectionSide, clock_frequency: ClockFrequency,
                  decimal: u32) -> Option<f32> {
    let mut frequency: f32;
    let f_ref = match clock_frequency {
        ClockFrequency::Clk32_768Khz => 32_768_f32,
        ClockFrequency::Clk6_5MHz => 6_500_000_f32,
        ClockFrequency::Clk13Mhz => 13_000_000_f32,
    };
    match injection_side {
        InjectionSide::HighSide => {
            frequency = ((decimal as f32 * f_ref / 4.0) - 225_000.0) / 1_000_000.0;
        }
        InjectionSide::LowSide => {
            frequency = (decimal as f32 * f_ref / 4.0) + 225_000.0 / 1_000_000.0;
        }
    }
    Some(frequency)
}

fn to_register_format_pll(decimal: u32) -> Option<[u8; 2]> {
    let pll_binary = [decimal.get_bits(8..14) as u8,
        decimal.get_bits(0..8) as u8];
    Some(pll_binary)
}

fn from_register_format_pll(mut pll: [u8; 2]) -> Option<u32> {
    pll[0].set_bits(6..8, 0b00);
    let mut pll_decimal: u32 = 0;
    pll_decimal.set_bits(8..14, pll[0] as u32); // MSB
    pll_decimal.set_bits(0..8, pll[1] as u32); // LSB

    Some(pll_decimal)
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
    #[test]
    fn test_from_register_format_pll() {
        assert_eq!(from_register_format_pll([0b0010_1010, 0b1111_1001]).unwrap(), 11001);
    }

    #[test]
    fn test_from_decimal_format_pll() {
        assert_eq!(from_decimal_pll(InjectionSide::HighSide,
                                    ClockFrequency::Clk32_768Khz,
                                    11001).unwrap(),89.895195);
    }

}