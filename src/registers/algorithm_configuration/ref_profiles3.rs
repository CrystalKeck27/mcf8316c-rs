//! Section 7.7.1.11

use super::*;
use arbitrary_int::*;
use bitbybit::*;

/// Register to configure reference profile3
#[bitfield(u32, debug, default = 0x0)]
#[derive(PartialEq, Eq)]
pub struct RefProfiles3 {
    /// 4 LSB for Duty Cycle E
    #[bits(27..=30, rw)]
    pub duty_e: DutyELow4,
    /// Turn On Duty Cycle
    #[bits(19..=26, rw)]
    pub duty_on2: PercentAsU8,
    /// Turn Off Duty Cycle
    #[bits(11..=18, rw)]
    pub duty_off2: PercentAsU8,
    /// Duty Cycle for clamping Duty Input
    #[bits(3..=10, rw)]
    pub duty_clamp2: PercentAsU8,
    /// Duty Hysteresis
    #[bits(1..=2, rw)]
    pub duty_hys: DutyHysteresis,
}

impl Register for RefProfiles3 {
    const ADDRESS: u12 = REF_PROFILES3;

    fn value(&self) -> u32 {
        self.raw_value()
    }

    fn from_value(value: u32) -> Self {
        Self::new_with_raw_value(value)
    }
}

/// Duty hysteresis
#[bitenum(u2, exhaustive = true)]
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, strum::Display)]
pub enum DutyHysteresis {
    /// 0% Hysteresis
    #[strum(to_string = "0%")]
    P0 = 0x0,
    /// 0.5% Hysteresis
    #[strum(to_string = "0.5%")]
    P0_5 = 0x1,
    /// 1% Hysteresis
    #[strum(to_string = "1%")]
    P1 = 0x2,
    /// 2% Hysteresis
    #[strum(to_string = "2%")]
    P2 = 0x3,
}
