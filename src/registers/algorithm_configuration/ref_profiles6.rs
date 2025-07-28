//! Section 7.7.1.14

use super::*;
use arbitrary_int::*;
use bitbybit::bitfield;

/// Register to configure reference profile6
#[bitfield(u32, debug, default = 0x0)]
#[derive(PartialEq, Eq)]
pub struct RefProfiles6 {
    /// Turn off reference (% of Maximum Reference)
    #[bits(23..=30, rw)]
    pub ref_off2: PercentAsU8,
    /// Clamp Ref 2 (% of Maximum Reference)
    #[bits(15..=22, rw)]
    pub ref_clamp2: PercentAsU8,
}

impl Register for RefProfiles6 {
    const ADDRESS: u12 = REF_PROFILES6;

    fn value(&self) -> u32 {
        self.raw_value()
    }

    fn from_value(value: u32) -> Self {
        Self::new_with_raw_value(value)
    }
}
