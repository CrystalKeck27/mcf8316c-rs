pub use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RevDriveConfig {
    /// Open loop acceleration coefficient A1 during reverse drive
    pub rev_drv_open_loop_accel_a1: OpenLoopAccelerationA1,
    /// Open loop acceleration coefficient A2 during reverse drive
    pub rev_drv_open_loop_accel_a2: OpenLoopAccelerationA2,
    /// Bus current limit during active braking
    pub active_brake_current_limit: ActiveBrakeCurrentLimit,
    /// 10-bit value for active braking loop Kp. Kp = ACTIVE_BRAKE_KP / 2^7
    pub active_brake_kp: u16,
    /// 10-bit value for active braking loop Ki. Ki = ACTIVE_BRAKE_KI / 2^9
    pub active_brake_ki: u16,
}

impl Register for RevDriveConfig {
    const ADDRESS: u16 = REV_DRIVE_CONFIG; // Example address, replace with actual address
}

impl From<RevDriveConfig> for u32 {
    fn from(config: RevDriveConfig) -> Self {
        let mut value = 0;
        value |= (config.rev_drv_open_loop_accel_a1 as u32) << 27;
        value |= (config.rev_drv_open_loop_accel_a2 as u32) << 23;
        value |= (config.active_brake_current_limit as u32) << 20;
        value |= ((config.active_brake_kp & 0x3FF) as u32) << 10; // 10 bits
        value |= (config.active_brake_ki & 0x3FF) as u32; // 10 bits
        value
    }
}

impl From<u32> for RevDriveConfig {
    fn from(value: u32) -> Self {
        RevDriveConfig {
            rev_drv_open_loop_accel_a1: OpenLoopAccelerationA1::from(((value >> 27) & 0x07) as u8),
            rev_drv_open_loop_accel_a2: OpenLoopAccelerationA2::from(((value >> 23) & 0x0F) as u8),
            active_brake_current_limit: ActiveBrakeCurrentLimit::from(((value >> 20) & 0x07) as u8),
            active_brake_kp: ((value >> 10) & 0x3FF) as u16,
            active_brake_ki: (value & 0x3FF) as u16,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, strum::Display)]
#[repr(u8)]
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

impl From<u8> for ActiveBrakeCurrentLimit {
    fn from(value: u8) -> Self {
        match value {
            0x0 => ActiveBrakeCurrentLimit::A0_5,
            0x1 => ActiveBrakeCurrentLimit::A1,
            0x2 => ActiveBrakeCurrentLimit::A2,
            0x3 => ActiveBrakeCurrentLimit::A3,
            0x4 => ActiveBrakeCurrentLimit::A4,
            0x5 => ActiveBrakeCurrentLimit::A5,
            0x6 => ActiveBrakeCurrentLimit::A6,
            0x7 => ActiveBrakeCurrentLimit::A7,
            _ => panic!("Invalid ActiveBrakeCurrentLimit value"),
        }
    }
}
