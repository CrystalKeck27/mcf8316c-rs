use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IntAlgo2 {
    /// Close loop acceleration when estimator is not yet fully aligned just
    /// after transition to closed loop
    pub cl_slow_acc: SlowClosedLoopAcceleration,
    /// Bus current slew rate during active braking
    pub active_brake_bus_current_slew_rate: ActiveBrakeBusCurrentSlewRate,
    /// Selection between MPET_IPD_CURRENT_LIMIT for IPD current
    /// limit, MPET_IPD_FREQ for IPD Repeat OR IPD_CURR_THR for
    /// IPD current limit, IPD_REPEAT for IPD Repeat.
    /// 0 = Configured parameters for normal motor operation,
    /// 1 = MPET specific parameters
    pub mpet_ipd_select: bool,
    /// Selection between MPET_OPEN_LOOP_SLEW_RATE for slew
    /// rate, MPET_OPEN_LOOP_CURR_REF for current reference,
    /// MPET_OPEN_LOOP_SPEED_REF for speed reference OR
    /// OL_ACC_A1, OL_ACC_A2 for slew rate, open loop current
    /// reference for current reference and open to closed loop speed
    /// threshold for speed reference.
    /// 0 = Configured parameters for normal motor operation,
    /// 1 = MPET specific parameters
    pub mpet_ke_meas_parameter_select: bool,
    /// IPD high resolution enable.
    /// 0 = Disable,
    /// 1 = Enable
    pub ipd_high_resolution_en: bool,
}

impl Register for IntAlgo2 {
    const ADDRESS: u16 = INT_ALGO_2;
}

impl From<IntAlgo2> for u32 {
    fn from(config: IntAlgo2) -> Self {
        let mut value = 0;
        value |= (config.cl_slow_acc as u32) << 6;
        value |= (config.active_brake_bus_current_slew_rate as u32) << 3;
        value |= (config.mpet_ipd_select as u32) << 2;
        value |= (config.mpet_ke_meas_parameter_select as u32) << 1;
        value |= config.ipd_high_resolution_en as u32;
        value
    }
}

impl From<u32> for IntAlgo2 {
    fn from(value: u32) -> Self {
        IntAlgo2 {
            cl_slow_acc: SlowClosedLoopAcceleration::from(((value >> 6) & 0xF) as u8),
            active_brake_bus_current_slew_rate: ActiveBrakeBusCurrentSlewRate::from(((value >> 3) & 0x7) as u8),
            mpet_ipd_select: (value >> 2) & 0x1 != 0,
            mpet_ke_meas_parameter_select: (value >> 1) & 0x1 != 0,
            ipd_high_resolution_en: value & 0x1 != 0,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, strum::Display)]
#[repr(u8)]
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

impl From<u8> for SlowClosedLoopAcceleration {
    fn from(value: u8) -> Self {
        match value {
            0x0 => SlowClosedLoopAcceleration::Hz0_1,
            0x1 => SlowClosedLoopAcceleration::Hz1,
            0x2 => SlowClosedLoopAcceleration::Hz2,
            0x3 => SlowClosedLoopAcceleration::Hz3,
            0x4 => SlowClosedLoopAcceleration::Hz5,
            0x5 => SlowClosedLoopAcceleration::Hz10,
            0x6 => SlowClosedLoopAcceleration::Hz20,
            0x7 => SlowClosedLoopAcceleration::Hz30,
            0x8 => SlowClosedLoopAcceleration::Hz40,
            0x9 => SlowClosedLoopAcceleration::Hz50,
            0xA => SlowClosedLoopAcceleration::Hz100,
            0xB => SlowClosedLoopAcceleration::Hz200,
            0xC => SlowClosedLoopAcceleration::Hz500,
            0xD => SlowClosedLoopAcceleration::Hz750,
            0xE => SlowClosedLoopAcceleration::Hz1000,
            0xF => SlowClosedLoopAcceleration::Hz2000,
            _ => panic!("Invalid value for SlowClosedLoopAcceleration: {}", value),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, strum::Display)]
#[repr(u8)]
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

impl From<u8> for ActiveBrakeBusCurrentSlewRate {
    fn from(value: u8) -> Self {
        match value {
            0x0 => ActiveBrakeBusCurrentSlewRate::A10,
            0x1 => ActiveBrakeBusCurrentSlewRate::A50,
            0x2 => ActiveBrakeBusCurrentSlewRate::A100,
            0x3 => ActiveBrakeBusCurrentSlewRate::A250,
            0x4 => ActiveBrakeBusCurrentSlewRate::A500,
            0x5 => ActiveBrakeBusCurrentSlewRate::A1000,
            0x6 => ActiveBrakeBusCurrentSlewRate::A5000,
            0x7 => ActiveBrakeBusCurrentSlewRate::NoLimit,
            _ => panic!("Invalid value for ActiveBrakeBusCurrentSlewRate: {}", value),
        }
    }
}