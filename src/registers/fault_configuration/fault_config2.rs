//! Section 7.7.2.2

use super::*;
use arbitrary_int::*;
use bitbybit::*;

/// Register to configure fault settings2
#[bitfield(u32, default = 0x0)]
#[derive(Debug, PartialEq, Eq)]
pub struct FaultConfig2 {
    /// Lock 1 (Abnormal Speed) Enable.
    /// 0 = Disable, 1 = Enable
    #[bit(30, rw)]
    pub lock1_en: bool,
    /// Lock 2 (Abnormal BEMF) Enable.
    /// 0 = Disable, 1 = Enable
    #[bit(29, rw)]
    pub lock2_en: bool,
    /// Lock 3 (No Motor) Enable.
    /// 0 = Disable, 1 = Enable
    #[bit(28, rw)]
    pub lock3_en: bool,
    /// Abnormal speed lock threshold (% of MAX_SPEED)
    #[bits(25..=27, rw)]
    pub lock_abn_speed: AbnormalSpeedLockThreshold,
    /// Abnormal BEMF lock threshold (% of expected BEMF)
    #[bits(22..=24, rw)]
    pub abnormal_bemf_thr: AbnormalBemfThreshold,
    /// No motor lock threshold
    #[bits(19..=21, rw)]
    pub no_mtr_thr: NoMotorThreshold,
    /// Hardware lock detection current mode
    #[bits(15..=18, rw)]
    pub hw_lock_ilimit_mode: LockModeRaw,
    /// Hardware lock detection current limit deglitch time
    #[bits(12..=14, rw)]
    pub hw_lock_ilimit_deg: LockIlimitDeglitchTime,
    /// Minimum DC Bus voltage for running motor
    #[bits(8..=10, rw)]
    pub min_vm_motor: MinimumBusVoltage,
    /// DC Bus Undervoltage Fault Recovery Mode.
    /// 0 = Latch on undervoltage,
    /// 1 = Automatic clear if voltage in bounds
    #[bit(7, rw)]
    pub min_vm_mode: bool,
    /// Maximum DC Bus voltage for running motor
    #[bits(4..=6, rw)]
    pub max_vm_motor: MaximumBusVoltage,
    /// DC Bus Overvoltage Fault Recovery Mode.
    /// 0 = Latch on overvoltage,
    /// 1 = Automatic clear if voltage in bounds
    #[bit(3, rw)]
    pub max_vm_mode: bool,
    /// Automatic retry attempts
    #[bits(0..=2, rw)]
    pub auto_retry_times: AutoRetryTimes,
}

impl Register for FaultConfig2 {
    const ADDRESS: u12 = FAULT_CONFIG2;

    fn value(&self) -> u32 {
        self.raw_value()
    }
}

/// Abnormal speed lock threshold (% of MAX_SPEED)
#[bitenum(u3, exhaustive = true)]
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, strum::Display)]
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

/// Abnormal BEMF lock threshold (% of expected BEMF)
#[bitenum(u3, exhaustive = true)]
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, strum::Display)]
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

/// No motor lock threshold
#[bitenum(u3, exhaustive = true)]
#[derive(Debug, PartialEq, Eq, strum::Display)]
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

/// Hardware lock detection current limit deglitch time
#[bitenum(u3, exhaustive = true)]
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, strum::Display)]
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

/// Minimum DC Bus voltage for running motor
#[bitenum(u3, exhaustive = true)]
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, strum::Display)]
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

/// Maximum DC Bus voltage for running motor
#[bitenum(u3, exhaustive = true)]
#[derive(Debug, PartialEq, Eq, strum::Display)]
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

impl PartialOrd for MaximumBusVoltage {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for MaximumBusVoltage {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        match (self, other) {
            (MaximumBusVoltage::NoLimit, MaximumBusVoltage::NoLimit) => core::cmp::Ordering::Equal,
            (MaximumBusVoltage::NoLimit, _) => core::cmp::Ordering::Greater,
            (_, MaximumBusVoltage::NoLimit) => core::cmp::Ordering::Less,
            _ => (*self as u8).cmp(&(*other as u8)),
        }
    }
}

/// Automatic retry attempts
#[bitenum(u3, exhaustive = true)]
#[derive(Debug, PartialEq, Eq, strum::Display)]
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

impl PartialOrd for AutoRetryTimes {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for AutoRetryTimes {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        match (self, other) {
            (AutoRetryTimes::NoLimit, AutoRetryTimes::NoLimit) => core::cmp::Ordering::Equal,
            (AutoRetryTimes::NoLimit, _) => core::cmp::Ordering::Greater,
            (_, AutoRetryTimes::NoLimit) => core::cmp::Ordering::Less,
            _ => (*self as u8).cmp(&(*other as u8)),
        }
    }
}





