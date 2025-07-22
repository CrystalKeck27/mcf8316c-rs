use super::*;
use bitbybit::*;

#[bitfield(u32, default = 0x0)]
#[derive(Debug, PartialEq, Eq)]
pub struct PinConfig {
    /// Vdc (VM) filter disable.
    /// 0 = Vdc filter Enable, 1 = Vdc filter Disable
    #[bit(27, rw)]
    pub vdc_filter_disable: bool,
    /// FG configuration during stop
    #[bits(9..=10, rw)]
    pub fg_idle_config: FgIdleConfig,
    /// FG configuration during fault
    #[bits(7..=8, rw)]
    pub fg_fault_config: FgFaultConfig,
    /// Alarm pin enable.
    /// 0 = Disable, 1 = Enable
    #[bit(6, rw)]
    pub alarm_pin_en: bool,
    /// Brake pin mode.
    /// 0 = Low side brake, 1 = Align brake
    #[bit(5, rw)]
    pub brake_pin_mode: bool,
    /// Align brake angle select.
    /// 0 = Use last commutation angle before entering align braking,
    /// 1 = Use ALIGN_ANGLE configuration for align braking
    #[bit(4, rw)]
    pub align_brake_angle_sel: bool,
    /// Brake pin override
    #[bits(2..=3, rw)]
    pub brake_input: BrakeInput,
    /// Configure motor control input source
    #[bits(0..=1, rw)]
    pub speed_mode: SpeedMode,
}

impl Register for PinConfig {
    const ADDRESS: u16 = PIN_CONFIG;

    fn value(&self) -> u32 {
        self.raw_value()
    }
}

#[bitenum(u2, exhaustive = true)]
#[derive(Debug, PartialEq, Eq, strum::Display)]
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

#[bitenum(u2, exhaustive = true)]
#[derive(Debug, PartialEq, Eq, strum::Display)]
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

#[bitenum(u2, exhaustive = true)]
#[derive(Debug, PartialEq, Eq, strum::Display)]
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

#[bitenum(u2, exhaustive = true)]
#[derive(Debug, PartialEq, Eq, strum::Display)]
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
