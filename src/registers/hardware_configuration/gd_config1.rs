use super::*;
use bitbybit::*;

const GD_CONFIG1_RESET: u32 = 0b_00010000_00100010_10000001_00000000;

#[bitfield(u32)]
#[derive(Debug, PartialEq, Eq)]
pub struct GdConfig1 {
    /// Parity bit
    #[bit(31, rw)]
    parity: bool,
    /// Slew rate
    #[bits(26..=27, rw)]
    pub slew_rate: Option<SlewRate>,
    /// Overvoltage level.
    /// 0 = VM overvoltage level is 34-V,
    /// 1 = VM overvoltage level is 22-V
    #[bit(19, rw)]
    pub ovp_sel: bool,
    /// Overvoltage enable.
    /// 0 = Overvoltage protection is disabled,
    /// 1 = Overvoltage protection is enabled
    #[bit(18, rw)]
    pub ovp_en: bool,
    /// Overtemperature warning enable.
    /// 0 = Over temperature reporting on nFAULT is disabled,
    /// 1 = Over temperature reporting on nFAULT is enabled
    #[bit(16, rw)]
    pub otw_rep: bool,
    /// OCP Deglitch Time Settings
    #[bits(12..=13, rw)]
    pub ocp_deg: OvercurrentDeglitch,
    /// Overcurrent Level Setting
    /// 0 = OCP level is 16 A (Typical)
    /// 1 = OCP level is 24 A (Typical)
    #[bit(10, rw)]
    pub ocp_lvl: bool,
    /// OCP Fault Mode
    #[bits(8..=9, rw)]
    pub ocp_mode: Option<OvercurrentMode>,
    /// Current Sense Amplifier gain (used only if DYNAMIC_CSA_GAIN_EN = 0b)
    #[bits(0..=1, rw)]
    pub csa_gain: CurrentSenseAmplifierGain,
}

impl Register for GdConfig1 {
    const ADDRESS: u16 = GD_CONFIG1; // Example address, replace with actual address
}

#[bitenum(u2, exhaustive = false)]
#[derive(Debug, PartialEq, Eq, strum::Display)]
pub enum SlewRate {
    /// Slew rate is 125 V/μs
    #[strum(to_string = "125 V/μs")]
    V125 = 0x2,
    /// Slew rate is 200 V/μs
    #[strum(to_string = "200 V/μs")]
    V200 = 0x3,
}

#[bitenum(u2, exhaustive = true)]
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, strum::Display)]
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

#[bitenum(u2, exhaustive = false)]
#[derive(Debug, PartialEq, Eq, strum::Display)]
#[repr(u8)]
pub enum OvercurrentMode {
    /// Overcurrnt causes a latched fault
    #[strum(to_string = "Latched Fault")]
    LatchedFault = 0x0,
    /// Overcurrent causes an automatic retrying fault after 500ms
    #[strum(to_string = "Retry")]
    Retry = 0x1,
}

#[bitenum(u2, exhaustive = true)]
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, strum::Display)]
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
