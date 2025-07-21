use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RefProfiles3 {
    /// 4 LSB for Duty Cycle E
    pub duty_e: DutyELow4,
    /// Turn On Duty Cycle
    pub duty_on2: PercentAsU8,
    /// Turn Off Duty Cycle
    pub duty_off2: PercentAsU8,
    /// Duty Cycle for clamping Duty Input
    pub duty_clamp2: PercentAsU8,
    /// Duty Hysteresis
    pub duty_hys: DutyHysteresis,
}

impl Register for RefProfiles3 {
    const ADDRESS: u16 = REF_PROFILES3; // Example address, replace with actual address
}

impl From<RefProfiles3> for u32 {
    fn from(config: RefProfiles3) -> Self {
        let mut value = 0;
        value |= (config.duty_e.0 as u32) << 27; // 4 bits
        value |= (config.duty_on2.inner as u32) << 19;
        value |= (config.duty_off2.inner as u32) << 11;
        value |= (config.duty_clamp2.inner as u32) << 3;
        value |= (config.duty_hys as u32) & 0x0F; // 2 bits
        value
    }
}

impl From<u32> for RefProfiles3 {
    fn from(value: u32) -> Self {
        RefProfiles3 {
            duty_e: DutyELow4(((value >> 27) & 0x0F) as u8),
            duty_on2: PercentAsU8::new(((value >> 19) & 0xFF) as u8),
            duty_off2: PercentAsU8::new(((value >> 11) & 0xFF) as u8),
            duty_clamp2: PercentAsU8::new(((value >> 3) & 0xFF) as u8),
            duty_hys: DutyHysteresis::from((value & 0x03) as u8),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, strum::Display)]
#[repr(u8)]
pub enum DutyHysteresis {
    /// 0% Hysteresis
    #[strum(to_string = "0%")]
    P0 = 0x0,
    /// 0.5% Hysteresis
    #[strum(to_string = "0.5%")]
    P0_5 = 0x1,
    /// 1% Hysteresis
    #[strum(to_string = "1%")]
    P1 = 0x2,
    /// 2% Hysteresis
    #[strum(to_string = "2%")]
    P2 = 0x3,
}

impl From<u8> for DutyHysteresis {
    fn from(value: u8) -> Self {
        match value {
            0x0 => DutyHysteresis::P0,
            0x1 => DutyHysteresis::P0_5,
            0x2 => DutyHysteresis::P1,
            0x3 => DutyHysteresis::P2,
            _ => panic!("Invalid DutyHysteresis value"),
        }
    }
}
