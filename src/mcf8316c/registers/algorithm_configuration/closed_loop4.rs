pub use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ClosedLoop4 {
    /// 7 LSB for speed loop Kp.
    pub spd_loop_kp: SpeedLoopKpLow7,
    /// 10-bit value for speed loop Ki. 0 = Auto
    pub spd_loop_ki: KVal<SpeedKi>,
    /// 14-bit value representing the maximum speed in Hz*6.
    /// For example: if MOTOR_SPEED is 0x2710, then maximum motor speed
    /// (Hz) = 10000(0x2710)/6 = 1666 Hz
    pub max_speed: u16,
}

impl Register for ClosedLoop4 {
    const ADDRESS: u16 = CLOSED_LOOP4; // Example address, replace with actual address
}

impl From<ClosedLoop4> for u32 {
    fn from(config: ClosedLoop4) -> Self {
        let mut value = 0;
        value |= (config.spd_loop_kp.0 as u32) << 24;
        value |= (Into::<u16>::into(config.spd_loop_ki) as u32) << 14;
        value |= (config.max_speed as u32) & 0x3FFF; // 14 bits
        value
    }
}

impl From<u32> for ClosedLoop4 {
    fn from(value: u32) -> Self {
        ClosedLoop4 {
            spd_loop_kp: SpeedLoopKpLow7(((value >> 24) & 0x7F) as u8),
            spd_loop_ki: KVal::try_from(((value >> 14) & 0x3FF) as u16).unwrap(),
            max_speed: ((value & 0x3FFF) as u16),
        }
    }
}
