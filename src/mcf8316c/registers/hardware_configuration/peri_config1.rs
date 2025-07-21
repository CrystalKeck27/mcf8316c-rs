pub use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PeriConfig1 {
    /// Spread Spectrum Modulation disable.
    /// 0 = SSM is Enabled, 1 = SSM is Disabled
    pub spread_spectrum_modulation_disable: bool,
    /// Bus current limit.
    pub bus_current_limit: CurrentSelection,
    /// Bus current limit enable.
    /// 0 = Disable, 1 = Enable
    pub bus_current_limit_en: bool,
    /// DIR pin override
    pub dir_input: DirectionPinOverride,
    /// Response to change of DIR pin status.
    /// 0 = Follow motor stop options and ISD routine on detecting DIR change,
    /// 1 = Change the direction through Reverse Drive while continuously driving the motor
    pub dir_change_mode: bool,
    /// Enables self-test on power up.
    /// 0 = STL is disabled, 1 = STL is enabled
    pub self_test_enable: bool,
    /// Difference between final speed and present speed below which
    /// active braking will be applied.
    pub active_brake_speed_delta_limit_entry: BrakeDeltaLimit,
    /// Modulation Index limit below which active braking will be applied
    pub active_brake_mod_index_limit: ModulationIndexLimit,
    /// Frequency range selection for PWM/duty based motor control input
    /// 0 = 325Hz to 100kHz, 1 = 10Hz to 325Hz
    pub speed_range_sel: bool
}

impl Register for PeriConfig1 {
    const ADDRESS: u16 = PERI_CONFIG1; // Example address, replace with actual address
}

impl From<PeriConfig1> for u32 {
    fn from(config: PeriConfig1) -> Self {
        let mut value = 0;
        value |= (config.spread_spectrum_modulation_disable as u32) << 30;
        value |= (config.bus_current_limit as u32) << 26;
        value |= (config.bus_current_limit_en as u32) << 22;
        value |= (config.dir_input as u32) << 21;
        value |= (config.dir_change_mode as u32) << 19;
        value |= (config.self_test_enable as u32) << 17;
        value |= (config.active_brake_speed_delta_limit_entry as u32) << 13;
        value |= (config.active_brake_mod_index_limit as u32) << 10;
        value |= (config.speed_range_sel as u32) << 9;
        value
    }
}

impl From<u32> for PeriConfig1 {
    fn from(value: u32) -> Self {
        PeriConfig1 {
            spread_spectrum_modulation_disable: (value >> 30) & 0x1 != 0,
            bus_current_limit: CurrentSelection::from(((value >> 26) & 0xF) as u8),
            bus_current_limit_en: (value >> 22) & 0x1 != 0,
            dir_input: DirectionPinOverride::from(((value >> 21) & 0x3) as u8),
            dir_change_mode: (value >> 19) & 0x1 != 0,
            self_test_enable: (value >> 17) & 0x1 != 0,
            active_brake_speed_delta_limit_entry: BrakeDeltaLimit::from(((value >> 13) & 0xF) as u8),
            active_brake_mod_index_limit: ModulationIndexLimit::from(((value >> 10) & 0x7) as u8),
            speed_range_sel: (value >> 9) & 0x1 != 0,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, strum::Display)]
#[repr(u8)]
pub enum DirectionPinOverride {
    /// Hardware Pin DIR
    #[strum(to_string = "Hardware Pin DIR")]
    Hardware = 0x0,
    /// Override DIR pin with clockwise rotation OUTA-OUTB-OUTC
    #[strum(to_string = "Override DIR pin with clockwise rotation OUTA-OUTB-OUTC")]
    OverrideClockwise = 0x1,
    /// Override DIR pin with counter-clockwise rotation OUTA-OUTC-OUTB
    #[strum(to_string = "Override DIR pin with counter-clockwise rotation OUTA-OUTC-OUTB")]
    OverrideCounterClockwise = 0x2,
    /// Hardware Pin DIR (Same as Hardware)
    #[strum(to_string = "Hardware Pin DIR")]
    Hardware2 = 0x3,
}

impl From<u8> for DirectionPinOverride {
    fn from(value: u8) -> Self {
        match value {
            0x0 => DirectionPinOverride::Hardware,
            0x1 => DirectionPinOverride::OverrideClockwise,
            0x2 => DirectionPinOverride::OverrideCounterClockwise,
            0x3 => DirectionPinOverride::Hardware2,
            _ => panic!("Invalid value for DirectionPinOverride: {}", value),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, strum::Display)]
#[repr(u8)]
pub enum BrakeDeltaLimit {
    /// Not Applicable
    #[strum(to_string = "Not Applicable")]
    NotApplicable = 0x0,
    /// 5%
    #[strum(to_string = "5%")]
    P5 = 0x1,
    /// 10%
    #[strum(to_string = "10%")]
    P10 = 0x2,
    /// 15%
    #[strum(to_string = "15%")]
    P15 = 0x3,
    /// 20%
    #[strum(to_string = "20%")]
    P20 = 0x4,
    /// 25%
    #[strum(to_string = "25%")]
    P25 = 0x5,
    /// 30%
    #[strum(to_string = "30%")]
    P30 = 0x6,
    /// 35%
    #[strum(to_string = "35%")]
    P35 = 0x7,
    /// 40%
    #[strum(to_string = "40%")]
    P40 = 0x8,
    /// 45%
    #[strum(to_string = "45%")]
    P45 = 0x9,
    /// 50%
    #[strum(to_string = "50%")]
    P50 = 0xA,
    /// 60%
    #[strum(to_string = "60%")]
    P60 = 0xB,
    /// 70%
    #[strum(to_string = "70%")]
    P70 = 0xC,
    /// 80%
    #[strum(to_string = "80%")]
    P80 = 0xD,
    /// 90%
    #[strum(to_string = "90%")]
    P90 = 0xE,
    /// 100%
    #[strum(to_string = "100%")]
    P100 = 0xF,
}

impl From<u8> for BrakeDeltaLimit {
    fn from(value: u8) -> Self {
        match value {
            0x0 => BrakeDeltaLimit::NotApplicable,
            0x1 => BrakeDeltaLimit::P5,
            0x2 => BrakeDeltaLimit::P10,
            0x3 => BrakeDeltaLimit::P15,
            0x4 => BrakeDeltaLimit::P20,
            0x5 => BrakeDeltaLimit::P25,
            0x6 => BrakeDeltaLimit::P30,
            0x7 => BrakeDeltaLimit::P35,
            0x8 => BrakeDeltaLimit::P40,
            0x9 => BrakeDeltaLimit::P45,
            0xA => BrakeDeltaLimit::P50,
            0xB => BrakeDeltaLimit::P60,
            0xC => BrakeDeltaLimit::P70,
            0xD => BrakeDeltaLimit::P80,
            0xE => BrakeDeltaLimit::P90,
            0xF => BrakeDeltaLimit::P100,
            _ => panic!("Invalid value for BrakeDeltaLimit: {}", value),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, strum::Display)]
#[repr(u8)]
pub enum ModulationIndexLimit {
    /// 0%
    #[strum(to_string = "0%")]
    P0 = 0x0,
    /// 40%
    #[strum(to_string = "40%")]
    P40 = 0x1,
    /// 50%
    #[strum(to_string = "50%")]
    P50 = 0x2,
    /// 60%
    #[strum(to_string = "60%")]
    P60 = 0x3,
    /// 70%
    #[strum(to_string = "70%")]
    P70 = 0x4,
    /// 80%
    #[strum(to_string = "80%")]
    P80 = 0x5,
    /// 90%
    #[strum(to_string = "90%")]
    P90 = 0x6,
    /// 100%
    #[strum(to_string = "100%")]
    P100 = 0x7,
}

impl From<u8> for ModulationIndexLimit {
    fn from(value: u8) -> Self {
        match value {
            0x0 => ModulationIndexLimit::P0,
            0x1 => ModulationIndexLimit::P40,
            0x2 => ModulationIndexLimit::P50,
            0x3 => ModulationIndexLimit::P60,
            0x4 => ModulationIndexLimit::P70,
            0x5 => ModulationIndexLimit::P80,
            0x6 => ModulationIndexLimit::P90,
            0x7 => ModulationIndexLimit::P100,
            _ => panic!("Invalid value for ModulationIndexLimit: {}", value),
        }
    }
}
