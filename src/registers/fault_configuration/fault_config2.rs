use core::cmp::Ordering;

use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FaultConfig2 {
    /// Lock 1 (Abnormal Speed) Enable.
    /// 0 = Disable, 1 = Enable
    pub lock1_en: bool,
    /// Lock 2 (Abnormal BEMF) Enable.
    /// 0 = Disable, 1 = Enable
    pub lock2_en: bool,
    /// Lock 3 (No Motor) Enable.
    /// 0 = Disable, 1 = Enable
    pub lock3_en: bool,
    /// Abnormal speed lock threshold (% of MAX_SPEED)
    pub lock_abn_speed: AbnormalSpeedLockThreshold,
    /// Abnormal BEMF lock threshold (% of expected BEMF)
    pub abnormal_bemf_thr: AbnormalBemfThreshold,
    /// No motor lock threshold
    pub no_mtr_thr: NoMotorThreshold,
    /// Hardware lock detection current mode
    pub hw_lock_ilimit_mode: LockModeRaw,
    /// Hardware lock detection current limit deglitch time
    pub hw_lock_ilimit_deg: LockIlimitDeglitchTime,
    /// Minimum DC Bus voltage for running motor
    pub min_vm_motor: MinimumBusVoltage,
    /// DC Bus Undervoltage Fault Recovery Mode.
    /// 0 = Latch on undervoltage,
    /// 1 = Automatic clear if voltage in bounds
    pub min_vm_mode: bool,
    /// Maximum DC Bus voltage for running motor
    pub max_vm_motor: MaximumBusVoltage,
    /// DC Bus Overvoltage Fault Recovery Mode.
    /// 0 = Latch on overvoltage,
    /// 1 = Automatic clear if voltage in bounds
    pub max_vm_mode: bool,
    /// Automatic retry attempts
    pub auto_retry_times: AutoRetryTimes,
}

impl Register for FaultConfig2 {
    const ADDRESS: u16 = FAULT_CONFIG2; // Example address, replace with actual address
}

impl From<FaultConfig2> for u32 {
    fn from(config: FaultConfig2) -> Self {
        let mut value = 0;
        value |= (config.lock1_en as u32) << 30;
        value |= (config.lock2_en as u32) << 29;
        value |= (config.lock3_en as u32) << 28;
        value |= (config.lock_abn_speed as u32) << 25;
        value |= (config.abnormal_bemf_thr as u32) << 22;
        value |= (config.no_mtr_thr as u32) << 19;
        value |= (config.hw_lock_ilimit_mode as u32) << 15;
        value |= (config.hw_lock_ilimit_deg as u32) << 12;
        value |= (config.min_vm_motor as u32) << 8;
        value |= (config.min_vm_mode as u32) << 7;
        value |= (config.max_vm_motor as u32) << 4;
        value |= (config.max_vm_mode as u32) << 3;
        value |= config.auto_retry_times as u32;
        value
    }
}

impl From<u32> for FaultConfig2 {
    fn from(value: u32) -> Self {
        FaultConfig2 {
            lock1_en: ((value >> 30) & 0x1) != 0,
            lock2_en: ((value >> 29) & 0x1) != 0,
            lock3_en: ((value >> 28) & 0x1) != 0,
            lock_abn_speed: AbnormalSpeedLockThreshold::from(((value >> 25) & 0x7) as u8),
            abnormal_bemf_thr: AbnormalBemfThreshold::from(((value >> 22) & 0x7) as u8),
            no_mtr_thr: NoMotorThreshold::from(((value >> 19) & 0x7) as u8),
            hw_lock_ilimit_mode: LockModeRaw::from(((value >> 15) & 0x7) as u8),
            hw_lock_ilimit_deg: LockIlimitDeglitchTime::from(((value >> 12) & 0x7) as u8),
            min_vm_motor: MinimumBusVoltage::from(((value >> 8) & 0x7) as u8),
            min_vm_mode: ((value >> 7) & 0x1) != 0,
            max_vm_motor: MaximumBusVoltage::from(((value >> 4) & 0x7) as u8),
            max_vm_mode: ((value >> 3) & 0x1) != 0,
            auto_retry_times: AutoRetryTimes::from((value & 0x3) as u8),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, strum::Display)]
#[repr(u8)]
pub enum AbnormalSpeedLockThreshold {
    /// 130% of MAX_SPEED
    #[strum(to_string = "130%")]
    P130 = 0x0,
    /// 140% of MAX_SPEED
    #[strum(to_string = "140%")]
    P140 = 0x1,
    /// 150% of MAX_SPEED
    #[strum(to_string = "150%")]
    P150 = 0x2,
    /// 160% of MAX_SPEED
    #[strum(to_string = "160%")]
    P160 = 0x3,
    /// 170% of MAX_SPEED
    #[strum(to_string = "170%")]
    P170 = 0x4,
    /// 180% of MAX_SPEED
    #[strum(to_string = "180%")]
    P180 = 0x5,
    /// 190% of MAX_SPEED
    #[strum(to_string = "190%")]
    P190 = 0x6,
    /// 200% of MAX_SPEED
    #[strum(to_string = "200%")]
    P200 = 0x7,
}

impl From<u8> for AbnormalSpeedLockThreshold {
    fn from(value: u8) -> Self {
        match value {
            0x0 => AbnormalSpeedLockThreshold::P130,
            0x1 => AbnormalSpeedLockThreshold::P140,
            0x2 => AbnormalSpeedLockThreshold::P150,
            0x3 => AbnormalSpeedLockThreshold::P160,
            0x4 => AbnormalSpeedLockThreshold::P170,
            0x5 => AbnormalSpeedLockThreshold::P180,
            0x6 => AbnormalSpeedLockThreshold::P190,
            0x7 => AbnormalSpeedLockThreshold::P200,
            _ => panic!("Invalid value for AbnormalSpeedLockThreshold"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, strum::Display)]
#[repr(u8)]
pub enum AbnormalBemfThreshold {
    /// 40% of expected BEMF
    #[strum(to_string = "40%")]
    P40 = 0x0,
    /// 45% of expected BEMF
    #[strum(to_string = "45%")]
    P45 = 0x1,
    /// 50% of expected BEMF
    #[strum(to_string = "50%")]
    P50 = 0x2,
    /// 55% of expected BEMF
    #[strum(to_string = "55%")]
    P55 = 0x3,
    /// 60% of expected BEMF
    #[strum(to_string = "60%")]
    P60 = 0x4,
    /// 65% of expected BEMF
    #[strum(to_string = "65%")]
    P65 = 0x5,
    /// 67.5% of expected BEMF
    #[strum(to_string = "67.5%")]
    P67_5 = 0x6,
    /// 70% of expected BEMF
    #[strum(to_string = "70%")]
    P70 = 0x7,
}

impl From<u8> for AbnormalBemfThreshold {
    fn from(value: u8) -> Self {
        match value {
            0x0 => AbnormalBemfThreshold::P40,
            0x1 => AbnormalBemfThreshold::P45,
            0x2 => AbnormalBemfThreshold::P50,
            0x3 => AbnormalBemfThreshold::P55,
            0x4 => AbnormalBemfThreshold::P60,
            0x5 => AbnormalBemfThreshold::P65,
            0x6 => AbnormalBemfThreshold::P67_5,
            0x7 => AbnormalBemfThreshold::P70,
            _ => panic!("Invalid value for AbnormalBemfThreshold"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, strum::Display)]
#[repr(u8)]
pub enum NoMotorThreshold {
    /// 0.075A
    #[strum(to_string = "0.075A")]
    P0_075 = 0x0,
    /// 0.075A (Duplicate)
    #[strum(to_string = "0.075A")]
    P0_075Duplicate = 0x1,
    /// 0.1A
    #[strum(to_string = "0.1A")]
    P0_1 = 0x2,
    /// 0.125A
    #[strum(to_string = "0.125A")]
    P0_125 = 0x3,
    /// 0.25A
    #[strum(to_string = "0.25A")]
    P0_25 = 0x4,
    /// 0.5A
    #[strum(to_string = "0.5A")]
    P0_5 = 0x5,
    /// 0.75A
    #[strum(to_string = "0.75A")]
    P0_75 = 0x6,
    /// 1.0A
    #[strum(to_string = "1.0A")]
    P1_0 = 0x7,
}

impl From<u8> for NoMotorThreshold {
    fn from(value: u8) -> Self {
        match value {
            0x0 => NoMotorThreshold::P0_075,
            0x1 => NoMotorThreshold::P0_075Duplicate,
            0x2 => NoMotorThreshold::P0_1,
            0x3 => NoMotorThreshold::P0_125,
            0x4 => NoMotorThreshold::P0_25,
            0x5 => NoMotorThreshold::P0_5,
            0x6 => NoMotorThreshold::P0_75,
            0x7 => NoMotorThreshold::P1_0,
            _ => panic!("Invalid value for NoMotorThreshold"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, strum::Display)]
#[repr(u8)]
pub enum LockIlimitDeglitchTime {
    /// No deglitch
    #[strum(to_string = "0µs (No Deglitch)")]
    NoDeglitch = 0x0,
    /// 1µs
    #[strum(to_string = "1µs")]
    Us1 = 0x1,
    /// 2µs
    #[strum(to_string = "2µs")]
    Us2 = 0x2,
    /// 3µs
    #[strum(to_string = "3µs")]
    Us3 = 0x3,
    /// 4µs
    #[strum(to_string = "4µs")]
    Us4 = 0x4,
    /// 5µs
    #[strum(to_string = "5µs")]
    Us5 = 0x5,
    /// 6µs
    #[strum(to_string = "6µs")]
    Us6 = 0x6,
    /// 7µs
    #[strum(to_string = "7µs")]
    Us7 = 0x7,
}

impl From<u8> for LockIlimitDeglitchTime {
    fn from(value: u8) -> Self {
        match value {
            0x0 => LockIlimitDeglitchTime::NoDeglitch,
            0x1 => LockIlimitDeglitchTime::Us1,
            0x2 => LockIlimitDeglitchTime::Us2,
            0x3 => LockIlimitDeglitchTime::Us3,
            0x4 => LockIlimitDeglitchTime::Us4,
            0x5 => LockIlimitDeglitchTime::Us5,
            0x6 => LockIlimitDeglitchTime::Us6,
            0x7 => LockIlimitDeglitchTime::Us7,
            _ => panic!("Invalid value for LockIlimitDeglitchTime"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, strum::Display)]
#[repr(u8)]
pub enum MinimumBusVoltage {
    /// No limit
    #[strum(to_string = "No Limit")]
    NoLimit = 0x0,
    /// 4.5V
    #[strum(to_string = "4.5V")]
    V4_5 = 0x1,
    /// 5V
    #[strum(to_string = "5V")]
    V5 = 0x2,
    /// 5.5V
    #[strum(to_string = "5.5V")]
    V5_5 = 0x3,
    /// 6V
    #[strum(to_string = "6V")]
    V6 = 0x4,
    /// 7.5V
    #[strum(to_string = "7.5V")]
    V7_5 = 0x5,
    /// 10V
    #[strum(to_string = "10V")]
    V10 = 0x6,
    /// 12.5V
    #[strum(to_string = "12.5V")]
    V12_5 = 0x7,
}

impl From<u8> for MinimumBusVoltage {
    fn from(value: u8) -> Self {
        match value {
            0x0 => MinimumBusVoltage::NoLimit,
            0x1 => MinimumBusVoltage::V4_5,
            0x2 => MinimumBusVoltage::V5,
            0x3 => MinimumBusVoltage::V5_5,
            0x4 => MinimumBusVoltage::V6,
            0x5 => MinimumBusVoltage::V7_5,
            0x6 => MinimumBusVoltage::V10,
            0x7 => MinimumBusVoltage::V12_5,
            _ => panic!("Invalid value for MinimumBusVoltage"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, strum::Display)]
#[repr(u8)]
pub enum MaximumBusVoltage {
    /// No limit
    #[strum(to_string = "No Limit")]
    NoLimit = 0x0,
    /// 20V
    #[strum(to_string = "20V")]
    V20 = 0x1,
    /// 22.5V
    #[strum(to_string = "22.5V")]
    V22_5 = 0x2,
    /// 25V
    #[strum(to_string = "25V")]
    V25 = 0x3,
    /// 27.5V
    #[strum(to_string = "27.5V")]
    V27_5 = 0x4,
    /// 30V
    #[strum(to_string = "30V")]
    V30 = 0x5,
    /// 32.5V
    #[strum(to_string = "32.5V")]
    V32_5 = 0x6,
    /// 35V
    #[strum(to_string = "35V")]
    V35 = 0x7,
}

impl From<u8> for MaximumBusVoltage {
    fn from(value: u8) -> Self {
        match value {
            0x0 => MaximumBusVoltage::NoLimit,
            0x1 => MaximumBusVoltage::V20,
            0x2 => MaximumBusVoltage::V22_5,
            0x3 => MaximumBusVoltage::V25,
            0x4 => MaximumBusVoltage::V27_5,
            0x5 => MaximumBusVoltage::V30,
            0x6 => MaximumBusVoltage::V32_5,
            0x7 => MaximumBusVoltage::V35,
            _ => panic!("Invalid value for MaximumBusVoltage"),
        }
    }
}

impl PartialOrd for MaximumBusVoltage {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for MaximumBusVoltage {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        match (self, other) {
            (MaximumBusVoltage::NoLimit, MaximumBusVoltage::NoLimit) => Ordering::Equal,
            (MaximumBusVoltage::NoLimit, _) => Ordering::Greater,
            (_, MaximumBusVoltage::NoLimit) => Ordering::Less,
            _ => (*self as u8).cmp(&(*other as u8)),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, strum::Display)]
#[repr(u8)]
pub enum AutoRetryTimes {
    /// No limit
    #[strum(to_string = "No Limit")]
    NoLimit = 0x0,
    /// 2 retries
    #[strum(to_string = "2")]
    Retry2 = 0x1,
    /// 3 retries
    #[strum(to_string = "3")]
    Retry3 = 0x2,
    /// 5 retries
    #[strum(to_string = "5")]
    Retry5 = 0x3,
    /// 7 retries
    #[strum(to_string = "7")]
    Retry7 = 0x4,
    /// 10 retries
    #[strum(to_string = "10")]
    Retry10 = 0x5,
    /// 15 retries
    #[strum(to_string = "15")]
    Retry15 = 0x6,
    /// 20 retries
    #[strum(to_string = "20")]
    Retry20 = 0x7,
}

impl From<u8> for AutoRetryTimes {
    fn from(value: u8) -> Self {
        match value {
            0x0 => AutoRetryTimes::NoLimit,
            0x1 => AutoRetryTimes::Retry2,
            0x2 => AutoRetryTimes::Retry3,
            0x3 => AutoRetryTimes::Retry5,
            0x4 => AutoRetryTimes::Retry7,
            0x5 => AutoRetryTimes::Retry10,
            0x6 => AutoRetryTimes::Retry15,
            0x7 => AutoRetryTimes::Retry20,
            _ => panic!("Invalid value for AutoRetryTimes"),
        }
    }
}

impl PartialOrd for AutoRetryTimes {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for AutoRetryTimes {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        match (self, other) {
            (AutoRetryTimes::NoLimit, AutoRetryTimes::NoLimit) => Ordering::Equal,
            (AutoRetryTimes::NoLimit, _) => Ordering::Greater,
            (_, AutoRetryTimes::NoLimit) => Ordering::Less,
            _ => (*self as u8).cmp(&(*other as u8)),
        }
    }
}





