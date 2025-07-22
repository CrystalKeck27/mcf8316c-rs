use super::*;
use bitbybit::*;

#[bitfield(u32)]
#[derive(Debug, PartialEq, Eq)]
pub struct IntAlgo2 {
    /// Close loop acceleration when estimator is not yet fully aligned just
    /// after transition to closed loop
    #[bits(6..=9, rw)]
    pub cl_slow_acc: SlowClosedLoopAcceleration,
    /// Bus current slew rate during active braking
    #[bits(3..=5, rw)]
    pub active_brake_bus_current_slew_rate: ActiveBrakeBusCurrentSlewRate,
    /// Selection between MPET_IPD_CURRENT_LIMIT for IPD current
    /// limit, MPET_IPD_FREQ for IPD Repeat OR IPD_CURR_THR for
    /// IPD current limit, IPD_REPEAT for IPD Repeat.
    /// 0 = Configured parameters for normal motor operation,
    /// 1 = MPET specific parameters
    #[bit(2, rw)]
    pub mpet_ipd_select: bool,
    /// Selection between MPET_OPEN_LOOP_SLEW_RATE for slew
    /// rate, MPET_OPEN_LOOP_CURR_REF for current reference,
    /// MPET_OPEN_LOOP_SPEED_REF for speed reference OR
    /// OL_ACC_A1, OL_ACC_A2 for slew rate, open loop current
    /// reference for current reference and open to closed loop speed
    /// threshold for speed reference.
    /// 0 = Configured parameters for normal motor operation,
    /// 1 = MPET specific parameters
    #[bit(1, rw)]
    pub mpet_ke_meas_parameter_select: bool,
    /// IPD high resolution enable.
    /// 0 = Disable,
    /// 1 = Enable
    #[bit(0, rw)]
    pub ipd_high_resolution_en: bool,
}

impl Register for IntAlgo2 {
    const ADDRESS: u16 = INT_ALGO_2;
}

#[bitenum(u4, exhaustive = true)]
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, strum::Display)]
pub enum SlowClosedLoopAcceleration {
    /// 0.1Hz/s
    #[strum(to_string = "0.1Hz/s")]
    Hz0_1 = 0x0,
    /// 1.0Hz/s
    #[strum(to_string = "1.0Hz/s")]
    Hz1 = 0x1,
    /// 2.0Hz/s
    #[strum(to_string = "2.0Hz/s")]
    Hz2 = 0x2,
    /// 3.0Hz/s
    #[strum(to_string = "3.0Hz/s")]
    Hz3 = 0x3,
    /// 5.0Hz/s
    #[strum(to_string = "5.0Hz/s")]
    Hz5 = 0x4,
    /// 10.0Hz/s
    #[strum(to_string = "10.0Hz/s")]
    Hz10 = 0x5,
    /// 20.0Hz/s
    #[strum(to_string = "20.0Hz/s")]
    Hz20 = 0x6,
    /// 30.0Hz/s
    #[strum(to_string = "30.0Hz/s")]
    Hz30 = 0x7,
    /// 40.0Hz/s
    #[strum(to_string = "40.0Hz/s")]
    Hz40 = 0x8,
    /// 50.0Hz/s
    #[strum(to_string = "50.0Hz/s")]
    Hz50 = 0x9,
    /// 100.0Hz/s
    #[strum(to_string = "100.0Hz/s")]
    Hz100 = 0xA,
    /// 200.0Hz/s
    #[strum(to_string = "200.0Hz/s")]
    Hz200 = 0xB,
    /// 500.0Hz/s
    #[strum(to_string = "500.0Hz/s")]
    Hz500 = 0xC,
    /// 750.0Hz/s
    #[strum(to_string = "750.0Hz/s")]
    Hz750 = 0xD,
    /// 1000.0Hz/s
    #[strum(to_string = "1000.0Hz/s")]
    Hz1000 = 0xE,
    /// 2000.0Hz/s
    #[strum(to_string = "2000.0Hz/s")]
    Hz2000 = 0xF,
}

#[bitenum(u3, exhaustive = true)]
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, strum::Display)]
pub enum ActiveBrakeBusCurrentSlewRate {
    /// 10A/s
    #[strum(to_string = "10A/s")]
    A10 = 0x0,
    /// 50A/s
    #[strum(to_string = "50A/s")]
    A50 = 0x1,
    /// 100A/s
    #[strum(to_string = "100A/s")]
    A100 = 0x2,
    /// 250A/s
    #[strum(to_string = "250A/s")]
    A250 = 0x3,
    /// 500A/s
    #[strum(to_string = "500A/s")]
    A500 = 0x4,
    /// 1000A/s
    #[strum(to_string = "1000A/s")]
    A1000 = 0x5,
    /// 5000A/s
    #[strum(to_string = "5000A/s")]
    A5000 = 0x6,
    /// No Limit
    #[strum(to_string = "No Limit")]
    NoLimit = 0x7,
}
