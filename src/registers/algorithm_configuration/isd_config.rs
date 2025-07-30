//! Section 7.7.1.1

use super::*;
use arbitrary_int::*;
use bitbybit::*;

/// Register to configure initial speed detect settings.
#[bitfield(u32, debug, default = 0x0)]
#[derive(PartialEq, Eq)]
pub struct IsdConfig {
    /// ISD Enable.
    /// 1 = Enable ISD, 0 = Disable ISD
    #[bit(30, rw)]
    pub isd_en: bool,
    /// Brake Enable.
    /// 1 = Enable Brake, 0 = Disable Brake
    #[bit(29, rw)]
    pub brake_en: bool,
    /// Hi-Z Enable.
    /// 1 = Enable High Impedance, 0 = Disable High Impedance
    #[bit(28, rw)]
    pub hiz_en: bool,
    /// Reverse Drive Enable.
    /// 1 = Enable Reverse Drive, 0 = Disable Reverse Drive
    #[bit(27, rw)]
    pub rvs_dr_en: bool,
    /// Resynchronization Enable.
    /// 1 = Enable Resynchronization, 0 = Disable Resynchronization
    #[bit(26, rw)]
    pub resync_en: bool,
    /// Minimum Speed threshold to resynchronize to close loop (% of MAX_SPEED)
    #[bits(22..=25, rw)]
    pub fw_drv_resyn_thr: ResyncThreshold,
    /// Brake mode
    /// 0 = All high-side FETs on, 1 = All low-side FETs on
    #[bit(21, rw)]
    pub brk_mode: bool,
    /// Brake config
    /// 0 = Brake time is used to exit Brake state,
    /// 1 = Brake current threshold and Brake time are used to exit Brake state
    #[bit(20, rw)]
    pub brk_config: bool,
    /// Brake current threshold
    #[bits(17..=19, rw)]
    pub brk_curr_thr: BrakeCurrentThreshold,
    /// Brake time
    #[bits(13..=16, rw)]
    pub brk_time: TimeFormatA,
    /// Hi-Z time
    #[bits(9..=12, rw)]
    pub hiz_time: TimeFormatA,
    /// BEMF threshold to detect if motor is stationary
    #[bits(6..=8, rw)]
    pub stat_detect_thr: BemfStationaryVoltageThreshold,
    /// Speed threshold used to transition th open loop during reverse drive (% of MAX_SPEED)
    #[bits(2..=5, rw)]
    pub rev_drv_handoff_thr: ReverseDriveHandoffThreshold,
    /// Open loop current limit during reverse drive
    #[bits(0..=1, rw)]
    pub rev_drv_open_loop_current: ReverseDriveOpenLoopCurrent,
}

impl Register for IsdConfig {
    fn address(&self) -> u12 {
        ISD_CONFIG
    }

    fn value(&self) -> u32 {
        self.raw_value()
    }

    fn from_value(value: u32) -> Self {
        Self::new_with_raw_value(value)
    }
}

/// Minimum Speed threshold to resynchronize to close loop (% of MAX_SPEED)
#[bitenum(u4, exhaustive = true)]
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, strum::Display)]
pub enum ResyncThreshold {
    /// 5% of MAX_SPEED
    #[strum(to_string = "5%")]
    P5 = 0x0,
    /// 10% of MAX_SPEED
    #[strum(to_string = "10%")]
    P10 = 0x1,
    /// 15% of MAX_SPEED
    #[strum(to_string = "15%")]
    P15 = 0x2,
    /// 20% of MAX_SPEED
    #[strum(to_string = "20%")]
    P20 = 0x3,
    /// 25% of MAX_SPEED
    #[strum(to_string = "25%")]
    P25 = 0x4,
    /// 30% of MAX_SPEED
    #[strum(to_string = "30%")]
    P30 = 0x5,
    /// 35% of MAX_SPEED
    #[strum(to_string = "35%")]
    P35 = 0x6,
    /// 40% of MAX_SPEED
    #[strum(to_string = "40%")]
    P40 = 0x7,
    /// 45% of MAX_SPEED
    #[strum(to_string = "45%")]
    P45 = 0x8,
    /// 50% of MAX_SPEED
    #[strum(to_string = "50%")]
    P50 = 0x9,
    /// 55% of MAX_SPEED
    #[strum(to_string = "55%")]
    P55 = 0xA,
    /// 60% of MAX_SPEED
    #[strum(to_string = "60%")]
    P60 = 0xB,
    /// 70% of MAX_SPEED
    #[strum(to_string = "70%")]
    P70 = 0xC,
    /// 80% of MAX_SPEED
    #[strum(to_string = "80%")]
    P80 = 0xD,
    /// 90% of MAX_SPEED
    #[strum(to_string = "90%")]
    P90 = 0xE,
    /// 100% of MAX_SPEED
    #[strum(to_string = "100%")]
    P100 = 0xF,
}

/// Brake current threshold
#[bitenum(u3, exhaustive = true)]
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, strum::Display)]
pub enum BrakeCurrentThreshold {
    /// 0.1A
    #[strum(to_string = "0.1 A")]
    A0_1 = 0x0,
    /// 0.2A
    #[strum(to_string = "0.2 A")]
    A0_2 = 0x1,
    /// 0.3A
    #[strum(to_string = "0.3 A")]
    A0_3 = 0x2,
    /// 0.5A
    #[strum(to_string = "0.5 A")]
    A0_5 = 0x3,
    /// 1.0A
    #[strum(to_string = "1.0 A")]
    A1_0 = 0x4,
    /// 2.0A
    #[strum(to_string = "2.0 A")]
    A2_0 = 0x5,
    /// 4.0A
    #[strum(to_string = "4.0 A")]
    A4_0 = 0x6,
    /// 8.0A
    #[strum(to_string = "8.0 A")]
    A8_0 = 0x7,
}

/// A set of durations in milliseconds or seconds.
#[bitenum(u4, exhaustive = true)]
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, strum::Display)]
pub enum TimeFormatA {
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
    /// 2s
    #[strum(to_string = "2 s")]
    S2 = 0x9,
    /// 3s
    #[strum(to_string = "3 s")]
    S3 = 0xA,
    /// 4s
    #[strum(to_string = "4 s")]
    S4 = 0xB,
    /// 5s
    #[strum(to_string = "5 s")]
    S5 = 0xC,
    /// 7.5s
    #[strum(to_string = "7.5 s")]
    S7_5 = 0xD,
    /// 10s
    #[strum(to_string = "10 s")]
    S10 = 0xE,
    /// 15s
    #[strum(to_string = "15 s")]
    S15 = 0xF,
}

/// BEMF threshold to detect if motor is stationary
#[bitenum(u3, exhaustive = true)]
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, strum::Display)]
pub enum BemfStationaryVoltageThreshold {
    /// 50mV
    #[strum(to_string = "50 mV")]
    MV50 = 0x0,
    /// 75mV
    #[strum(to_string = "75 mV")]
    MV75 = 0x1,
    /// 100mV
    #[strum(to_string = "100 mV")]
    MV100 = 0x2,
    /// 250mV
    #[strum(to_string = "250 mV")]
    MV250 = 0x3,
    /// 500mV
    #[strum(to_string = "500 mV")]
    MV500 = 0x4,
    /// 750mV
    #[strum(to_string = "750 mV")]
    MV750 = 0x5,
    /// 1000mV
    #[strum(to_string = "1000 mV")]
    MV1000 = 0x6,
    /// 1500mV
    #[strum(to_string = "1500 mV")]
    MV1500 = 0x7,
}

/// Speed threshold used to transition to open loop during reverse drive
/// (% of MAX_SPEED)
#[bitenum(u4, exhaustive = true)]
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, strum::Display)]
pub enum ReverseDriveHandoffThreshold {
    /// 2.5% of MAX_SPEED
    #[strum(to_string = "2.5%")]
    P2_5 = 0x0,
    /// 5% of MAX_SPEED
    #[strum(to_string = "5%")]
    P5 = 0x1,
    /// 7.5% of MAX_SPEED
    #[strum(to_string = "7.5%")]
    P7_5 = 0x2,
    /// 10% of MAX_SPEED
    #[strum(to_string = "10%")]
    P10 = 0x3,
    /// 12.5% of MAX_SPEED
    #[strum(to_string = "12.5%")]
    P12_5 = 0x4,
    /// 15% of MAX_SPEED
    #[strum(to_string = "15%")]
    P15 = 0x5,
    /// 20% of MAX_SPEED
    #[strum(to_string = "20%")]
    P20 = 0x6,
    /// 25% of MAX_SPEED
    #[strum(to_string = "25%")]
    P25 = 0x7,
    /// 30% of MAX_SPEED
    #[strum(to_string = "30%")]
    P30 = 0x8,
    /// 40% of MAX_SPEED
    #[strum(to_string = "40%")]
    P40 = 0x9,
    /// 50% of MAX_SPEED
    #[strum(to_string = "50%")]
    P50 = 0xA,
    /// 60% of MAX_SPEED
    #[strum(to_string = "60%")]
    P60 = 0xB,
    /// 70% of MAX_SPEED
    #[strum(to_string = "70%")]
    P70 = 0xC,
    /// 80% of MAX_SPEED
    #[strum(to_string = "80%")]
    P80 = 0xD,
    /// 90% of MAX_SPEED
    #[strum(to_string = "90%")]
    P90 = 0xE,
    /// 100% of MAX_SPEED
    #[strum(to_string = "100%")]
    P100 = 0xF,
}

/// Open loop current limit during reverse drive
#[bitenum(u2, exhaustive = true)]
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, strum::Display)]
pub enum ReverseDriveOpenLoopCurrent {
    /// 1.5A
    #[strum(to_string = "1.5 A")]
    A1_5 = 0x0,
    /// 2.5A
    #[strum(to_string = "2.5 A")]
    A2_5 = 0x1,
    /// 3.5A
    #[strum(to_string = "3.5 A")]
    A3_5 = 0x2,
    /// 5.0A
    #[strum(to_string = "5.0 A")]
    A5_0 = 0x3,
}
