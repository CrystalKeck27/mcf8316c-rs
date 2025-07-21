use super::*;
use bitbybit::bitfield;

#[bitfield(u32)]
#[derive(Debug, PartialEq, Eq)]
pub struct RefProfiles2 {
    /// 3 LSB for Duty Cycle A
    #[bits(28..=30, rw)]
    pub duty_a: DutyALow3,
    /// Duty Cycle B
    #[bits(20..=27, rw)]
    pub duty_b: PercentAsU8,
    /// Duty Cycle C
    #[bits(12..=19, rw)]
    pub duty_c: PercentAsU8,
    /// Duty Cycle D
    #[bits(4..=11, rw)]
    pub duty_d: PercentAsU8,
    /// 4 MSB for Duty Cycle E
    #[bits(0..=3, rw)]
    pub duty_e: DutyEHigh4,
}

impl Register for RefProfiles2 {
    const ADDRESS: u16 = REF_PROFILES2; // Example address, replace with actual address
}
