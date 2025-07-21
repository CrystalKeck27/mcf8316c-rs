use super::*;

const GD_CONFIG1_RESET: u32 = 0b_00010000_00100010_10000001_00000000;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GdConfig1 {
    /// Parity bit
    parity: bool,
    /// Slew rate
    pub slew_rate: SlewRate,
    /// Overvoltage level.
    /// 0 = VM overvoltage level is 34-V,
    /// 1 = VM overvoltage level is 22-V
    pub ovp_sel: bool,
    /// Overvoltage enable.
    /// 0 = Overvoltage protection is disabled,
    /// 1 = Overvoltage protection is enabled
    pub ovp_en: bool,
    /// Overtemperature warning enable.
    /// 0 = Over temperature reporting on nFAULT is disabled,
    /// 1 = Over temperature reporting on nFAULT is enabled
    pub otw_rep: bool,
    /// OCP Deglitch Time Settings
    pub ocp_deg: OvercurrentDeglitch,
    /// Overcurrent Level Setting
    /// 0 = OCP level is 16 A (Typical)
    /// 1 = OCP level is 24 A (Typical)
    pub ocp_lvl: bool,
    /// OCP Fault Mode
    pub ocp_mode: OvercurrentMode,
    /// Current Sense Amplifier gain (used only if DYNAMIC_CSA_GAIN_EN = 0b)
    pub csa_gain: CurrentSenseAmplifierGain,
}

impl Register for GdConfig1 {
    const ADDRESS: u16 = GD_CONFIG1; // Example address, replace with actual address
}

// TODO: Remove parity field and calculate it on the fly
impl From<GdConfig1> for u32 {
    fn from(config: GdConfig1) -> Self {
        let mut value = GD_CONFIG1_RESET;
        value |= (config.parity as u32) << 31;
        value |= (config.slew_rate as u32) << 26;
        value |= (config.ovp_sel as u32) << 19;
        value |= (config.ovp_en as u32) << 18;
        value |= (config.otw_rep as u32) << 16;
        value |= (config.ocp_deg as u32) << 12;
        value |= (config.ocp_lvl as u32) << 10;
        value |= (config.ocp_mode as u32) << 8;
        value |= config.csa_gain as u32;
        value
    }
}

impl From<u32> for GdConfig1 {
    fn from(value: u32) -> Self {
        GdConfig1 {
            parity: (value >> 31) & 0x1 != 0,
            slew_rate: SlewRate::from(((value >> 26) & 0x3) as u8),
            ovp_sel: (value >> 19) & 0x1 != 0,
            ovp_en: (value >> 18) & 0x1 != 0,
            otw_rep: (value >> 16) & 0x1 != 0,
            ocp_deg: OvercurrentDeglitch::from(((value >> 12) & 0x3) as u8),
            ocp_lvl: (value >> 10) & 0x1 != 0,
            ocp_mode: OvercurrentMode::from(((value >> 8) & 0x3) as u8),
            csa_gain: CurrentSenseAmplifierGain::from((value & 0x3) as u8),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, strum::Display)]
#[repr(u8)]
pub enum SlewRate {
    /// NotApplicable
    #[strum(to_string = "Not Applicable")]
    NotApplicable = 0x0,
    /// Not Applicable
    #[strum(to_string = "Not Applicable")]
    NotApplicable2 = 0x1,
    /// Slew rate is 125 V/μs
    #[strum(to_string = "125 V/μs")]
    V125 = 0x2,
    /// Slew rate is 200 V/μs
    #[strum(to_string = "200 V/μs")]
    V200 = 0x3,
}

impl From<u8> for SlewRate {
    fn from(value: u8) -> Self {
        match value {
            0x0 => SlewRate::NotApplicable,
            0x1 => SlewRate::NotApplicable2,
            0x2 => SlewRate::V125,
            0x3 => SlewRate::V200,
            _ => panic!("Invalid value for SlewRate: {}", value),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, strum::Display)]
#[repr(u8)]
pub enum OvercurrentDeglitch {
    /// 0.2 microseconds
    #[strum(to_string = "0.2 μs")]
    U0_2 = 0x0,
    /// 0.6 microseconds
    #[strum(to_string = "0.6 μs")]
    U0_6 = 0x1,
    /// 1.1 microseconds
    #[strum(to_string = "1.1 μs")]
    U1_1 = 0x2,
    /// 1.6 microseconds
    #[strum(to_string = "1.6 μs")]
    U1_6 = 0x3,
}

impl From<u8> for OvercurrentDeglitch {
    fn from(value: u8) -> Self {
        match value {
            0x0 => OvercurrentDeglitch::U0_2,
            0x1 => OvercurrentDeglitch::U0_6,
            0x2 => OvercurrentDeglitch::U1_1,
            0x3 => OvercurrentDeglitch::U1_6,
            _ => panic!("Invalid value for OvercurrentDeglitch: {}", value),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, strum::Display)]
#[repr(u8)]
pub enum OvercurrentMode {
    /// Overcurrnt causes a latched fault
    #[strum(to_string = "Latched Fault")]
    LatchedFault = 0x0,
    /// Overcurrent causes an automatic retrying fault after 500ms
    #[strum(to_string = "Retry")]
    Retry = 0x1,
    /// Not Applicable
    #[strum(to_string = "Not Applicable")]
    NotApplicable = 0x2,
    /// Not Applicable
    #[strum(to_string = "Not Applicable")]
    NotApplicable2 = 0x3,
}

impl From<u8> for OvercurrentMode {
    fn from(value: u8) -> Self {
        match value {
            0x0 => OvercurrentMode::LatchedFault,
            0x1 => OvercurrentMode::Retry,
            0x2 => OvercurrentMode::NotApplicable,
            0x3 => OvercurrentMode::NotApplicable2,
            _ => panic!("Invalid value for OvercurrentMode: {}", value),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, strum::Display)]
#[repr(u8)]
pub enum CurrentSenseAmplifierGain {
    /// CSA gain is 0.15 V/A
    #[strum(to_string = "0.15 V/A")]
    VA0_15 = 0x0,
    /// CSA gain is 0.3 V/A
    #[strum(to_string = "0.3 V/A")]
    VA0_3 = 0x1,
    /// CSA gain is 0.6 V/A
    #[strum(to_string = "0.6 V/A")]
    VA0_6 = 0x2,
    /// CSA gain is 1.2 V/A
    #[strum(to_string = "1.2 V/A")]
    VA1_2 = 0x3,
}

impl From<u8> for CurrentSenseAmplifierGain {
    fn from(value: u8) -> Self {
        match value {
            0x0 => CurrentSenseAmplifierGain::VA0_15,
            0x1 => CurrentSenseAmplifierGain::VA0_3,
            0x2 => CurrentSenseAmplifierGain::VA0_6,
            0x3 => CurrentSenseAmplifierGain::VA1_2,
            _ => panic!("Invalid value for CurrentSenseAmplifierGain: {}", value),
        }
    }
}
