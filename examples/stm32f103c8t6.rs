#![deny(unsafe_code)]
#![no_std]
#![no_main]

use panic_halt as _;

use nb::block;

use cortex_m_rt::entry;
use stm32f1xx_hal::{pac, prelude::*};
use tea5767::defs;

use stm32f1xx_hal::{
    i2c::{BlockingI2c, Mode, DutyCycle}
};

use tea5767::defs::*;

#[entry]
fn main() -> ! {

    let cp = cortex_m::Peripherals::take().unwrap();
    let dp = pac::Peripherals::take().unwrap();

    let mut flash = dp.FLASH.constrain();
    let mut rcc = dp.RCC.constrain();

    let clocks = rcc.cfgr.freeze(&mut flash.acr);

    let mut afio = dp.AFIO.constrain(&mut rcc.apb2);

    let mut gpiob = dp.GPIOB.split(&mut rcc.apb2);

    let scl = gpiob.pb6.into_alternate_open_drain(&mut gpiob.crl);
    let sda = gpiob.pb7.into_alternate_open_drain(&mut gpiob.crl);

    let i2c = BlockingI2c::i2c1(
        dp.I2C1,
        (scl, sda),
        &mut afio.mapr,
        Mode::Fast {
            frequency: 400_000.hz(),
            duty_cycle: DutyCycle::Ratio2to1,
        },
        clocks,
        &mut rcc.apb1,
        1000,
        10,
        1000,
        1000,
    );

    //  Device driver initialization with default values. This values can be found in defs mod.
    let mut radio_tuner = TEA5767::new(
        i2c,
        107.0,
        BandLimits::EuropeUS,
        SoundMode::Stereo
    ).unwrap();

    // start searching down for radio channel from  frequency: 107.0 MHz
    // stop at low level signal strength
    let stat = radio_tuner.search_down(SearchAdcLevel::Low,
                                       107.0).unwrap();

    match stat {
        SearchStatus::Success => {
            // radio station is found, get that frequency
            let freq = radio_tuner.get_frequency().unwrap();
            // get signal strength level;
            let signal_level = radio_tuner.get_signal_level().unwrap();
            // get sound mode information, stereo or mono reception
            let mode = radio_tuner.get_sound_mode().unwrap();
        }
        _ => (),
    }

    // set new channel frequency
    radio_tuner.set_frequency(106.1).unwrap();
    // mute both channels
    radio_tuner.mute();
    // unmute both channels
    radio_tuner.unmute();
    // mute right channel
    radio_tuner.mute_right();
    // unmute right channel
    radio_tuner.unmute_right();
    // mute left channel
    radio_tuner.mute_left();
    // unmute left channel
    radio_tuner.unmute_left();
    // set standby mode, device works at low power consumption
    radio_tuner.set_standby();
    // reset standby mode
    radio_tuner.reset_standby();
    // set mono mode
    radio_tuner.set_mono();
    // set stereo mode
    radio_tuner.set_stereo();
    // destroy TEA567 instance
    let i2c = radio_tuner.destroy().unwrap();
    // and create again...
    let mut radio_tuner = TEA5767::new(
        i2c,
        107.0,
        BandLimits::EuropeUS,
        SoundMode::Stereo
    ).unwrap();

    loop {

    }
}