/// TEA5767 I2C slave address
pub(crate) const DEVICE_ADDRESS: u32 =  0x60;

/// TEA5767 device driver
#[derive(Debug)]
pub struct TEA5767<I2C> {
    pub(crate) i2c: I2C,
    pub(crate) address: u32,
    pub(crate) frequency: f32,
    pub(crate) band_limits: BandLimits,
    pub(crate) standby: bool,
    pub(crate) mute: MuteChannel,
    pub(crate) soft_mute: bool,
    pub(crate) search_mode: bool,
    pub(crate) search_mode_dir: SearchModeDirection,
    pub(crate) search_adc_level: SearchAdcLevel,
    pub(crate) injection_side: InjectionSide,
    pub(crate) sound_mode: SoundMode,
    pub(crate) high_cut_control: bool,
    pub(crate) stereo_noise_canceling: bool,
    pub(crate) clock_frequency: ClockFrequency,
    pub(crate) ready_flag: bool,
    pub(crate) band_limit_flag: bool,
    pub(crate) stereo_indication: bool,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SearchModeDirection {
    /// Up, default
    Up,
    /// Down
    Down,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SearchAdcLevel {
    /// low ADC output = 5
    Low,
    /// mid ADC output = 7
    Mid,
    /// high ADC output = 10
    High,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum InjectionSide {
    LowSide,
    HighSide,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SoundMode {
    Stereo,
    Mono,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MuteChannel {
    Right,
    Left,
    Both,
    None,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BandLimits {
    Japanese,
    EuropeUS,
}
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ClockFrequency {
    /// 13 Mhz
    Clk13Mhz,
    /// 32.768 Mhz
    Clk32_768Khz,
    /// 6.5 Mhz
    Clk6_5MHz,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DeemphasisTime {
    /// 75 μs
    Dtc75,
    /// 50 μs
    Dtc50,
}