use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RefProfiles1 {
    /// Configuration for reference profiles
    pub ref_profile_config: RefProfileConfig,
    /// Turn On Duty Cycle
    pub duty_on1: PercentAsU8,
    /// Turn Off Duty Cycle
    pub duty_off1: PercentAsU8,
    /// Duty Cycle for clamping Duty Input
    pub duty_clamp1: PercentAsU8,
    /// 5 MSB for Duty Cycle A
    pub duty_a: DutyAHigh5,
}

impl Register for RefProfiles1 {
    const ADDRESS: u16 = REF_PROFILES1; // Example address, replace with actual address
}

impl From<RefProfiles1> for u32 {
    fn from(config: RefProfiles1) -> Self {
        let mut value = 0;
        value |= (config.ref_profile_config as u32) << 29;
        value |= (config.duty_on1.inner as u32) << 21;
        value |= (config.duty_off1.inner as u32) << 13;
        value |= (config.duty_clamp1.inner as u32) << 5;
        value |= config.duty_a.0 as u32 & 0x1F; // 5 bits
        value
    }
}

impl From<u32> for RefProfiles1 {
    fn from(value: u32) -> Self {
        RefProfiles1 {
            ref_profile_config: RefProfileConfig::from(((value >> 29) & 0x3) as u8),
            duty_on1: PercentAsU8::new(((value >> 21) & 0xFF) as u8),
            duty_off1: PercentAsU8::new(((value >> 13) & 0xFF) as u8),
            duty_clamp1: PercentAsU8::new(((value >> 5) & 0xFF) as u8),
            duty_a: DutyAHigh5((value & 0x1F) as u8),
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
