use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PinConfig {
    /// Vdc (VM) filter disable.
    /// 0 = Vdc filter Enable, 1 = Vdc filter Disable
    pub vdc_filter_disable: bool,
    /// FG configuration during stop
    pub fg_idle_config: FgIdleConfig,
    /// FG configuration during fault
    pub fg_fault_config: FgFaultConfig,
    /// Alarm pin enable.
    /// 0 = Disable, 1 = Enable
    pub alarm_pin_en: bool,
    /// Brake pin mode.
    /// 0 = Low side brake, 1 = Align brake
    pub brake_pin_mode: bool,
    /// Align brake angle select.
    /// 0 = Use last commutation angle before entering align braking,
    /// 1 = Use ALIGN_ANGLE configuration for align braking
    pub align_brake_angle_sel: bool,
    /// Brake pin override
    pub brake_input: BrakeInput,
    /// Configure motor control input source
    pub speed_mode: SpeedMode,
}

impl Register for PinConfig {
    const ADDRESS: u16 = PIN_CONFIG; // Example address, replace with actual address
}

impl From<PinConfig> for u32 {
    fn from(config: PinConfig) -> Self {
        let mut value = 0;
        value |= (config.vdc_filter_disable as u32) << 27;
        value |= (config.fg_idle_config as u32) << 9;
        value |= (config.fg_fault_config as u32) << 7;
        value |= (config.alarm_pin_en as u32) << 6;
        value |= (config.brake_pin_mode as u32) << 5;
        value |= (config.align_brake_angle_sel as u32) << 4;
        value |= (config.brake_input as u32) << 2;
        value |= config.speed_mode as u32;
        value
    }
}

impl From<u32> for PinConfig {
    fn from(value: u32) -> Self {
        PinConfig {
            vdc_filter_disable: ((value >> 27) & 0x1) != 0,
            fg_idle_config: FgIdleConfig::from(((value >> 9) & 0x3) as u8),
            fg_fault_config: FgFaultConfig::from(((value >> 7) & 0x3) as u8),
            alarm_pin_en: ((value >> 6) & 0x1) != 0,
            brake_pin_mode: ((value >> 5) & 0x1) != 0,
            align_brake_angle_sel: ((value >> 4) & 0x1) != 0,
            brake_input: BrakeInput::from(((value >> 2) & 0x3) as u8),
            speed_mode: SpeedMode::from((value & 0x3) as u8),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, strum::Display)]
#[repr(u8)]
pub enum FgIdleConfig {
    /// FG state decided by FG_CONFIG
    #[strum(to_string = "FG state decided by FG_CONFIG")]
    FgConfig = 0x0,
    /// FG is pulled High
    #[strum(to_string = "FG is pulled High")]
    FgHigh = 0x1,
    /// FG is pulled Low
    #[strum(to_string = "FG is pulled Low")]
    FgLow = 0x2,
    /// FG is pulled High (Same as FgHigh)
    #[strum(to_string = "FG is pulled High")]
    FgHigh2 = 0x3,
}

impl From<u8> for FgIdleConfig {
    fn from(value: u8) -> Self {
        match value {
            0x0 => FgIdleConfig::FgConfig,
            0x1 => FgIdleConfig::FgHigh,
            0x2 => FgIdleConfig::FgLow,
            0x3 => FgIdleConfig::FgHigh2,
            _ => panic!("Invalid FgIdleConfig value"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, strum::Display)]
#[repr(u8)]
pub enum FgFaultConfig {
    /// Use last FG Signal when motor is driven before fault
    #[strum(to_string = "Use last FG Signal when motor is driven before fault")]
    FgLast = 0x0,
    /// FG is pulled High
    #[strum(to_string = "FG is pulled High")]
    FgHigh = 0x1,
    /// FG is pulled Low
    #[strum(to_string = "FG is pulled Low")]
    FgLow = 0x2,
    /// FG state decided by FG_CONFIG
    #[strum(to_string = "FG state decided by FG_CONFIG")]
    FgConfig = 0x3,
}

impl From<u8> for FgFaultConfig {
    fn from(value: u8) -> Self {
        match value {
            0x0 => FgFaultConfig::FgLast,
            0x1 => FgFaultConfig::FgHigh,
            0x2 => FgFaultConfig::FgLow,
            0x3 => FgFaultConfig::FgConfig,
            _ => panic!("Invalid FgFaultConfig value"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, strum::Display)]
#[repr(u8)]
pub enum BrakeInput {
    /// Hardware Pin BRAKE
    #[strum(to_string = "Hardware Pin BRAKE")]
    Pin = 0x0,
    /// Ovberride pin and brake/align according to BRAKE_PIN_MODE
    #[strum(to_string = "Override pin and brake/align according to BRAKE_PIN_MODE")]
    Override = 0x1,
    /// Ovberride pin and do not brake/align
    #[strum(to_string = "Override pin and do not brake/align")]
    OverrideNoBrake = 0x2,
    /// Hardware Pin BRAKE (Same as Pin)
    #[strum(to_string = "Hardware Pin BRAKE")]
    Pin2 = 0x3,
}

impl From<u8> for BrakeInput {
    fn from(value: u8) -> Self {
        match value {
            0x0 => BrakeInput::Pin,
            0x1 => BrakeInput::Override,
            0x2 => BrakeInput::OverrideNoBrake,
            0x3 => BrakeInput::Pin2,
            _ => panic!("Invalid BrakeInput value"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, strum::Display)]
#[repr(u8)]
pub enum SpeedMode {
    /// Controlled by analog voltage on SPEED pin
    #[strum(to_string = "Controlled by analog voltage on SPEED pin")]
    Analog = 0x0,
    /// Controlled by duty cycle (PWM) on SPEED pin
    #[strum(to_string = "Controlled by duty cycle (PWM) on SPEED pin")]
    Pwm = 0x1,
    /// Controlled by DIGITAL_SPEED_CTRL value (I2C)
    #[strum(to_string = "Controlled by DIGITAL_SPEED_CTRL value (I2C)")]
    I2C = 0x2,
    /// Controlled by frequency on SPEED pin
    #[strum(to_string = "Controlled by frequency on SPEED pin")]
    Frequency = 0x3,
}

impl From<u8> for SpeedMode {
    fn from(value: u8) -> Self {
        match value {
            0x0 => SpeedMode::Analog,
            0x1 => SpeedMode::Pwm,
            0x2 => SpeedMode::I2C,
            0x3 => SpeedMode::Frequency,
            _ => panic!("Invalid SpeedMode value"),
        }
    }
}
