//! Section 7.7.1.3

use super::*;
use arbitrary_int::*;
use bitbybit::*;

/// Register to configure motor startup settings1
#[bitfield(u32, default = 0x0)]
#[derive(Debug, PartialEq, Eq)]
pub struct MotorStartup1 {
    /// Mortor start-up method
    #[bits(29..=30, rw)]
    pub mtr_startup: StartupMode,
    /// Align, slow first cycle, and open loop current ramp rate
    #[bits(25..=28, rw)]
    pub align_slow_ramp_rate: CurrentRampRate,
    /// Align time
    #[bits(21..=24, rw)]
    pub align_time: AlignTime,
    /// Align or slow first cycle current limit
    #[bits(17..=20, rw)]
    pub align_or_slow_current_ilimit: CurrentSelection,
    /// Initial Position Detection (IPD) Clock Frequency
    #[bits(14..=16, rw)]
    pub ipd_clk_freq: IpdClockFreq,
    /// Initial Position Detection (IPD) Current Threshold
    #[bits(9..=13, rw)]
    pub ipd_curr_thr: Option<IpdCurrentThreshold>,
    /// Initial Position Detection (IPD) Release Mode
    /// 0 = Brake, 1 = Tristate
    #[bit(8, rw)]
    pub ipd_rls_mode: bool,
    /// Initial Position Detection (IPD) Advance Angle
    #[bits(6..=7, rw)]
    pub ipd_adv_angle: IpdAdvanceAngle,
    /// Number of times IPD is executed.
    /// Limited 0-3
    #[bits(4..=5, rw)]
    pub ipd_repeat: u2,
    /// Open Loop Current Limit Configuration.
    /// 0 = Open loop current limit defined by OL_ILIMIT,
    /// 1 = Open loop current limit defined by ILIMIT
    #[bit(3, rw)]
    pub ol_ilimit_config: bool,
    /// Iq ramp down for transition from open loop to closed loop
    /// 0 = Iq ramp down disabled, 1 = Iq ramp down enabled
    #[bit(2, rw)]
    pub iq_ramp_en: bool,
    /// Enables active braking during deceleration
    /// 0 = Active braking disabled, 1 = Active braking enabled
    #[bit(1, rw)]
    pub active_brake_en: bool,
    /// Choose between forward and reverse drive setting for reverse drive.
    /// 0 = Open loop current, A1, A2 based on forward drive,
    /// 1 = Open loop current, A1, A2 based on reverse drive
    #[bit(0, rw)]
    pub rev_drv_config: bool,
}

impl Register for MotorStartup1 {
    const ADDRESS: u12 = MOTOR_STARTUP1;

    fn value(&self) -> u32 {
        self.raw_value()
    }
}

/// Motor start-up method
#[bitenum(u2, exhaustive = true)]
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, strum::Display)]
pub enum StartupMode {
    /// Align
    #[strum(to_string = "Align")]
    Align = 0x0,
    /// Double Align
    #[strum(to_string = "Double Align")]
    DoubleAlign = 0x1,
    /// Initial Position Detection
    #[strum(to_string = "IPD")]
    Ipd = 0x2,
    /// Slow First Cycle
    #[strum(to_string = "Slow First Cycle")]
    SlowFirstCycle = 0x3,
}

/// Align, slow first cycle and open loop current ramp rate
#[bitenum(u4, exhaustive = true)]
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, strum::Display)]
pub enum CurrentRampRate {
    /// 0.1 A/s
    #[strum(to_string = "0.1 A/s")]
    A0_1 = 0x0,
    /// 1 A/s
    #[strum(to_string = "1 A/s")]
    A1 = 0x1,
    /// 5 A/s
    #[strum(to_string = "5 A/s")]
    A5 = 0x2,
    /// 10 A/s
    #[strum(to_string = "10 A/s")]
    A10 = 0x3,
    /// 15 A/s
    #[strum(to_string = "15 A/s")]
    A15 = 0x4,
    /// 25 A/s
    #[strum(to_string = "25 A/s")]
    A25 = 0x5,
    /// 50 A/s
    #[strum(to_string = "50 A/s")]
    A50 = 0x6,
    /// 100 A/s
    #[strum(to_string = "100 A/s")]
    A100 = 0x7,
    /// 150 A/s
    #[strum(to_string = "150 A/s")]
    A150 = 0x8,
    /// 200 A/s
    #[strum(to_string = "200 A/s")]
    A200 = 0x9,
    /// 250 A/s
    #[strum(to_string = "250 A/s")]
    A250 = 0xA,
    /// 500 A/s
    #[strum(to_string = "500 A/s")]
    A500 = 0xB,
    /// 1000 A/s
    #[strum(to_string = "1000 A/s")]
    A1000 = 0xC,
    /// 2000 A/s
    #[strum(to_string = "2000 A/s")]
    A2000 = 0xD,
    /// 5000 A/s
    #[strum(to_string = "5000 A/s")]
    A5000 = 0xE,
    /// No limit
    #[strum(to_string = "No limit")]
    NoLimit = 0xF,
}

/// Align time
#[bitenum(u4, exhaustive = true)]
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, strum::Display)]
pub enum AlignTime {
    /// 10ms
    #[strum(to_string = "10 ms")]
    MS10 = 0x0,
    /// 50ms
    #[strum(to_string = "50 ms")]
    MS50 = 0x1,
    /// 100ms
    #[strum(to_string = "100 ms")]
    MS100 = 0x2,
    /// 200ms
    #[strum(to_string = "200 ms")]
    MS200 = 0x3,
    /// 300ms
    #[strum(to_string = "300 ms")]
    MS300 = 0x4,
    /// 400ms
    #[strum(to_string = "400 ms")]
    MS400 = 0x5,
    /// 500ms
    #[strum(to_string = "500 ms")]
    MS500 = 0x6,
    /// 750ms
    #[strum(to_string = "750 ms")]
    MS750 = 0x7,
    /// 1s
    #[strum(to_string = "1 s")]
    S1 = 0x8,
    /// 1.5s
    #[strum(to_string = "1.5 s")]
    S1_5 = 0x9,
    /// 2s
    #[strum(to_string = "2 s")]
    S2 = 0xA,
    /// 3s
    #[strum(to_string = "3 s")]
    S3 = 0xB,
    /// 4s
    #[strum(to_string = "4 s")]
    S4 = 0xC,
    /// 5s
    #[strum(to_string = "5 s")]
    S5 = 0xD,
    /// 7.5s
    #[strum(to_string = "7.5 s")]
    S7_5 = 0xE,
    /// 10s
    #[strum(to_string = "10 s")]
    S10 = 0xF,
}

/// IPD Clock Frequency
#[bitenum(u3, exhaustive = true)]
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, strum::Display)]
pub enum IpdClockFreq {
    /// 50 Hz
    #[strum(to_string = "50 Hz")]
    Hz50 = 0x0,
    /// 100 Hz
    #[strum(to_string = "100 Hz")]
    Hz100 = 0x1,
    /// 250 Hz
    #[strum(to_string = "250 Hz")]
    Hz250 = 0x2,
    /// 500 Hz
    #[strum(to_string = "500 Hz")]
    Hz500 = 0x3,
    /// 1000 Hz
    #[strum(to_string = "1000 Hz")]
    Hz1000 = 0x4,
    /// 2000 Hz
    #[strum(to_string = "2000 Hz")]
    Hz2000 = 0x5,
    /// 5000 Hz
    #[strum(to_string = "5000 Hz")]
    Hz5000 = 0x6,
    /// 10000 Hz
    #[strum(to_string = "10000 Hz")]
    Hz10000 = 0x7,
}

/// Initial Position Detection (IPD) Current Threshold
#[bitenum(u5, exhaustive = false)]
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, strum::Display)]
pub enum IpdCurrentThreshold {
    /// 0.25A
    #[strum(to_string = "0.25 A")]
    A0_25 = 0x0,
    /// 0.5A
    #[strum(to_string = "0.5 A")]
    A0_5 = 0x1,
    /// 0.75A
    #[strum(to_string = "0.75 A")]
    A0_75 = 0x2,
    /// 1.0A
    #[strum(to_string = "1.0 A")]
    A1_0 = 0x3,
    /// 1.25A
    #[strum(to_string = "1.25 A")]
    A1_25 = 0x4,
    /// 1.5A
    #[strum(to_string = "1.5 A")]
    A1_5 = 0x5,
    /// 2.0A
    #[strum(to_string = "2.0 A")]
    A2_0 = 0x6,
    /// 2.5A
    #[strum(to_string = "2.5 A")]
    A2_5 = 0x7,
    /// 3.0A
    #[strum(to_string = "3.0 A")]
    A3_0 = 0x8,
    /// 3.667A
    #[strum(to_string = "3.667 A")]
    A3_667 = 0x9,
    /// 4.0A
    #[strum(to_string = "4.0 A")]
    A4_0 = 0xA,
    /// 4.667A
    #[strum(to_string = "4.667 A")]
    A4_667 = 0xB,
    /// 5.0A
    #[strum(to_string = "5.0 A")]
    A5_0 = 0xC,
    /// 5.333A
    #[strum(to_string = "5.333 A")]
    A5_333 = 0xD,
    /// 6.0A
    #[strum(to_string = "6.0 A")]
    A6_0 = 0xE,
    /// 6.667A
    #[strum(to_string = "6.667 A")]
    A6_667 = 0xF,
    /// 7.333A
    #[strum(to_string = "7.333 A")]
    A7_333 = 0x10,
    /// 8.0A
    #[strum(to_string = "8.0 A")]
    A8_0 = 0x11,
    // 5-bit field, I know. The rest is N/A. See table 7-18 of the datasheet.
}

/// Initial Position Detection (IPD) Advance Angle
#[bitenum(u2, exhaustive = true)]
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, strum::Display)]
pub enum IpdAdvanceAngle {
    /// 0 degrees
    #[strum(to_string = "0°")]
    Deg0 = 0x0,
    /// 30 degrees
    #[strum(to_string = "30°")]
    Deg30 = 0x1,
    /// 60 degrees
    #[strum(to_string = "60°")]
    Deg60 = 0x2,
    /// 90 degrees
    #[strum(to_string = "90°")]
    Deg90 = 0x3,
}
