//! Section 7.7.3.4

use super::*;
use arbitrary_int::*;
use bitbybit::*;

/// Register to configure PERI_CONFIG1 register
pub const PERI_CONFIG1_RESET: u32 = 0b01000000_00000000_00000000_00000000;

/// Register to peripheral
#[bitfield(u32, default = PERI_CONFIG1_RESET)]
#[derive(Debug, PartialEq, Eq)]
pub struct PeriConfig1 {
    /// Spread Spectrum Modulation disable.
    /// 0 = SSM is Enabled, 1 = SSM is Disabled
    #[bit(30, rw)]
    pub spread_spectrum_modulation_disable: bool,
    /// Bus current limit.
    #[bits(22..=25, rw)]
    pub bus_current_limit: CurrentSelection,
    /// Bus current limit enable.
    /// 0 = Disable, 1 = Enable
    #[bit(21, rw)]
    pub bus_current_limit_en: bool,
    /// DIR pin override
    #[bits(19..=20, rw)]
    pub dir_input: DirectionPinOverride,
    /// Response to change of DIR pin status.
    /// 0 = Follow motor stop options and ISD routine on detecting DIR change,
    /// 1 = Change the direction through Reverse Drive while continuously driving the motor
    #[bit(18, rw)]
    pub dir_change_mode: bool,
    /// Enables self-test on power up.
    /// 0 = STL is disabled, 1 = STL is enabled
    #[bit(17, rw)]
    pub self_test_enable: bool,
    /// Difference between final speed and present speed below which
    /// active braking will be applied.
    #[bits(13..=16, rw)]
    pub active_brake_speed_delta_limit_entry: Option<BrakeDeltaLimit>,
    /// Modulation Index limit below which active braking will be applied
    #[bits(10..=12, rw)]
    pub active_brake_mod_index_limit: ModulationIndexLimit,
    /// Frequency range selection for PWM/duty based motor control input
    /// 0 = 325Hz to 100kHz, 1 = 10Hz to 325Hz
    #[bit(9, rw)]
    pub speed_range_sel: bool
}

impl Register for PeriConfig1 {
    const ADDRESS: u12 = PERI_CONFIG1;

    fn value(&self) -> u32 {
        self.raw_value()
    }
}

/// DIR pin override
#[bitenum(u2, exhaustive = true)]
#[derive(Debug, PartialEq, Eq, strum::Display)]
pub enum DirectionPinOverride {
    /// Hardware Pin DIR
    #[strum(to_string = "Hardware Pin DIR")]
    Hardware = 0x0,
    /// Override DIR pin with clockwise rotation OUTA-OUTB-OUTC
    #[strum(to_string = "Override DIR pin with clockwise rotation OUTA-OUTB-OUTC")]
    OverrideClockwise = 0x1,
    /// Override DIR pin with counter-clockwise rotation OUTA-OUTC-OUTB
    #[strum(to_string = "Override DIR pin with counter-clockwise rotation OUTA-OUTC-OUTB")]
    OverrideCounterClockwise = 0x2,
    /// Hardware Pin DIR (Same as Hardware)
    #[strum(to_string = "Hardware Pin DIR")]
    Hardware2 = 0x3,
}

/// Difference between final speed and present speed below which
/// active braking will be applied
#[bitenum(u4, exhaustive = false)]
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, strum::Display)]
pub enum BrakeDeltaLimit {
    /// 5%
    #[strum(to_string = "5%")]
    P5 = 0x1,
    /// 10%
    #[strum(to_string = "10%")]
    P10 = 0x2,
    /// 15%
    #[strum(to_string = "15%")]
    P15 = 0x3,
    /// 20%
    #[strum(to_string = "20%")]
    P20 = 0x4,
    /// 25%
    #[strum(to_string = "25%")]
    P25 = 0x5,
    /// 30%
    #[strum(to_string = "30%")]
    P30 = 0x6,
    /// 35%
    #[strum(to_string = "35%")]
    P35 = 0x7,
    /// 40%
    #[strum(to_string = "40%")]
    P40 = 0x8,
    /// 45%
    #[strum(to_string = "45%")]
    P45 = 0x9,
    /// 50%
    #[strum(to_string = "50%")]
    P50 = 0xA,
    /// 60%
    #[strum(to_string = "60%")]
    P60 = 0xB,
    /// 70%
    #[strum(to_string = "70%")]
    P70 = 0xC,
    /// 80%
    #[strum(to_string = "80%")]
    P80 = 0xD,
    /// 90%
    #[strum(to_string = "90%")]
    P90 = 0xE,
    /// 100%
    #[strum(to_string = "100%")]
    P100 = 0xF,
}

/// Modulation Index limit below which active braking will be applied
#[bitenum(u3, exhaustive = true)]
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, strum::Display)]
pub enum ModulationIndexLimit {
    /// 0%
    #[strum(to_string = "0%")]
    P0 = 0x0,
    /// 40%
    #[strum(to_string = "40%")]
    P40 = 0x1,
    /// 50%
    #[strum(to_string = "50%")]
    P50 = 0x2,
    /// 60%
    #[strum(to_string = "60%")]
    P60 = 0x3,
    /// 70%
    #[strum(to_string = "70%")]
    P70 = 0x4,
    /// 80%
    #[strum(to_string = "80%")]
    P80 = 0x5,
    /// 90%
    #[strum(to_string = "90%")]
    P90 = 0x6,
    /// 100%
    #[strum(to_string = "100%")]
    P100 = 0x7,
}
