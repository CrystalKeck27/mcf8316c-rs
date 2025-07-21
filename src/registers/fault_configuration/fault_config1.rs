use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FaultConfig1 {
    /// Current limit for Iq axis (torque) current reference in closed loop
    pub ilimit: CurrentSelection,
    /// Comparator based lock detection current threshold
    pub hw_lock_ilimit: CurrentSelection,
    /// ADC based lock detection current threshold
    pub lock_ilimit: CurrentSelection,
    /// Lock current limit mode
    pub lock_ilimit_mode: LockModeRaw,
    /// Lock detection current limit deglitch time
    pub lock_limit_deg: LockIlimitDeglitchTime,
    /// Lock detection retry time
    pub lck_retry: LockRetryTime,
    /// Motor Lock Mode
    pub mtr_lck_mode: LockModeRaw,
    /// IPD timeout fault enable
    /// 0 = Disable, 1 = Enable
    pub ipd_timeout_fault_en: bool,
    /// IPD frequency fault enable
    /// 0 = Disable, 1 = Enable
    pub ipd_freq_fault_en: bool,
    /// Enables indication of current loop and speed loop saturation
    /// 0 = Disable, 1 = Enable
    pub saturation_flags_en: bool,
}

impl Register for FaultConfig1 {
    const ADDRESS: u16 = FAULT_CONFIG1; // Example address, replace with actual address
}

impl From<FaultConfig1> for u32 {
    fn from(config: FaultConfig1) -> Self {
        let mut value = 0;
        value |= (config.ilimit as u32) << 27;
        value |= (config.hw_lock_ilimit as u32) << 23;
        value |= (config.lock_ilimit as u32) << 19;
        value |= (config.lock_ilimit_mode as u32) << 15;
        value |= (config.lock_limit_deg as u32) << 11;
        value |= (config.lck_retry as u32) << 7;
        value |= (config.mtr_lck_mode as u32) << 3;
        value |= (config.ipd_timeout_fault_en as u32) << 2;
        value |= (config.ipd_freq_fault_en as u32) << 1;
        value |= config.saturation_flags_en as u32;
        value
    }
}

impl From<u32> for FaultConfig1 {
    fn from(value: u32) -> Self {
        FaultConfig1 {
            ilimit: CurrentSelection::from(((value >> 27) & 0xF) as u8),
            hw_lock_ilimit: CurrentSelection::from(((value >> 23) & 0xF) as u8),
            lock_ilimit: CurrentSelection::from(((value >> 19) & 0xF) as u8),
            lock_ilimit_mode: LockModeRaw::from(((value >> 15) & 0x7) as u8),
            lock_limit_deg: LockIlimitDeglitchTime::from(((value >> 11) & 0xF) as u8),
            lck_retry: LockRetryTime::from(((value >> 7) & 0xF) as u8),
            mtr_lck_mode: LockModeRaw::from(((value >> 3) & 0x7) as u8),
            ipd_timeout_fault_en: ((value >> 2) & 0x1) != 0,
            ipd_freq_fault_en: ((value >> 1) & 0x1) != 0,
            saturation_flags_en: (value & 0x1) != 0,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, strum::Display)]
#[repr(u8)]
pub enum LockIlimitDeglitchTime {
    /// Not Applicable
    #[strum(to_string = "N/A")]
    NA = 0x0,
    /// Not Applicable
    #[strum(to_string = "N/A")]
    NA2 = 0x1,
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

impl From<u8> for LockIlimitDeglitchTime {
    fn from(value: u8) -> Self {
        match value {
            0x0 => LockIlimitDeglitchTime::NA,
            0x1 => LockIlimitDeglitchTime::NA2,
            0x2 => LockIlimitDeglitchTime::Ms0_2,
            0x3 => LockIlimitDeglitchTime::Ms0_5,
            0x4 => LockIlimitDeglitchTime::Ms1,
            0x5 => LockIlimitDeglitchTime::Ms2_5,
            0x6 => LockIlimitDeglitchTime::Ms5,
            0x7 => LockIlimitDeglitchTime::Ms7_5,
            0x8 => LockIlimitDeglitchTime::Ms10,
            0x9 => LockIlimitDeglitchTime::Ms25,
            0xA => LockIlimitDeglitchTime::Ms50,
            0xB => LockIlimitDeglitchTime::Ms75,
            0xC => LockIlimitDeglitchTime::Ms100,
            0xD => LockIlimitDeglitchTime::Ms200,
            0xE => LockIlimitDeglitchTime::Ms500,
            0xF => LockIlimitDeglitchTime::Ms1000,
            _ => panic!("Invalid value for LockIlimitDeglitchTime"),
        }
    }
}

impl PartialOrd for LockIlimitDeglitchTime {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        match (self, other) {
            (LockIlimitDeglitchTime::NA, _)
            | (_, LockIlimitDeglitchTime::NA)
            | (LockIlimitDeglitchTime::NA2, _)
            | (_, LockIlimitDeglitchTime::NA2) => None,
            _ => (*self as u8).partial_cmp(&(*other as u8)),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, strum::Display)]
#[repr(u8)]
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

impl From<u8> for LockRetryTime {
    fn from(value: u8) -> Self {
        match value {
            0x0 => LockRetryTime::S0_3,
            0x1 => LockRetryTime::S0_5,
            0x2 => LockRetryTime::S1,
            0x3 => LockRetryTime::S2,
            0x4 => LockRetryTime::S3,
            0x5 => LockRetryTime::S4,
            0x6 => LockRetryTime::S5,
            0x7 => LockRetryTime::S6,
            0x8 => LockRetryTime::S7,
            0x9 => LockRetryTime::S8,
            0xA => LockRetryTime::S9,
            0xB => LockRetryTime::S10,
            0xC => LockRetryTime::S11,
            0xD => LockRetryTime::S12,
            0xE => LockRetryTime::S13,
            0xF => LockRetryTime::S14,
            _ => panic!("Invalid value for LockRetryTime"),
        }
    }
}
