//! Section 7.7.1.13

use super::*;
use arbitrary_int::*;
use bitbybit::bitfield;

/// Register to configure reference profile5
#[bitfield(u32, default = 0x0)]
#[derive(Debug, PartialEq, Eq)]
pub struct RefProfiles5 {
    /// 1 LSB for Ref B
    #[bit(30, rw)]
    pub ref_b: RefBLow1,
    /// Ref C (% of Maximum Reference)
    #[bits(22..=29, rw)]
    pub ref_c: PercentAsU8,
    /// Ref D (% of Maximum Reference)
    #[bits(14..=21, rw)]
    pub ref_d: PercentAsU8,
    /// Ref E (% of Maximum Reference)
    #[bits(6..=13, rw)]
    pub ref_e: PercentAsU8,
}

impl Register for RefProfiles5 {
    const ADDRESS: u12 = REF_PROFILES5;

    fn value(&self) -> u32 {
        self.raw_value()
    }
}
