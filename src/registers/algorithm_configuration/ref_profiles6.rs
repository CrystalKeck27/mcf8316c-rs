use super::*;
use bitbybit::bitfield;

#[bitfield(u32)]
#[derive(Debug, PartialEq, Eq)]
pub struct RefProfiles6 {
    /// Turn off reference (% of Maximum Reference)
    #[bits(23..=30, rw)]
    pub ref_off2: PercentAsU8,
    /// Clamp Ref 2 (% of Maximum Reference)
    #[bits(15..=22, rw)]
    pub ref_clamp2: PercentAsU8,
}

impl Register for RefProfiles6 {
    const ADDRESS: u16 = REF_PROFILES6; // Example address, replace with actual address
}
