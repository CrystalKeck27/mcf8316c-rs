use super::*;
use bitbybit::*;

#[bitfield(u32, default = 0x0)]
#[derive(Debug, PartialEq, Eq)]
pub struct FaultConfig1 {
    /// Current limit for Iq axis (torque) current reference in closed loop
    #[bits(27..=30, rw)]
    pub ilimit: CurrentSelection,
    /// Comparator based lock detection current threshold
    #[bits(23..=26, rw)]
    pub hw_lock_ilimit: CurrentSelection,
    /// ADC based lock detection current threshold
    #[bits(19..=22, rw)]
    pub lock_ilimit: CurrentSelection,
    /// Lock current limit mode
    #[bits(15..=18, rw)]
    pub lock_ilimit_mode: LockModeRaw,
    /// Lock detection current limit deglitch time
    #[bits(11..=14, rw)]
    pub lock_limit_deg: Option<LockIlimitDeglitchTime>,
    /// Lock detection retry time
    #[bits(7..=10, rw)]
    pub lck_retry: LockRetryTime,
    /// Motor Lock Mode
    #[bits(3..=6, rw)]
    pub mtr_lck_mode: LockModeRaw,
    /// IPD timeout fault enable
    /// 0 = Disable, 1 = Enable
    #[bit(2, rw)]
    pub ipd_timeout_fault_en: bool,
    /// IPD frequency fault enable
    /// 0 = Disable, 1 = Enable
    #[bit(1, rw)]
    pub ipd_freq_fault_en: bool,
    /// Enables indication of current loop and speed loop saturation
    /// 0 = Disable, 1 = Enable
    #[bit(0, rw)]
    pub saturation_flags_en: bool,
}

impl Register for FaultConfig1 {
    const ADDRESS: u16 = FAULT_CONFIG1;

    fn value(&self) -> u32 {
        self.raw_value()
    }
}

#[bitenum(u4, exhaustive = false)]
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, strum::Display)]
pub enum LockIlimitDeglitchTime {
    /// 0.2 ms
    #[strum(to_string = "0.2 ms")]
    Ms0_2 = 0x2,
    /// 0.5 ms
    #[strum(to_string = "0.5 ms")]
    Ms0_5 = 0x3,
    /// 1 ms
    #[strum(to_string = "1 ms")]
    Ms1 = 0x4,
    /// 2.5 ms
    #[strum(to_string = "2.5 ms")]
    Ms2_5 = 0x5,
    /// 5 ms
    #[strum(to_string = "5 ms")]
    Ms5 = 0x6,
    /// 7.5 ms
    #[strum(to_string = "7.5 ms")]
    Ms7_5 = 0x7,
    /// 10 ms
    #[strum(to_string = "10 ms")]
    Ms10 = 0x8,
    /// 25 ms
    #[strum(to_string = "25 ms")]
    Ms25 = 0x9,
    /// 50 ms
    #[strum(to_string = "50 ms")]
    Ms50 = 0xA,
    /// 75 ms
    #[strum(to_string = "75 ms")]
    Ms75 = 0xB,
    /// 100 ms
    #[strum(to_string = "100 ms")]
    Ms100 = 0xC,
    /// 200 ms
    #[strum(to_string = "200 ms")]
    Ms200 = 0xD,
    /// 500 ms
    #[strum(to_string = "500 ms")]
    Ms500 = 0xE,
    /// 1000 ms
    #[strum(to_string = "1000 ms")]
    Ms1000 = 0xF,
}

#[bitenum(u4, exhaustive = true)]
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, strum::Display)]
pub enum LockRetryTime {
    /// 0.3 s
    #[strum(to_string = "0.3 s")]
    S0_3 = 0x0,
    /// 0.5 s
    #[strum(to_string = "0.5 s")]
    S0_5 = 0x1,
    /// 1 s
    #[strum(to_string = "1 s")]
    S1 = 0x2,
    /// 2 s
    #[strum(to_string = "2 s")]
    S2 = 0x3,
    /// 3 s
    #[strum(to_string = "3 s")]
    S3 = 0x4,
    /// 4 s
    #[strum(to_string = "4 s")]
    S4 = 0x5,
    /// 5 s
    #[strum(to_string = "5 s")]
    S5 = 0x6,
    /// 6 s
    #[strum(to_string = "6 s")]
    S6 = 0x7,
    /// 7 s
    #[strum(to_string = "7 s")]
    S7 = 0x8,
    /// 8 s
    #[strum(to_string = "8 s")]
    S8 = 0x9,
    /// 9 s
    #[strum(to_string = "9 s")]
    S9 = 0xA,
    /// 10 s
    #[strum(to_string = "10 s")]
    S10 = 0xB,
    /// 11 s
    #[strum(to_string = "11 s")]
    S11 = 0xC,
    /// 12 s
    #[strum(to_string = "12 s")]
    S12 = 0xD,
    /// 13 s
    #[strum(to_string = "13 s")]
    S13 = 0xE,
    /// 14 s
    #[strum(to_string = "14 s")]
    S14 = 0xF,
}
