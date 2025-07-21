use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RefProfiles2 {
    /// 3 LSB for Duty Cycle A
    pub duty_a: DutyALow3,
    /// Duty Cycle B
    pub duty_b: PercentAsU8,
    /// Duty Cycle C
    pub duty_c: PercentAsU8,
    /// Duty Cycle D
    pub duty_d: PercentAsU8,
    /// 4 MSB for Duty Cycle E
    pub duty_e: DutyEHigh4,
}

impl Register for RefProfiles2 {
    const ADDRESS: u16 = REF_PROFILES2; // Example address, replace with actual address
}

impl From<RefProfiles2> for u32 {
    fn from(config: RefProfiles2) -> Self {
        let mut value = 0;
        value |= (config.duty_a.0 as u32) << 28; // 3 bits
        value |= (config.duty_b.inner as u32) << 20;
        value |= (config.duty_c.inner as u32) << 12;
        value |= (config.duty_d.inner as u32) << 4;
        value |= (config.duty_e.0 as u32) & 0x0F; // 4 bits
        value
    }
}

impl From<u32> for RefProfiles2 {
    fn from(value: u32) -> Self {
        RefProfiles2 {
            duty_a: DutyALow3(((value >> 28) & 0x07) as u8),
            duty_b: PercentAsU8::new(((value >> 20) & 0xFF) as u8),
            duty_c: PercentAsU8::new(((value >> 12) & 0xFF) as u8),
            duty_d: PercentAsU8::new(((value >> 4) & 0xFF) as u8),
            duty_e: DutyEHigh4((value & 0x0F) as u8),
        }
    }
}
