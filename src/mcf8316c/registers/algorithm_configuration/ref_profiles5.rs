use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RefProfiles5 {
    /// 1 LSB for Ref B
    pub ref_b: RefBLow1,
    /// Ref C (% of Maximum Reference)
    pub ref_c: PercentAsU8,
    /// Ref D (% of Maximum Reference)
    pub ref_d: PercentAsU8,
    /// Ref E (% of Maximum Reference)
    pub ref_e: PercentAsU8,
}

impl Register for RefProfiles5 {
    const ADDRESS: u16 = REF_PROFILES5; // Example address, replace with actual address
}

impl From<RefProfiles5> for u32 {
    fn from(config: RefProfiles5) -> Self {
        let mut value = 0;
        value |= (config.ref_b.0 as u32 & 0x01) << 30; // 1 bit
        value |= (config.ref_c.inner as u32 & 0xFF) << 22;
        value |= (config.ref_d.inner as u32 & 0xFF) << 14;
        value |= (config.ref_e.inner as u32 & 0xFF) << 6;
        value
    }
}

impl From<u32> for RefProfiles5 {
    fn from(value: u32) -> Self {
        RefProfiles5 {
            ref_b: RefBLow1(((value >> 30) & 0x01) as u8),
            ref_c: PercentAsU8::new(((value >> 22) & 0xFF) as u8),
            ref_d: PercentAsU8::new(((value >> 14) & 0xFF) as u8),
            ref_e: PercentAsU8::new(((value >> 6) & 0xFF) as u8),
        }
    }
}
