//! Section 7.7.1.8

use super::*;
use arbitrary_int::*;
use bitbybit::*;

/// Register to configure close loop settings4
#[bitfield(u32, debug, default = 0x0)]
#[derive(PartialEq, Eq)]
pub struct ClosedLoop4 {
    /// 7 LSB for speed loop Kp.
    #[bits(24..=30, rw)]
    pub spd_loop_kp: SpeedLoopKpLow7,
    /// 10-bit value for speed loop Ki. 0 = Auto
    #[bits(14..=23, rw)]
    pub spd_loop_ki: SpeedKiVal,
    /// 14-bit value representing the maximum speed in Hz*6.
    /// For example: if MOTOR_SPEED is 0x2710, then maximum motor speed
    /// (Hz) = 10000(0x2710)/6 = 1666 Hz
    #[bits(0..=13, rw)]
    pub max_speed: u14,
}

impl Register for ClosedLoop4 {
    const ADDRESS: u12 = CLOSED_LOOP4;

    fn value(&self) -> u32 {
        self.raw_value()
    }

    fn from_value(value: u32) -> Self {
        Self::new_with_raw_value(value)
    }
}
