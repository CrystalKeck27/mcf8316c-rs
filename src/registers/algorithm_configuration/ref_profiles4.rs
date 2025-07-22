use super::*;
use bitbybit::*;

#[bitfield(u32, default = 0x0)]
#[derive(Debug, PartialEq, Eq)]
pub struct RefProfiles1 {
    /// Turn off reference (% of Maximum Reference)
    #[bits(23..=30, rw)]
    pub ref_off1: PercentAsU8,
    /// Clamp Ref 1 (% of Maximum Reference)
    #[bits(15..=22, rw)]
    pub ref_clamp1: PercentAsU8,
    /// Ref A (% of Maximum Reference)
    #[bits(7..=14, rw)]
    pub ref_a: PercentAsU8,
    /// 7 MSB for Ref B
    #[bits(0..=6, rw)]
    pub ref_b: RefBHigh7,
}

impl Register for RefProfiles1 {
    const ADDRESS: u16 = REF_PROFILES1;

    fn value(&self) -> u32 {
        self.raw_value()
    }
}

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
