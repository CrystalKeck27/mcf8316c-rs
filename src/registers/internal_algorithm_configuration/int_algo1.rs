//! Section 7.7.4.1

use super::*;
use arbitrary_int::*;
use bitbybit::*;

/// Register to configure internal algorithm parameters1
#[bitfield(u32, default = 0x0)]
#[derive(Debug, PartialEq, Eq)]
pub struct IntAlgo1 {
    /// Difference between final speed and present speed below which
    /// active braking will be stopped
    #[bits(29..=30, rw)]
    pub active_brake_speed_delta_limit_exit: SpeedDeltaLimitExit,
    /// Glitch filter applied on speed pin input
    #[bits(27..=28, rw)]
    pub speed_pin_glitch_filter: SpeedPinGlitchFilter,
    /// Enable fast speed detection during ISD.
    /// 0 = Disable Fast ISD,
    /// 1 = Enable Fast ISD
    #[bit(26, rw)]
    pub fast_isd_en: bool,
    /// Persistence time for declaring motor has stopped
    #[bits(24..=25, rw)]
    pub isd_stop_time: PersistanceTime,
    /// Persistence time for declaring motor is running
    #[bits(22..=23, rw)]
    pub isd_run_time: PersistanceTime,
    /// Timeout in case ISD is unable to reliably detect speed or direction
    #[bits(20..=21, rw)]
    pub isd_timeout: IsdTimeout,
    /// Minimum BEMF for handoff
    #[bits(17..=19, rw)]
    pub auto_handoff_min_bemf: AutoHandoffBemf,
    /// Persistence time for current below threshold during current based ISD brake
    #[bits(15..=16, rw)]
    pub brake_current_persist: BrakeCurrentPersist,
    /// IPD current limit for MPET
    #[bits(13..=14, rw)]
    pub mpet_ipd_current_limit: MpetIpdCurrentLimit,
    /// Number of times IPD is executed for MPET
    #[bits(11..=12, rw)]
    pub mpet_ipd_freq: MpetIpdCount,
    /// Open loop current reference for MPET
    #[bits(8..=10, rw)]
    pub mpet_open_loop_current_ref: MpetOpenLoopCurrentRef,
    /// Open loop speed reference for MPET (% of MAX_SPEED)
    #[bits(6..=7, rw)]
    pub mpet_open_loop_speed_ref: MpetOpenLoopSpeedRef,
    /// Open loop acceleration for MPET
    #[bits(3..=5, rw)]
    pub mpet_open_loop_slew_rate: MpetOpenLoopSlewRate,
    /// % of open loop acceleration to be applied during open loop
    /// deceleration in reverse drive
    #[bits(0..=2, rw)]
    pub rev_drv_open_loop_dec: ReverseOpenLoopDeceleration,
}

impl Register for IntAlgo1 {
    const ADDRESS: u12 = INT_ALGO_1;

    fn value(&self) -> u32 {
        self.raw_value()
    }
}

/// Difference between final speed and present speed below which
/// active braking will be stopped
#[bitenum(u2, exhaustive = true)]
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, strum::Display)]
pub enum SpeedDeltaLimitExit {
    /// 2.5%
    #[strum(to_string = "2.5%")]
    P2_5 = 0x0,
    /// 5%
    #[strum(to_string = "5%")]
    P5 = 0x1,
    /// 7.5%
    #[strum(to_string = "7.5%")]
    P7_5 = 0x2,
    /// 10%
    #[strum(to_string = "10%")]
    P10 = 0x3,
}

/// Glitch filter applied on speed pin input
#[bitenum(u2, exhaustive = true)]
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, strum::Display)]
pub enum SpeedPinGlitchFilter {
    /// No Glitch Filter
    #[strum(to_string = "No Glitch Filter")]
    None = 0x0,
    /// 0.2µs
    #[strum(to_string = "0.2µs")]
    Us02 = 0x1,
    /// 0.5µs
    #[strum(to_string = "0.5µs")]
    Us05 = 0x2,
    /// 1µs
    #[strum(to_string = "1µs")]
    Us1 = 0x3,
}

/// Persistence time
#[bitenum(u2, exhaustive = true)]
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, strum::Display)]
pub enum PersistanceTime {
    /// 1ms
    #[strum(to_string = "1ms")]
    Ms1 = 0x0,
    /// 5ms
    #[strum(to_string = "5ms")]
    Ms5 = 0x1,
    /// 50ms
    #[strum(to_string = "50ms")]
    Ms50 = 0x2,
    /// 100ms
    #[strum(to_string = "100ms")]
    Ms100 = 0x3,
}

/// Timeout in case ISD is unable to reliably detect speed or direction
#[bitenum(u2, exhaustive = true)]
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, strum::Display)]
pub enum IsdTimeout {
    /// 500ms
    #[strum(to_string = "500ms")]
    Ms500 = 0x0,
    /// 750ms
    #[strum(to_string = "750ms")]
    Ms750 = 0x1,
    /// 1000ms
    #[strum(to_string = "1000ms")]
    Ms1000 = 0x2,
    /// 2000ms
    #[strum(to_string = "2000ms")]
    Ms2000 = 0x3,
}

/// Minimum BEMF for handoff
#[bitenum(u3, exhaustive = true)]
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, strum::Display)]
pub enum AutoHandoffBemf {
    /// 0mV
    #[strum(to_string = "0mV")]
    Mv0 = 0x0,
    /// 50mV
    #[strum(to_string = "50mV")]
    Mv50 = 0x1,
    /// 100mV
    #[strum(to_string = "100mV")]
    Mv100 = 0x2,
    /// 250mV
    #[strum(to_string = "250mV")]
    Mv250 = 0x3,
    /// 500mV
    #[strum(to_string = "500mV")]
    Mv500 = 0x4,
    /// 1000mV
    #[strum(to_string = "1000mV")]
    Mv1000 = 0x5,
    /// 1250mV
    #[strum(to_string = "1250mV")]
    Mv1250 = 0x6,
    /// 1500mV
    #[strum(to_string = "1500mV")]
    Mv1500 = 0x7,
}

/// Persistence time for current below threshold during current based
/// ISD brake
#[bitenum(u2, exhaustive = true)]
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, strum::Display)]
pub enum BrakeCurrentPersist {
    /// 50ms
    #[strum(to_string = "50ms")]
    Ms50 = 0x0,
    /// 100ms
    #[strum(to_string = "100ms")]
    Ms100 = 0x1,
    /// 250ms
    #[strum(to_string = "250ms")]
    Ms250 = 0x2,
    /// 500ms
    #[strum(to_string = "500ms")]
    Ms500 = 0x3,
}

/// IPD current limit for MPET
#[bitenum(u2, exhaustive = true)]
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, strum::Display)]
pub enum MpetIpdCurrentLimit {
    /// 0.1A
    #[strum(to_string = "0.1A")]
    A0_1 = 0x0,
    /// 0.5A
    #[strum(to_string = "0.5A")]
    A0_5 = 0x1,
    /// 1.0A
    #[strum(to_string = "1.0A")]
    A1_0 = 0x2,
    /// 2.0A
    #[strum(to_string = "2.0A")]
    A2_0 = 0x3,
}

/// Number of times IPD is executed for MPET
#[bitenum(u2, exhaustive = true)]
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, strum::Display)]
pub enum MpetIpdCount {
    /// 1
    #[strum(to_string = "1")]
    C1 = 0x0,
    /// 2
    #[strum(to_string = "2")]
    C2 = 0x1,
    /// 4
    #[strum(to_string = "4")]
    C4 = 0x2,
    /// 8
    #[strum(to_string = "8")]
    C8 = 0x3,
}

/// Open loop current reference for MPET
#[bitenum(u3, exhaustive = true)]
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, strum::Display)]
pub enum MpetOpenLoopCurrentRef {
    /// 1.0A
    #[strum(to_string = "1.0A")]
    A1 = 0x0,
    /// 2.0A
    #[strum(to_string = "2.0A")]
    A2 = 0x1,
    /// 3.0A
    #[strum(to_string = "3.0A")]
    A3 = 0x2,
    /// 4.0A
    #[strum(to_string = "4.0A")]
    A4 = 0x3,
    /// 5.0A
    #[strum(to_string = "5.0A")]
    A5 = 0x4,
    /// 6.0A
    #[strum(to_string = "6.0A")]
    A6 = 0x5,
    /// 7.0A
    #[strum(to_string = "7.0A")]
    A7 = 0x6,
    /// 8.0A
    #[strum(to_string = "8.0A")]
    A8 = 0x7,
}

/// Open loop speed reference for MPET (% of MAX_SPEED)
#[bitenum(u2, exhaustive = true)]
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, strum::Display)]
pub enum MpetOpenLoopSpeedRef {
    /// 15% of MAX_SPEED
    #[strum(to_string = "15%")]
    P15 = 0x0,
    /// 25% of MAX_SPEED
    #[strum(to_string = "25%")]
    P25 = 0x1,
    /// 35% of MAX_SPEED
    #[strum(to_string = "35%")]
    P35 = 0x2,
    /// 50% of MAX_SPEED
    #[strum(to_string = "50%")]
    P50 = 0x3,
}

/// Open loop acceleration for MPET
#[bitenum(u3, exhaustive = true)]
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, strum::Display)]
pub enum MpetOpenLoopSlewRate {
    /// 0.1Hz/s
    #[strum(to_string = "0.1Hz/s")]
    Hz0_1 = 0x0,
    /// 0.5Hz/s
    #[strum(to_string = "0.5Hz/s")]
    Hz0_5 = 0x1,
    /// 1.0Hz/s
    #[strum(to_string = "1.0Hz/s")]
    Hz1_0 = 0x2,
    /// 2.0Hz/s
    #[strum(to_string = "2.0Hz/s")]
    Hz2_0 = 0x3,
    /// 3.0Hz/s
    #[strum(to_string = "3.0Hz/s")]
    Hz3_0 = 0x4,
    /// 5.0Hz/s
    #[strum(to_string = "5.0Hz/s")]
    Hz5_0 = 0x5,
    /// 10.0Hz/s
    #[strum(to_string = "10.0Hz/s")]
    Hz10_0 = 0x6,
    /// 20.0Hz/s
    #[strum(to_string = "20.0Hz/s")]
    Hz20_0 = 0x7,
}

/// % of open loop acceleration to be applied during open loop
/// deceleration in reverse drive
#[bitenum(u3, exhaustive = true)]
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, strum::Display)]
pub enum ReverseOpenLoopDeceleration {
    /// 50%
    #[strum(to_string = "50%")]
    P50 = 0x0,
    /// 60%
    #[strum(to_string = "60%")]
    P60 = 0x1,
    /// 70%
    #[strum(to_string = "70%")]
    P70 = 0x2,
    /// 80%
    #[strum(to_string = "80%")]
    P80 = 0x3,
    /// 90%
    #[strum(to_string = "90%")]
    P90 = 0x4,
    /// 100%
    #[strum(to_string = "100%")]
    P100 = 0x5,
    /// 125%
    #[strum(to_string = "125%")]
    P125 = 0x6,
    /// 150%
    #[strum(to_string = "150%")]
    P150 = 0x7,
}
