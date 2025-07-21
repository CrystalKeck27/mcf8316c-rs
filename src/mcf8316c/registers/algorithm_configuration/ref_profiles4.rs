use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RefProfiles1 {
    /// Turn off reference (% of Maximum Reference)
    pub ref_off1: PercentAsU8,
    /// Clamp Ref 1 (% of Maximum Reference)
    pub ref_clamp1: PercentAsU8,
    /// Ref A (% of Maximum Reference)
    pub ref_a: PercentAsU8,
    /// 7 MSB for Ref B
    pub ref_b: RefBHigh7,
}

impl Register for RefProfiles1 {
    const ADDRESS: u16 = REF_PROFILES1; // Example address, replace with actual address
}

impl From<RefProfiles1> for u32 {
    fn from(config: RefProfiles1) -> Self {
        let mut value = 0;
        value |= (config.ref_off1.inner as u32) << 23;
        value |= (config.ref_clamp1.inner as u32) << 15;
        value |= (config.ref_a.inner as u32) << 7;
        value |= (config.ref_b.0 as u32) & 0x7F; // 7 bits
        value
    }
}

impl From<u32> for RefProfiles1 {
    fn from(value: u32) -> Self {
        RefProfiles1 {
            ref_off1: PercentAsU8::new(((value >> 23) & 0xFF) as u8),
            ref_clamp1: PercentAsU8::new(((value >> 15) & 0xFF) as u8),
            ref_a: PercentAsU8::new(((value >> 7) & 0xFF) as u8),
            ref_b: RefBHigh7((value & 0x7F) as u8),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, strum::Display)]
#[repr(u8)]
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

impl From<u8> for RefProfileConfig {
    fn from(value: u8) -> Self {
        match value {
            0x0 => RefProfileConfig::RefEq,
            0x1 => RefProfileConfig::Linear,
            0x2 => RefProfileConfig::Staircase,
            0x3 => RefProfileConfig::ForwardReverse,
            _ => panic!("Invalid RefProfileConfig value"),
        }
    }
}
