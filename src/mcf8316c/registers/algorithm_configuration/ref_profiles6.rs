use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RefProfiles6 {
    /// Turn off reference (% of Maximum Reference)
    pub ref_off2: PercentAsU8,
    /// Clamp Ref 2 (% of Maximum Reference)
    pub ref_clamp2: PercentAsU8,
}

impl Register for RefProfiles6 {
    const ADDRESS: u16 = REF_PROFILES6; // Example address, replace with actual address
}

impl From<RefProfiles6> for u32 {
    fn from(config: RefProfiles6) -> Self {
        let mut value = 0;
        value |= (config.ref_off2.inner as u32) << 23;
        value |= (config.ref_clamp2.inner as u32) << 15;
        value
    }
}

impl From<u32> for RefProfiles6 {
    fn from(value: u32) -> Self {
        RefProfiles6 {
            ref_off2: PercentAsU8::new(((value >> 23) & 0xFF) as u8),
            ref_clamp2: PercentAsU8::new(((value >> 15) & 0xFF) as u8),
        }
    }
}
