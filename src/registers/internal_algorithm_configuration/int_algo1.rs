use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IntAlgo1 {
    /// Difference between final speed and present speed below which
    /// active braking will be stopped
    pub active_brake_speed_delta_limit_exit: SpeedDeltaLimitExit,
    /// Glitch filter applied on speed pin input
    pub speed_pin_glitch_filter: SpeedPinGlitchFilter,
    /// Enable fast speed detection during ISD.
    /// 0 = Disable Fast ISD,
    /// 1 = Enable Fast ISD
    pub fast_isd_en: bool,
    /// Persistence time for declaring motor has stopped
    pub isd_stop_time: PersistanceTime,
    /// Persistence time for declaring motor is running
    pub isd_run_time: PersistanceTime,
    /// Timeout in case ISD is unable to reliably detect speed or direction
    pub isd_timeout: IsdTimeout,
    /// Minimum BEMF for handoff
    pub auto_handoff_min_bemf: AutoHandoffBemf,
    /// Persistence time for current below threshold during current based ISD brake
    pub brake_current_persist: BrakeCurrentPersist,
    /// IPD current limit for MPET
    pub mpet_ipd_current_limit: MpetIpdCurrentLimit,
    /// Number of times IPD is executed for MPET
    pub mpet_ipd_freq: MpetIpdCount,
    /// Open loop current reference for MPET
    pub mpet_open_loop_current_ref: MpetOpenLoopCurrentRef,
    /// Open loop speed reference for MPET (% of MAX_SPEED)
    pub mpet_open_loop_speed_ref: MpetOpenLoopSpeedRef,
    /// Open loop acceleration for MPET
    pub mpet_open_loop_slew_rate: MpetOpenLoopSlewRate,
    /// % of open loop acceleration to be applied during open loop
    /// deceleration in reverse drive
    pub rev_drv_open_loop_dec: ReverseOpenLoopDeceleration,
}

impl Register for IntAlgo1 {
    const ADDRESS: u16 = INT_ALGO_1;
}

impl From<IntAlgo1> for u32 {
    fn from(config: IntAlgo1) -> Self {
        let mut value = 0;
        value |= (config.active_brake_speed_delta_limit_exit as u32) << 29;
        value |= (config.speed_pin_glitch_filter as u32) << 27;
        value |= (config.fast_isd_en as u32) << 26;
        value |= (config.isd_stop_time as u32) << 24;
        value |= (config.isd_run_time as u32) << 22;
        value |= (config.isd_timeout as u32) << 20;
        value |= (config.auto_handoff_min_bemf as u32) << 17;
        value |= (config.brake_current_persist as u32) << 15;
        value |= (config.mpet_ipd_current_limit as u32) << 13;
        value |= (config.mpet_ipd_freq as u32) << 11;
        value |= (config.mpet_open_loop_current_ref as u32) << 8;
        value |= (config.mpet_open_loop_speed_ref as u32) << 6;
        value |= (config.mpet_open_loop_slew_rate as u32) << 3;
        value |= config.rev_drv_open_loop_dec as u32;
        value
    }
}

impl From<u32> for IntAlgo1 {
    fn from(value: u32) -> Self {
        IntAlgo1 {
            active_brake_speed_delta_limit_exit: SpeedDeltaLimitExit::from(((value >> 29) & 0x03) as u8),
            speed_pin_glitch_filter: SpeedPinGlitchFilter::from(((value >> 27) & 0x03) as u8),
            fast_isd_en: ((value >> 26) & 0x01) != 0,
            isd_stop_time: PersistanceTime::from(((value >> 24) & 0x03) as u8),
            isd_run_time: PersistanceTime::from(((value >> 22) & 0x03) as u8),
            isd_timeout: IsdTimeout::from(((value >> 20) & 0x03) as u8),
            auto_handoff_min_bemf: AutoHandoffBemf::from(((value >> 17) & 0x07) as u8),
            brake_current_persist: BrakeCurrentPersist::from(((value >> 15) & 0x03) as u8),
            mpet_ipd_current_limit: MpetIpdCurrentLimit::from(((value >> 13) & 0x03) as u8),
            mpet_ipd_freq: MpetIpdCount::from(((value >> 11) & 0x03) as u8),
            mpet_open_loop_current_ref: MpetOpenLoopCurrentRef::from(((value >> 8) & 0x07) as u8),
            mpet_open_loop_speed_ref: MpetOpenLoopSpeedRef::from(((value >> 6) & 0x03) as u8),
            mpet_open_loop_slew_rate: MpetOpenLoopSlewRate::from(((value >> 3) & 0x07) as u8),
            rev_drv_open_loop_dec: ReverseOpenLoopDeceleration::from((value & 0x07) as u8),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, strum::Display)]
#[repr(u8)]
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

impl From<u8> for SpeedDeltaLimitExit {
    fn from(value: u8) -> Self {
        match value {
            0x0 => SpeedDeltaLimitExit::P2_5,
            0x1 => SpeedDeltaLimitExit::P5,
            0x2 => SpeedDeltaLimitExit::P7_5,
            0x3 => SpeedDeltaLimitExit::P10,
            _ => panic!("Invalid value for SpeedDeltaLimitExit"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, strum::Display)]
#[repr(u8)]
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

impl From<u8> for SpeedPinGlitchFilter {
    fn from(value: u8) -> Self {
        match value {
            0x0 => SpeedPinGlitchFilter::None,
            0x1 => SpeedPinGlitchFilter::Us02,
            0x2 => SpeedPinGlitchFilter::Us05,
            0x3 => SpeedPinGlitchFilter::Us1,
            _ => panic!("Invalid value for SpeedPinGlitchFilter"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, strum::Display)]
#[repr(u8)]
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

impl From<u8> for PersistanceTime {
    fn from(value: u8) -> Self {
        match value {
            0x0 => PersistanceTime::Ms1,
            0x1 => PersistanceTime::Ms5,
            0x2 => PersistanceTime::Ms50,
            0x3 => PersistanceTime::Ms100,
            _ => panic!("Invalid value for PersistanceTime"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, strum::Display)]
#[repr(u8)]
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

impl From<u8> for IsdTimeout {
    fn from(value: u8) -> Self {
        match value {
            0x0 => IsdTimeout::Ms500,
            0x1 => IsdTimeout::Ms750,
            0x2 => IsdTimeout::Ms1000,
            0x3 => IsdTimeout::Ms2000,
            _ => panic!("Invalid value for IsdTimeout"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, strum::Display)]
#[repr(u8)]
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

impl From<u8> for AutoHandoffBemf {
    fn from(value: u8) -> Self {
        match value {
            0x0 => AutoHandoffBemf::Mv0,
            0x1 => AutoHandoffBemf::Mv50,
            0x2 => AutoHandoffBemf::Mv100,
            0x3 => AutoHandoffBemf::Mv250,
            0x4 => AutoHandoffBemf::Mv500,
            0x5 => AutoHandoffBemf::Mv1000,
            0x6 => AutoHandoffBemf::Mv1250,
            0x7 => AutoHandoffBemf::Mv1500,
            _ => panic!("Invalid value for AutoHandoffBemf"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, strum::Display)]
#[repr(u8)]
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

impl From<u8> for BrakeCurrentPersist {
    fn from(value: u8) -> Self {
        match value {
            0x0 => BrakeCurrentPersist::Ms50,
            0x1 => BrakeCurrentPersist::Ms100,
            0x2 => BrakeCurrentPersist::Ms250,
            0x3 => BrakeCurrentPersist::Ms500,
            _ => panic!("Invalid value for BrakeCurrentPersist"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, strum::Display)]
#[repr(u8)]
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

impl From<u8> for MpetIpdCurrentLimit {
    fn from(value: u8) -> Self {
        match value {
            0x0 => MpetIpdCurrentLimit::A0_1,
            0x1 => MpetIpdCurrentLimit::A0_5,
            0x2 => MpetIpdCurrentLimit::A1_0,
            0x3 => MpetIpdCurrentLimit::A2_0,
            _ => panic!("Invalid value for MpetIpdCurrentLimit"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, strum::Display)]
#[repr(u8)]
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

impl From<u8> for MpetIpdCount {
    fn from(value: u8) -> Self {
        match value {
            0x0 => MpetIpdCount::C1,
            0x1 => MpetIpdCount::C2,
            0x2 => MpetIpdCount::C4,
            0x3 => MpetIpdCount::C8,
            _ => panic!("Invalid value for MpetIpdCount"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, strum::Display)]
#[repr(u8)]
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

impl From<u8> for MpetOpenLoopCurrentRef {
    fn from(value: u8) -> Self {
        match value {
            0x0 => MpetOpenLoopCurrentRef::A1,
            0x1 => MpetOpenLoopCurrentRef::A2,
            0x2 => MpetOpenLoopCurrentRef::A3,
            0x3 => MpetOpenLoopCurrentRef::A4,
            0x4 => MpetOpenLoopCurrentRef::A5,
            0x5 => MpetOpenLoopCurrentRef::A6,
            0x6 => MpetOpenLoopCurrentRef::A7,
            0x7 => MpetOpenLoopCurrentRef::A8,
            _ => panic!("Invalid value for MpetOpenLoopCurrentRef"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, strum::Display)]
#[repr(u8)]
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

impl From<u8> for MpetOpenLoopSpeedRef {
    fn from(value: u8) -> Self {
        match value {
            0x0 => MpetOpenLoopSpeedRef::P15,
            0x1 => MpetOpenLoopSpeedRef::P25,
            0x2 => MpetOpenLoopSpeedRef::P35,
            0x3 => MpetOpenLoopSpeedRef::P50,
            _ => panic!("Invalid value for MpetOpenLoopSpeedRef"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, strum::Display)]
#[repr(u8)]
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

impl From<u8> for MpetOpenLoopSlewRate {
    fn from(value: u8) -> Self {
        match value {
            0x0 => MpetOpenLoopSlewRate::Hz0_1,
            0x1 => MpetOpenLoopSlewRate::Hz0_5,
            0x2 => MpetOpenLoopSlewRate::Hz1_0,
            0x3 => MpetOpenLoopSlewRate::Hz2_0,
            0x4 => MpetOpenLoopSlewRate::Hz3_0,
            0x5 => MpetOpenLoopSlewRate::Hz5_0,
            0x6 => MpetOpenLoopSlewRate::Hz10_0,
            0x7 => MpetOpenLoopSlewRate::Hz20_0,
            _ => panic!("Invalid value for MpetOpenLoopSlewRate"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, strum::Display)]
#[repr(u8)]
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

impl From<u8> for ReverseOpenLoopDeceleration {
    fn from(value: u8) -> Self {
        match value {
            0x0 => ReverseOpenLoopDeceleration::P50,
            0x1 => ReverseOpenLoopDeceleration::P60,
            0x2 => ReverseOpenLoopDeceleration::P70,
            0x3 => ReverseOpenLoopDeceleration::P80,
            0x4 => ReverseOpenLoopDeceleration::P90,
            0x5 => ReverseOpenLoopDeceleration::P100,
            0x6 => ReverseOpenLoopDeceleration::P125,
            0x7 => ReverseOpenLoopDeceleration::P150,
            _ => panic!("Invalid value for ReverseOpenLoopDeceleration"),
        }
    }
}
