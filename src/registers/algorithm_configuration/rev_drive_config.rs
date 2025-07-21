pub use super::*;
use arbitrary_int::*;
use bitbybit::*;

#[bitfield(u32)]
#[derive(Debug, PartialEq, Eq)]
pub struct RevDriveConfig {
    /// Open loop acceleration coefficient A1 during reverse drive
    #[bits(27..=30, rw)]
    pub rev_drv_open_loop_accel_a1: OpenLoopAccelerationA1,
    /// Open loop acceleration coefficient A2 during reverse drive
    #[bits(23..=26, rw)]
    pub rev_drv_open_loop_accel_a2: OpenLoopAccelerationA2,
    /// Bus current limit during active braking
    #[bits(20..=22, rw)]
    pub active_brake_current_limit: ActiveBrakeCurrentLimit,
    /// 10-bit value for active braking loop Kp. Kp = ACTIVE_BRAKE_KP / 2^7
    #[bits(10..=19, rw)]
    pub active_brake_kp: u10,
    /// 10-bit value for active braking loop Ki. Ki = ACTIVE_BRAKE_KI / 2^9
    #[bits(0..=9, rw)]
    pub active_brake_ki: u10,
}

impl Register for RevDriveConfig {
    const ADDRESS: u16 = REV_DRIVE_CONFIG; // Example address, replace with actual address
}

#[bitenum(u3, exhaustive = true)]
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, strum::Display)]
pub enum ActiveBrakeCurrentLimit {
    /// 0.5 A
    #[strum(to_string = "0.5 A")]
    A0_5 = 0x0,
    /// 1.0 A
    #[strum(to_string = "1.0 A")]
    A1 = 0x1,
    /// 2.0 A
    #[strum(to_string = "2.0 A")]
    A2 = 0x2,
    /// 3.0 A
    #[strum(to_string = "3.0 A")]
    A3 = 0x3,
    /// 4.0 A
    #[strum(to_string = "4.0 A")]
    A4 = 0x4,
    /// 5.0 A
    #[strum(to_string = "5.0 A")]
    A5 = 0x5,
    /// 6.0 A
    #[strum(to_string = "6.0 A")]
    A6 = 0x6,
    /// 7.0 A
    #[strum(to_string = "7.0 A")]
    A7 = 0x7,
}
