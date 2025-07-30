//! Section 7.7.1.12

use super::*;
use arbitrary_int::*;
use bitbybit::*;

/// Register to configure reference profile4
#[bitfield(u32, debug, default = 0x0)]
#[derive(PartialEq, Eq)]
pub struct RefProfiles4 {
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

impl Register for RefProfiles4 {
    fn address(&self) -> u12 {
        REF_PROFILES4
    }

    fn value(&self) -> u32 {
        self.raw_value()
    }

    fn from_value(value: u32) -> Self {
        Self::new_with_raw_value(value)
    }
}
