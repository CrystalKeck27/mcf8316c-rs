//! Section 7.7.1.9

use super::*;
use arbitrary_int::*;
use bitbybit::*;

/// Register to configure reference profile1
#[bitfield(u32, debug, default = 0x0)]
#[derive(PartialEq, Eq)]
pub struct RefProfiles1 {
    /// Configuration for reference profiles
    #[bits(29..=30, rw)]
    pub ref_profile_config: RefProfileConfig,
    /// Turn On Duty Cycle
    #[bits(21..=28, rw)]
    pub duty_on1: PercentAsU8,
    /// Turn Off Duty Cycle
    #[bits(13..=20, rw)]
    pub duty_off1: PercentAsU8,
    /// Duty Cycle for clamping Duty Input
    #[bits(5..=12, rw)]
    pub duty_clamp1: PercentAsU8,
    /// 5 MSB for Duty Cycle A
    #[bits(0..=4, rw)]
    pub duty_a: DutyAHigh5,
}

impl Register for RefProfiles1 {
    const ADDRESS: u12 = REF_PROFILES1;

    fn value(&self) -> u32 {
        self.raw_value()
    }

    fn from_value(value: u32) -> Self {
        Self::new_with_raw_value(value)
    }
}

/// Configuration for Reference profiles
#[bitenum(u2, exhaustive = true)]
#[derive(Debug, PartialEq, Eq, strum::Display)]
pub enum RefProfileConfig {
    /// Reference/Equation
    #[strum(to_string = "Reference/Equation")]
    RefEq = 0x0,
    /// Linear Profile
    #[strum(to_string = "Linear Profile")]
    Linear = 0x1,
    /// Staircase Profile
    #[strum(to_string = "Staircase Profile")]
    Staircase = 0x2,
    /// Forward-Reverse Profile
    #[strum(to_string = "Forward-Reverse Profile")]
    ForwardReverse = 0x3,
}
