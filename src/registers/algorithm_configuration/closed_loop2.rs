//! Section 7.7.1.6

use super::*;
use arbitrary_int::*;
use bitbybit::*;

/// Register to configure close loop settings2
#[bitfield(u32, debug, default = 0x0)]
#[derive(PartialEq, Eq)]
pub struct ClosedLoop2 {
    /// Motor stop options
    #[bits(28..=30, rw)]
    pub mtr_stop: Option<MotorStop>,
    /// Brake time during motor stop
    #[bits(24..=27, rw)]
    pub mtr_stop_brk_time: MotorStopBrakeTime,
    /// Speed threshold for active spin down (% of MAX_SPEED)
    #[bits(20..=23, rw)]
    pub act_spin_thr: PercentDecreasing,
    /// Speed threshold for BRAKE pin and motor stop options (Low side
    /// Braking or High Side Braking or Align Braking) (% of MAX_SPEED)
    #[bits(16..=19, rw)]
    pub brake_speed_threshold: PercentDecreasing,
    /// 8-bit value for motor phase resistance
    #[bits(8..=15, rw)]
    pub motor_res: MotorResistance,
    /// 8-bit value for motor phase inductance
    #[bits(0..=7, rw)]
    pub motor_ind: MotorInductance,
}

impl Register for ClosedLoop2 {
    const ADDRESS: u12 = CLOSED_LOOP2;

    fn value(&self) -> u32 {
        self.raw_value()
    }

    fn from_value(value: u32) -> Self {
        Self::new_with_raw_value(value)
    }
}

/// Motor stop options
#[bitenum(u3, exhaustive = false)]
#[derive(Debug, PartialEq, Eq, strum::Display)]
pub enum MotorStop {
    /// Hi-Z
    #[strum(serialize = "Hi-Z")]
    HiZ = 0x0,
    /// Not Applicable
    #[strum(serialize = "Not Applicable")]
    NotApplicable = 0x1,
    /// Low side braking
    #[strum(serialize = "Low Side Braking")]
    LowSideBraking = 0x2,
    /// High side braking
    #[strum(serialize = "High Side Braking")]
    HighSideBraking = 0x3,
    /// Active spin down
    #[strum(serialize = "Active Spin Down")]
    ActiveSpinDown = 0x4,
    /// Align braking
    #[strum(serialize = "Align Braking")]
    AlignBraking = 0x5,
    // Not Defined
}

/// Brake time during motor stop
#[bitenum(u4, exhaustive = true)]
#[derive(Debug, strum::Display)]
pub enum MotorStopBrakeTime {
    // Why is 1ms listed 5 different ways? Nobody knows.
    /// 1 ms
    #[strum(serialize = "1 ms")]
    Ms1_1 = 0x0,
    /// 1 ms
    #[strum(serialize = "1 ms")]
    Ms1_2 = 0x1,
    /// 1 ms
    #[strum(serialize = "1 ms")]
    Ms1_3 = 0x2,
    /// 1 ms
    #[strum(serialize = "1 ms")]
    Ms1_4 = 0x3,
    /// 1 ms
    #[strum(serialize = "1 ms")]
    Ms1_5 = 0x4,
    /// 5 ms
    #[strum(serialize = "5 ms")]
    Ms5 = 0x5,
    /// 10 ms
    #[strum(serialize = "10 ms")]
    Ms10 = 0x6,
    /// 50 ms
    #[strum(serialize = "50 ms")]
    Ms50 = 0x7,
    /// 100 ms
    #[strum(serialize = "100 ms")]
    Ms100 = 0x8,
    /// 250 ms
    #[strum(serialize = "250 ms")]
    Ms250 = 0x9,
    /// 500 ms
    #[strum(serialize = "500 ms")]
    Ms500 = 0xA,
    /// 1000 ms
    #[strum(serialize = "1000 ms")]
    Ms1000 = 0xB,
    /// 2500 ms
    #[strum(serialize = "2500 ms")]
    Ms2500 = 0xC,
    /// 5000 ms
    #[strum(serialize = "5000 ms")]
    Ms5000 = 0xD,
    /// 10000 ms
    #[strum(serialize = "10000 ms")]
    Ms10000 = 0xE,
    /// 15000 ms
    #[strum(serialize = "15000 ms")]
    Ms15000 = 0xF,
}

impl PartialEq for MotorStopBrakeTime {
    fn eq(&self, other: &Self) -> bool {
        // All 1ms variants (0x0..=0x4) are equal to each other
        let a = *self as u8;
        let b = *other as u8;
        (a <= 4 && b <= 4) || a == b
    }
}

impl Eq for MotorStopBrakeTime {}

impl PartialOrd for MotorStopBrakeTime {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for MotorStopBrakeTime {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        let a = *self as u8;
        let b = *other as u8;
        let a_val = if a <= 4 { 0 } else { a };
        let b_val = if b <= 4 { 0 } else { b };
        a_val.cmp(&b_val)
    }
}

/// A set of percentages in decreasing order.
#[bitenum(u4, exhaustive = true)]
#[derive(Debug, PartialEq, Eq, strum::Display)]
pub enum PercentDecreasing {
    /// 100%
    #[strum(serialize = "100%")]
    P100 = 0x0,
    /// 90%
    #[strum(serialize = "90%")]
    P90 = 0x1,
    /// 80%
    #[strum(serialize = "80%")]
    P80 = 0x2,
    /// 70%
    #[strum(serialize = "70%")]
    P70 = 0x3,
    /// 60%
    #[strum(serialize = "60%")]
    P60 = 0x4,
    /// 50%
    #[strum(serialize = "50%")]
    P50 = 0x5,
    /// 45%
    #[strum(serialize = "45%")]
    P45 = 0x6,
    /// 40%
    #[strum(serialize = "40%")]
    P40 = 0x7,
    /// 35%
    #[strum(serialize = "35%")]
    P35 = 0x8,
    /// 30%
    #[strum(serialize = "30%")]
    P30 = 0x9,
    /// 25%
    #[strum(serialize = "25%")]
    P25 = 0xA,
    /// 20%
    #[strum(serialize = "20%")]
    P20 = 0xB,
    /// 15%
    #[strum(serialize = "15%")]
    P15 = 0xC,
    /// 10%
    #[strum(serialize = "10%")]
    P10 = 0xD,
    /// 5%
    #[strum(serialize = "5%")]
    P5 = 0xE,
    /// 2.5%
    #[strum(serialize = "2.5%")]
    P2_5 = 0xF,
}

impl PartialOrd for PercentDecreasing {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for PercentDecreasing {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        // Lower enum value means higher percentage, so reverse the order
        let a = *self as u8;
        let b = *other as u8;
        b.cmp(&a)
    }
}

/// 8-bit values for motor phase inductance. See Table 7-2 for values of
/// phase resistance
#[bitenum(u8, exhaustive = true)]
#[derive(Debug, PartialEq, Eq, strum::Display)]
pub enum MotorResistance {
    /// Self Measurement (see Motor Parameter Extraction Tool (MPET))
    #[strum(serialize = "Self Measurement")]
    SelfMeasurement = 0x0,
    /// 0.006 Ω
    #[strum(serialize = "0.006 Ω")]
    R006 = 0x1,
    /// 0.007 Ω
    #[strum(serialize = "0.007 Ω")]
    R007 = 0x2,
    /// 0.008 Ω
    #[strum(serialize = "0.008 Ω")]
    R008 = 0x3,
    /// 0.009 Ω
    #[strum(serialize = "0.009 Ω")]
    R009 = 0x4,
    /// 0.010 Ω
    #[strum(serialize = "0.010 Ω")]
    R010 = 0x5,
    /// 0.011 Ω
    #[strum(serialize = "0.011 Ω")]
    R011 = 0x6,
    /// 0.012 Ω
    #[strum(serialize = "0.012 Ω")]
    R012 = 0x7,
    /// 0.013 Ω
    #[strum(serialize = "0.013 Ω")]
    R013 = 0x8,
    /// 0.014 Ω
    #[strum(serialize = "0.014 Ω")]
    R014 = 0x9,
    /// 0.015 Ω
    #[strum(serialize = "0.015 Ω")]
    R015 = 0xA,
    /// 0.016 Ω
    #[strum(serialize = "0.016 Ω")]
    R016 = 0xB,
    /// 0.017 Ω
    #[strum(serialize = "0.017 Ω")]
    R017 = 0xC,
    /// 0.018 Ω
    #[strum(serialize = "0.018 Ω")]
    R018 = 0xD,
    /// 0.019 Ω
    #[strum(serialize = "0.019 Ω")]
    R019 = 0xE,
    /// 0.020 Ω
    #[strum(serialize = "0.020 Ω")]
    R020 = 0xF,
    /// 0.022 Ω
    #[strum(serialize = "0.022 Ω")]
    R022 = 0x10,
    /// 0.024 Ω
    #[strum(serialize = "0.024 Ω")]
    R024 = 0x11,
    /// 0.026 Ω
    #[strum(serialize = "0.026 Ω")]
    R026 = 0x12,
    /// 0.028 Ω
    #[strum(serialize = "0.028 Ω")]
    R028 = 0x13,
    /// 0.030 Ω
    #[strum(serialize = "0.030 Ω")]
    R030 = 0x14,
    /// 0.032 Ω
    #[strum(serialize = "0.032 Ω")]
    R032 = 0x15,
    /// 0.034 Ω
    #[strum(serialize = "0.034 Ω")]
    R034 = 0x16,
    /// 0.036 Ω
    #[strum(serialize = "0.036 Ω")]
    R036 = 0x17,
    /// 0.038 Ω
    #[strum(serialize = "0.038 Ω")]
    R038 = 0x18,
    /// 0.040 Ω
    #[strum(serialize = "0.040 Ω")]
    R040 = 0x19,
    /// 0.042 Ω
    #[strum(serialize = "0.042 Ω")]
    R042 = 0x1A,
    /// 0.044 Ω
    #[strum(serialize = "0.044 Ω")]
    R044 = 0x1B,
    /// 0.046 Ω
    #[strum(serialize = "0.046 Ω")]
    R046 = 0x1C,
    /// 0.048 Ω
    #[strum(serialize = "0.048 Ω")]
    R048 = 0x1D,
    /// 0.050 Ω
    #[strum(serialize = "0.050 Ω")]
    R050 = 0x1E,
    /// 0.052 Ω
    #[strum(serialize = "0.052 Ω")]
    R052 = 0x1F,
    /// 0.054 Ω
    #[strum(serialize = "0.054 Ω")]
    R054 = 0x20,
    /// 0.056 Ω
    #[strum(serialize = "0.056 Ω")]
    R056 = 0x21,
    /// 0.058 Ω
    #[strum(serialize = "0.058 Ω")]
    R058 = 0x22,
    /// 0.060 Ω
    #[strum(serialize = "0.060 Ω")]
    R060 = 0x23,
    /// 0.062 Ω
    #[strum(serialize = "0.062 Ω")]
    R062 = 0x24,
    /// 0.064 Ω
    #[strum(serialize = "0.064 Ω")]
    R064 = 0x25,
    /// 0.066 Ω
    #[strum(serialize = "0.066 Ω")]
    R066 = 0x26,
    /// 0.068 Ω
    #[strum(serialize = "0.068 Ω")]
    R068 = 0x27,
    /// 0.070 Ω
    #[strum(serialize = "0.070 Ω")]
    R070 = 0x28,
    /// 0.072 Ω
    #[strum(serialize = "0.072 Ω")]
    R072 = 0x29,
    /// 0.074 Ω
    #[strum(serialize = "0.074 Ω")]
    R074 = 0x2A,
    /// 0.076 Ω
    #[strum(serialize = "0.076 Ω")]
    R076 = 0x2B,
    /// 0.078 Ω
    #[strum(serialize = "0.078 Ω")]
    R078 = 0x2C,
    /// 0.080 Ω
    #[strum(serialize = "0.080 Ω")]
    R080 = 0x2D,
    /// 0.082 Ω
    #[strum(serialize = "0.082 Ω")]
    R082 = 0x2E,
    /// 0.084 Ω
    #[strum(serialize = "0.084 Ω")]
    R084 = 0x2F,
    /// 0.086 Ω
    #[strum(serialize = "0.086 Ω")]
    R086 = 0x30,
    /// 0.088 Ω
    #[strum(serialize = "0.088 Ω")]
    R088 = 0x31,
    /// 0.090 Ω
    #[strum(serialize = "0.090 Ω")]
    R090 = 0x32,
    /// 0.092 Ω
    #[strum(serialize = "0.092 Ω")]
    R092 = 0x33,
    /// 0.094 Ω
    #[strum(serialize = "0.094 Ω")]
    R094 = 0x34,
    /// 0.096 Ω
    #[strum(serialize = "0.096 Ω")]
    R096 = 0x35,
    /// 0.098 Ω
    #[strum(serialize = "0.098 Ω")]
    R098 = 0x36,
    /// 0.100 Ω
    #[strum(serialize = "0.100 Ω")]
    R100 = 0x37,
    /// 0.105 Ω
    #[strum(serialize = "0.105 Ω")]
    R105 = 0x38,
    /// 0.110 Ω
    #[strum(serialize = "0.110 Ω")]
    R110 = 0x39,
    /// 0.115 Ω
    #[strum(serialize = "0.115 Ω")]
    R115 = 0x3A,
    /// 0.120 Ω
    #[strum(serialize = "0.120 Ω")]
    R120 = 0x3B,
    /// 0.125 Ω
    #[strum(serialize = "0.125 Ω")]
    R125 = 0x3C,
    /// 0.130 Ω
    #[strum(serialize = "0.130 Ω")]
    R130 = 0x3D,
    /// 0.135 Ω
    #[strum(serialize = "0.135 Ω")]
    R135 = 0x3E,
    /// 0.140 Ω
    #[strum(serialize = "0.140 Ω")]
    R140 = 0x3F,
    /// 0.145 Ω
    #[strum(serialize = "0.145 Ω")]
    R145 = 0x40,
    /// 0.150 Ω
    #[strum(serialize = "0.150 Ω")]
    R150 = 0x41,
    /// 0.155 Ω
    #[strum(serialize = "0.155 Ω")]
    R155 = 0x42,
    /// 0.160 Ω
    #[strum(serialize = "0.160 Ω")]
    R160 = 0x43,
    /// 0.165 Ω
    #[strum(serialize = "0.165 Ω")]
    R165 = 0x44,
    /// 0.170 Ω
    #[strum(serialize = "0.170 Ω")]
    R170 = 0x45,
    /// 0.175 Ω
    #[strum(serialize = "0.175 Ω")]
    R175 = 0x46,
    /// 0.180 Ω
    #[strum(serialize = "0.180 Ω")]
    R180 = 0x47,
    /// 0.185 Ω
    #[strum(serialize = "0.185 Ω")]
    R185 = 0x48,
    /// 0.190 Ω
    #[strum(serialize = "0.190 Ω")]
    R190 = 0x49,
    /// 0.195 Ω
    #[strum(serialize = "0.195 Ω")]
    R195 = 0x4A,
    /// 0.200 Ω
    #[strum(serialize = "0.200 Ω")]
    R200 = 0x4B,
    /// 0.205 Ω
    #[strum(serialize = "0.205 Ω")]
    R205 = 0x4C,
    /// 0.210 Ω
    #[strum(serialize = "0.210 Ω")]
    R210 = 0x4D,
    /// 0.215 Ω
    #[strum(serialize = "0.215 Ω")]
    R215 = 0x4E,
    /// 0.220 Ω
    #[strum(serialize = "0.220 Ω")]
    R220 = 0x4F,
    /// 0.225 Ω
    #[strum(serialize = "0.225 Ω")]
    R225 = 0x50,
    /// 0.230 Ω
    #[strum(serialize = "0.230 Ω")]
    R230 = 0x51,
    /// 0.235 Ω
    #[strum(serialize = "0.235 Ω")]
    R235 = 0x52,
    /// 0.240 Ω
    #[strum(serialize = "0.240 Ω")]
    R240 = 0x53,
    /// 0.245 Ω
    #[strum(serialize = "0.245 Ω")]
    R245 = 0x54,
    /// 0.250 Ω
    #[strum(serialize = "0.250 Ω")]
    R250 = 0x55,
    /// 0.255 Ω
    #[strum(serialize = "0.255 Ω")]
    R255 = 0x56,
    /// 0.260 Ω
    #[strum(serialize = "0.260 Ω")]
    R260 = 0x57,
    /// 0.265 Ω
    #[strum(serialize = "0.265 Ω")]
    R265 = 0x58,
    /// 0.270 Ω
    #[strum(serialize = "0.270 Ω")]
    R270 = 0x59,
    /// 0.275 Ω
    #[strum(serialize = "0.275 Ω")]
    R275 = 0x5A,
    /// 0.280 Ω
    #[strum(serialize = "0.280 Ω")]
    R280 = 0x5B,
    /// 0.285 Ω
    #[strum(serialize = "0.285 Ω")]
    R285 = 0x5C,
    /// 0.290 Ω
    #[strum(serialize = "0.290 Ω")]
    R290 = 0x5D,
    /// 0.295 Ω
    #[strum(serialize = "0.295 Ω")]
    R295 = 0x5E,
    /// 0.300 Ω
    #[strum(serialize = "0.300 Ω")]
    R300 = 0x5F,
    /// 0.305 Ω
    #[strum(serialize = "0.305 Ω")]
    R305 = 0x60,
    /// 0.310 Ω
    #[strum(serialize = "0.310 Ω")]
    R310 = 0x61,
    /// 0.315 Ω
    #[strum(serialize = "0.315 Ω")]
    R315 = 0x62,
    /// 0.320 Ω
    #[strum(serialize = "0.320 Ω")]
    R320 = 0x63,
    /// 0.325 Ω
    #[strum(serialize = "0.325 Ω")]
    R325 = 0x64,
    /// 0.330 Ω
    #[strum(serialize = "0.330 Ω")]
    R330 = 0x65,
    /// 0.335 Ω
    #[strum(serialize = "0.335 Ω")]
    R335 = 0x66,
    /// 0.340 Ω
    #[strum(serialize = "0.340 Ω")]
    R340 = 0x67,
    /// 0.345 Ω
    #[strum(serialize = "0.345 Ω")]
    R345 = 0x68,
    /// 0.350 Ω
    #[strum(serialize = "0.350 Ω")]
    R350 = 0x69,
    /// 0.355 Ω
    #[strum(serialize = "0.355 Ω")]
    R355 = 0x6A,
    /// 0.360 Ω
    #[strum(serialize = "0.360 Ω")]
    R360 = 0x6B,
    /// 0.365 Ω
    #[strum(serialize = "0.365 Ω")]
    R365 = 0x6C,
    /// 0.370 Ω
    #[strum(serialize = "0.370 Ω")]
    R370 = 0x6D,
    /// 0.375 Ω
    #[strum(serialize = "0.375 Ω")]
    R375 = 0x6E,
    /// 0.380 Ω
    #[strum(serialize = "0.380 Ω")]
    R380 = 0x6F,
    /// 0.385 Ω
    #[strum(serialize = "0.385 Ω")]
    R385 = 0x70,
    /// 0.390 Ω
    #[strum(serialize = "0.390 Ω")]
    R390 = 0x71,
    /// 0.395 Ω
    #[strum(serialize = "0.395 Ω")]
    R395 = 0x72,
    /// 0.400 Ω
    #[strum(serialize = "0.400 Ω")]
    R400 = 0x73,
    /// 0.405 Ω
    #[strum(serialize = "0.405 Ω")]
    R405 = 0x74,
    /// 0.410 Ω
    #[strum(serialize = "0.410 Ω")]
    R410 = 0x75,
    /// 0.415 Ω
    #[strum(serialize = "0.415 Ω")]
    R415 = 0x76,
    /// 0.420 Ω
    #[strum(serialize = "0.420 Ω")]
    R420 = 0x77,
    /// 0.425 Ω
    #[strum(serialize = "0.425 Ω")]
    R425 = 0x78,
    /// 0.430 Ω
    #[strum(serialize = "0.430 Ω")]
    R430 = 0x79,
    /// 0.435 Ω
    #[strum(serialize = "0.435 Ω")]
    R435 = 0x7A,
    /// 0.440 Ω
    #[strum(serialize = "0.440 Ω")]
    R440 = 0x7B,
    /// 0.445 Ω
    #[strum(serialize = "0.445 Ω")]
    R445 = 0x7C,
    /// 0.450 Ω
    #[strum(serialize = "0.450 Ω")]
    R450 = 0x7D,
    /// 0.455 Ω
    #[strum(serialize = "0.455 Ω")]
    R455 = 0x7E,
    /// 0.460 Ω
    #[strum(serialize = "0.460 Ω")]
    R460 = 0x7F,
    /// 0.465 Ω
    #[strum(serialize = "0.465 Ω")]
    R465 = 0x80,
    /// 0.470 Ω
    #[strum(serialize = "0.470 Ω")]
    R470 = 0x81,
    /// 0.475 Ω
    #[strum(serialize = "0.475 Ω")]
    R475 = 0x82,
    /// 0.480 Ω
    #[strum(serialize = "0.480 Ω")]
    R480 = 0x83,
    /// 0.485 Ω
    #[strum(serialize = "0.485 Ω")]
    R485 = 0x84,
    /// 0.490 Ω
    #[strum(serialize = "0.490 Ω")]
    R490 = 0x85,
    /// 0.495 Ω
    #[strum(serialize = "0.495 Ω")]
    R495 = 0x86,
    /// 0.500 Ω
    #[strum(serialize = "0.500 Ω")]
    R500 = 0x87,
    /// 0.510 Ω
    #[strum(serialize = "0.510 Ω")]
    R510 = 0x88,
    /// 0.520 Ω
    #[strum(serialize = "0.520 Ω")]
    R520 = 0x89,
    /// 0.530 Ω
    #[strum(serialize = "0.530 Ω")]
    R530 = 0x8A,
    /// 0.540 Ω
    #[strum(serialize = "0.540 Ω")]
    R540 = 0x8B,
    /// 0.550 Ω
    #[strum(serialize = "0.550 Ω")]
    R550 = 0x8C,
    /// 0.560 Ω
    #[strum(serialize = "0.560 Ω")]
    R560 = 0x8D,
    /// 0.570 Ω
    #[strum(serialize = "0.570 Ω")]
    R570 = 0x8E,
    /// 0.580 Ω
    #[strum(serialize = "0.580 Ω")]
    R580 = 0x8F,
    /// 0.590 Ω
    #[strum(serialize = "0.590 Ω")]
    R590 = 0x90,
    /// 0.600 Ω
    #[strum(serialize = "0.600 Ω")]
    R600 = 0x91,
    /// 0.610 Ω
    #[strum(serialize = "0.610 Ω")]
    R610 = 0x92,
    /// 0.620 Ω
    #[strum(serialize = "0.620 Ω")]
    R620 = 0x93,
    /// 0.630 Ω
    #[strum(serialize = "0.630 Ω")]
    R630 = 0x94,
    /// 0.640 Ω
    #[strum(serialize = "0.640 Ω")]
    R640 = 0x95,
    /// 0.650 Ω
    #[strum(serialize = "0.650 Ω")]
    R650 = 0x96,
    /// 0.660 Ω
    #[strum(serialize = "0.660 Ω")]
    R660 = 0x97,
    /// 0.670 Ω
    #[strum(serialize = "0.670 Ω")]
    R670 = 0x98,
    /// 0.680 Ω
    #[strum(serialize = "0.680 Ω")]
    R680 = 0x99,
    /// 0.690 Ω
    #[strum(serialize = "0.690 Ω")]
    R690 = 0x9A,
    /// 0.700 Ω
    #[strum(serialize = "0.700 Ω")]
    R700 = 0x9B,
    /// 0.720 Ω
    #[strum(serialize = "0.720 Ω")]
    R720 = 0x9C,
    /// 0.740 Ω
    #[strum(serialize = "0.740 Ω")]
    R740 = 0x9D,
    /// 0.760 Ω
    #[strum(serialize = "0.760 Ω")]
    R760 = 0x9E,
    /// 0.780 Ω
    #[strum(serialize = "0.780 Ω")]
    R780 = 0x9F,
    /// 0.800 Ω
    #[strum(serialize = "0.800 Ω")]
    R800 = 0xA0,
    /// 0.820 Ω
    #[strum(serialize = "0.820 Ω")]
    R820 = 0xA1,
    /// 0.840 Ω
    #[strum(serialize = "0.840 Ω")]
    R840 = 0xA2,
    /// 0.860 Ω
    #[strum(serialize = "0.860 Ω")]
    R860 = 0xA3,
    /// 0.880 Ω
    #[strum(serialize = "0.880 Ω")]
    R880 = 0xA4,
    /// 0.900 Ω
    #[strum(serialize = "0.900 Ω")]
    R900 = 0xA5,
    /// 0.920 Ω
    #[strum(serialize = "0.920 Ω")]
    R920 = 0xA6,
    /// 0.940 Ω
    #[strum(serialize = "0.940 Ω")]
    R940 = 0xA7,
    /// 0.960 Ω
    #[strum(serialize = "0.960 Ω")]
    R960 = 0xA8,
    /// 0.980 Ω
    #[strum(serialize = "0.980 Ω")]
    R980 = 0xA9,
    /// 1.000 Ω
    #[strum(serialize = "1.000 Ω")]
    R1000 = 0xAA,
    /// 1.050 Ω
    #[strum(serialize = "1.050 Ω")]
    R1050 = 0xAB,
    /// 1.100 Ω
    #[strum(serialize = "1.100 Ω")]
    R1100 = 0xAC,
    /// 1.150 Ω
    #[strum(serialize = "1.150 Ω")]
    R1150 = 0xAD,
    /// 1.200 Ω
    #[strum(serialize = "1.200 Ω")]
    R1200 = 0xAE,
    /// 1.250 Ω
    #[strum(serialize = "1.250 Ω")]
    R1250 = 0xAF,
    /// 1.300 Ω
    #[strum(serialize = "1.300 Ω")]
    R1300 = 0xB0,
    /// 1.350 Ω
    #[strum(serialize = "1.350 Ω")]
    R1350 = 0xB1,
    /// 1.400 Ω
    #[strum(serialize = "1.400 Ω")]
    R1400 = 0xB2,
    /// 1.450 Ω
    #[strum(serialize = "1.450 Ω")]
    R1450 = 0xB3,
    /// 1.500 Ω
    #[strum(serialize = "1.500 Ω")]
    R1500 = 0xB4,
    /// 1.550 Ω
    #[strum(serialize = "1.550 Ω")]
    R1550 = 0xB5,
    /// 1.600 Ω
    #[strum(serialize = "1.600 Ω")]
    R1600 = 0xB6,
    /// 1.650 Ω
    #[strum(serialize = "1.650 Ω")]
    R1650 = 0xB7,
    /// 1.700 Ω
    #[strum(serialize = "1.700 Ω")]
    R1700 = 0xB8,
    /// 1.750 Ω
    #[strum(serialize = "1.750 Ω")]
    R1750 = 0xB9,
    /// 1.800 Ω
    #[strum(serialize = "1.800 Ω")]
    R1800 = 0xBA,
    /// 1.850 Ω
    #[strum(serialize = "1.850 Ω")]
    R1850 = 0xBB,
    /// 1.900 Ω
    #[strum(serialize = "1.900 Ω")]
    R1900 = 0xBC,
    /// 1.950 Ω
    #[strum(serialize = "1.950 Ω")]
    R1950 = 0xBD,
    /// 2.000 Ω
    #[strum(serialize = "2.000 Ω")]
    R2000 = 0xBE,
    /// 2.050 Ω
    #[strum(serialize = "2.050 Ω")]
    R2050 = 0xBF,
    /// 2.100 Ω
    #[strum(serialize = "2.100 Ω")]
    R2100 = 0xC0,
    /// 2.200 Ω
    #[strum(serialize = "2.200 Ω")]
    R2200 = 0xC1,
    /// 2.300 Ω
    #[strum(serialize = "2.300 Ω")]
    R2300 = 0xC2,
    /// 2.400 Ω
    #[strum(serialize = "2.400 Ω")]
    R2400 = 0xC3,
    /// 2.500 Ω
    #[strum(serialize = "2.500 Ω")]
    R2500 = 0xC4,
    /// 2.600 Ω
    #[strum(serialize = "2.600 Ω")]
    R2600 = 0xC5,
    /// 2.700 Ω
    #[strum(serialize = "2.700 Ω")]
    R2700 = 0xC6,
    /// 2.800 Ω
    #[strum(serialize = "2.800 Ω")]
    R2800 = 0xC7,
    /// 2.900 Ω
    #[strum(serialize = "2.900 Ω")]
    R2900 = 0xC8,
    /// 3.000 Ω
    #[strum(serialize = "3.000 Ω")]
    R3000 = 0xC9,
    /// 3.200 Ω
    #[strum(serialize = "3.200 Ω")]
    R3200 = 0xCA,
    /// 3.400 Ω
    #[strum(serialize = "3.400 Ω")]
    R3400 = 0xCB,
    /// 3.600 Ω
    #[strum(serialize = "3.600 Ω")]
    R3600 = 0xCC,
    /// 3.800 Ω
    #[strum(serialize = "3.800 Ω")]
    R3800 = 0xCD,
    /// 4.000 Ω
    #[strum(serialize = "4.000 Ω")]
    R4000 = 0xCE,
    /// 4.200 Ω
    #[strum(serialize = "4.200 Ω")]
    R4200 = 0xCF,
    /// 4.400 Ω
    #[strum(serialize = "4.400 Ω")]
    R4400 = 0xD0,
    /// 4.600 Ω
    #[strum(serialize = "4.600 Ω")]
    R4600 = 0xD1,
    /// 4.800 Ω
    #[strum(serialize = "4.800 Ω")]
    R4800 = 0xD2,
    /// 5.000 Ω
    #[strum(serialize = "5.000 Ω")]
    R5000 = 0xD3,
    /// 5.200 Ω
    #[strum(serialize = "5.200 Ω")]
    R5200 = 0xD4,
    /// 5.400 Ω
    #[strum(serialize = "5.400 Ω")]
    R5400 = 0xD5,
    /// 5.600 Ω
    #[strum(serialize = "5.600 Ω")]
    R5600 = 0xD6,
    /// 5.800 Ω
    #[strum(serialize = "5.800 Ω")]
    R5800 = 0xD7,
    /// 6.000 Ω
    #[strum(serialize = "6.000 Ω")]
    R6000 = 0xD8,
    /// 6.200 Ω
    #[strum(serialize = "6.200 Ω")]
    R6200 = 0xD9,
    /// 6.400 Ω
    #[strum(serialize = "6.400 Ω")]
    R6400 = 0xDA,
    /// 6.600 Ω
    #[strum(serialize = "6.600 Ω")]
    R6600 = 0xDB,
    /// 6.800 Ω
    #[strum(serialize = "6.800 Ω")]
    R6800 = 0xDC,
    /// 7.000 Ω
    #[strum(serialize = "7.000 Ω")]
    R7000 = 0xDD,
    /// 7.200 Ω
    #[strum(serialize = "7.200 Ω")]
    R7200 = 0xDE,
    /// 7.400 Ω
    #[strum(serialize = "7.400 Ω")]
    R7400 = 0xDF,
    /// 7.600 Ω
    #[strum(serialize = "7.600 Ω")]
    R7600 = 0xE0,
    /// 7.800 Ω
    #[strum(serialize = "7.800 Ω")]
    R7800 = 0xE1,
    /// 8.000 Ω
    #[strum(serialize = "8.000 Ω")]
    R8000 = 0xE2,
    /// 8.200 Ω
    #[strum(serialize = "8.200 Ω")]
    R8200 = 0xE3,
    /// 8.400 Ω
    #[strum(serialize = "8.400 Ω")]
    R8400 = 0xE4,
    /// 8.600 Ω
    #[strum(serialize = "8.600 Ω")]
    R8600 = 0xE5,
    /// 8.800 Ω
    #[strum(serialize = "8.800 Ω")]
    R8800 = 0xE6,
    /// 9.000 Ω
    #[strum(serialize = "9.000 Ω")]
    R9000 = 0xE7,
    /// 9.200 Ω
    #[strum(serialize = "9.200 Ω")]
    R9200 = 0xE8,
    /// 9.400 Ω
    #[strum(serialize = "9.400 Ω")]
    R9400 = 0xE9,
    /// 9.600 Ω
    #[strum(serialize = "9.600 Ω")]
    R9600 = 0xEA,
    /// 9.800 Ω
    #[strum(serialize = "9.800 Ω")]
    R9800 = 0xEB,
    /// 10.000 Ω
    #[strum(serialize = "10.000 Ω")]
    R10000 = 0xEC,
    /// 10.500 Ω
    #[strum(serialize = "10.500 Ω")]
    R10500 = 0xED,
    /// 11.000 Ω
    #[strum(serialize = "11.000 Ω")]
    R11000 = 0xEE,
    /// 11.500 Ω
    #[strum(serialize = "11.500 Ω")]
    R11500 = 0xEF,
    /// 12.000 Ω
    #[strum(serialize = "12.000 Ω")]
    R12000 = 0xF0,
    /// 12.500 Ω
    #[strum(serialize = "12.500 Ω")]
    R12500 = 0xF1,
    /// 13.000 Ω
    #[strum(serialize = "13.000 Ω")]
    R13000 = 0xF2,
    /// 13.500 Ω
    #[strum(serialize = "13.500 Ω")]
    R13500 = 0xF3,
    /// 14.000 Ω
    #[strum(serialize = "14.000 Ω")]
    R14000 = 0xF4,
    /// 14.500 Ω
    #[strum(serialize = "14.500 Ω")]
    R14500 = 0xF5,
    /// 15.000 Ω
    #[strum(serialize = "15.000 Ω")]
    R15000 = 0xF6,
    /// 15.500 Ω
    #[strum(serialize = "15.500 Ω")]
    R15500 = 0xF7,
    /// 16.000 Ω
    #[strum(serialize = "16.000 Ω")]
    R16000 = 0xF8,
    /// 16.500 Ω
    #[strum(serialize = "16.500 Ω")]
    R16500 = 0xF9,
    /// 17.000 Ω
    #[strum(serialize = "17.000 Ω")]
    R17000 = 0xFA,
    /// 17.500 Ω
    #[strum(serialize = "17.500 Ω")]
    R17500 = 0xFB,
    /// 18.000 Ω
    #[strum(serialize = "18.000 Ω")]
    R18000 = 0xFC,
    /// 18.500 Ω
    #[strum(serialize = "18.500 Ω")]
    R18500 = 0xFD,
    /// 19.000 Ω
    #[strum(serialize = "19.000 Ω")]
    R19000 = 0xFE,
    /// 20.000 Ω
    #[strum(serialize = "20.000 Ω")]
    R20000 = 0xFF,
}

impl From<u8> for MotorResistance {
    fn from(value: u8) -> Self {
        // Safety: MotorResistance has a valid representation for all u8 values
        unsafe { core::mem::transmute(value) }
    }
}

impl PartialOrd for MotorResistance {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        if *self == MotorResistance::SelfMeasurement || *other == MotorResistance::SelfMeasurement {
            None
        } else {
            (*self as u8).partial_cmp(&(*other as u8))
        }
    }
}

/// 8-bit values for motor phase inductance. See Table 7-3 for values of
/// phase inductance
#[bitenum(u8, exhaustive = true)]
#[derive(Debug, PartialEq, Eq, strum::Display)]
pub enum MotorInductance {
    /// Self Measurement (see Motor Parameter Extraction Tool (MPET))
    #[strum(serialize = "Self Measurement")]
    SelfMeasurement = 0x0,
    /// 0.006 mH
    #[strum(serialize = "0.006 mH")]
    L006 = 0x1,
    /// 0.007 mH
    #[strum(serialize = "0.007 mH")]
    L007 = 0x2,
    /// 0.008 mH
    #[strum(serialize = "0.008 mH")]
    L008 = 0x3,
    /// 0.009 mH
    #[strum(serialize = "0.009 mH")]
    L009 = 0x4,
    /// 0.010 mH
    #[strum(serialize = "0.010 mH")]
    L010 = 0x5,
    /// 0.011 mH
    #[strum(serialize = "0.011 mH")]
    L011 = 0x6,
    /// 0.012 mH
    #[strum(serialize = "0.012 mH")]
    L012 = 0x7,
    /// 0.013 mH
    #[strum(serialize = "0.013 mH")]
    L013 = 0x8,
    /// 0.014 mH
    #[strum(serialize = "0.014 mH")]
    L014 = 0x9,
    /// 0.015 mH
    #[strum(serialize = "0.015 mH")]
    L015 = 0xA,
    /// 0.016 mH
    #[strum(serialize = "0.016 mH")]
    L016 = 0xB,
    /// 0.017 mH
    #[strum(serialize = "0.017 mH")]
    L017 = 0xC,
    /// 0.018 mH
    #[strum(serialize = "0.018 mH")]
    L018 = 0xD,
    /// 0.019 mH
    #[strum(serialize = "0.019 mH")]
    L019 = 0xE,
    /// 0.020 mH
    #[strum(serialize = "0.020 mH")]
    L020 = 0xF,
    /// 0.022 mH
    #[strum(serialize = "0.022 mH")]
    L022 = 0x10,
    /// 0.024 mH
    #[strum(serialize = "0.024 mH")]
    L024 = 0x11,
    /// 0.026 mH
    #[strum(serialize = "0.026 mH")]
    L026 = 0x12,
    /// 0.028 mH
    #[strum(serialize = "0.028 mH")]
    L028 = 0x13,
    /// 0.030 mH
    #[strum(serialize = "0.030 mH")]
    L030 = 0x14,
    /// 0.032 mH
    #[strum(serialize = "0.032 mH")]
    L032 = 0x15,
    /// 0.034 mH
    #[strum(serialize = "0.034 mH")]
    L034 = 0x16,
    /// 0.036 mH
    #[strum(serialize = "0.036 mH")]
    L036 = 0x17,
    /// 0.038 mH
    #[strum(serialize = "0.038 mH")]
    L038 = 0x18,
    /// 0.040 mH
    #[strum(serialize = "0.040 mH")]
    L040 = 0x19,
    /// 0.042 mH
    #[strum(serialize = "0.042 mH")]
    L042 = 0x1A,
    /// 0.044 mH
    #[strum(serialize = "0.044 mH")]
    L044 = 0x1B,
    /// 0.046 mH
    #[strum(serialize = "0.046 mH")]
    L046 = 0x1C,
    /// 0.048 mH
    #[strum(serialize = "0.048 mH")]
    L048 = 0x1D,
    /// 0.050 mH
    #[strum(serialize = "0.050 mH")]
    L050 = 0x1E,
    /// 0.052 mH
    #[strum(serialize = "0.052 mH")]
    L052 = 0x1F,
    /// 0.054 mH
    #[strum(serialize = "0.054 mH")]
    L054 = 0x20,
    /// 0.056 mH
    #[strum(serialize = "0.056 mH")]
    L056 = 0x21,
    /// 0.058 mH
    #[strum(serialize = "0.058 mH")]
    L058 = 0x22,
    /// 0.060 mH
    #[strum(serialize = "0.060 mH")]
    L060 = 0x23,
    /// 0.062 mH
    #[strum(serialize = "0.062 mH")]
    L062 = 0x24,
    /// 0.064 mH
    #[strum(serialize = "0.064 mH")]
    L064 = 0x25,
    /// 0.066 mH
    #[strum(serialize = "0.066 mH")]
    L066 = 0x26,
    /// 0.068 mH
    #[strum(serialize = "0.068 mH")]
    L068 = 0x27,
    /// 0.070 mH
    #[strum(serialize = "0.070 mH")]
    L070 = 0x28,
    /// 0.072 mH
    #[strum(serialize = "0.072 mH")]
    L072 = 0x29,
    /// 0.074 mH
    #[strum(serialize = "0.074 mH")]
    L074 = 0x2A,
    /// 0.076 mH
    #[strum(serialize = "0.076 mH")]
    L076 = 0x2B,
    /// 0.078 mH
    #[strum(serialize = "0.078 mH")]
    L078 = 0x2C,
    /// 0.080 mH
    #[strum(serialize = "0.080 mH")]
    L080 = 0x2D,
    /// 0.082 mH
    #[strum(serialize = "0.082 mH")]
    L082 = 0x2E,
    /// 0.084 mH
    #[strum(serialize = "0.084 mH")]
    L084 = 0x2F,
    /// 0.086 mH
    #[strum(serialize = "0.086 mH")]
    L086 = 0x30,
    /// 0.088 mH
    #[strum(serialize = "0.088 mH")]
    L088 = 0x31,
    /// 0.090 mH
    #[strum(serialize = "0.090 mH")]
    L090 = 0x32,
    /// 0.092 mH
    #[strum(serialize = "0.092 mH")]
    L092 = 0x33,
    /// 0.094 mH
    #[strum(serialize = "0.094 mH")]
    L094 = 0x34,
    /// 0.096 mH
    #[strum(serialize = "0.096 mH")]
    L096 = 0x35,
    /// 0.098 mH
    #[strum(serialize = "0.098 mH")]
    L098 = 0x36,
    /// 0.100 mH
    #[strum(serialize = "0.100 mH")]
    L100 = 0x37,
    /// 0.105 mH
    #[strum(serialize = "0.105 mH")]
    L105 = 0x38,
    /// 0.110 mH
    #[strum(serialize = "0.110 mH")]
    L110 = 0x39,
    /// 0.115 mH
    #[strum(serialize = "0.115 mH")]
    L115 = 0x3A,
    /// 0.120 mH
    #[strum(serialize = "0.120 mH")]
    L120 = 0x3B,
    /// 0.125 mH
    #[strum(serialize = "0.125 mH")]
    L125 = 0x3C,
    /// 0.130 mH
    #[strum(serialize = "0.130 mH")]
    L130 = 0x3D,
    /// 0.135 mH
    #[strum(serialize = "0.135 mH")]
    L135 = 0x3E,
    /// 0.140 mH
    #[strum(serialize = "0.140 mH")]
    L140 = 0x3F,
    /// 0.145 mH
    #[strum(serialize = "0.145 mH")]
    L145 = 0x40,
    /// 0.150 mH
    #[strum(serialize = "0.150 mH")]
    L150 = 0x41,
    /// 0.155 mH
    #[strum(serialize = "0.155 mH")]
    L155 = 0x42,
    /// 0.160 mH
    #[strum(serialize = "0.160 mH")]
    L160 = 0x43,
    /// 0.165 mH
    #[strum(serialize = "0.165 mH")]
    L165 = 0x44,
    /// 0.170 mH
    #[strum(serialize = "0.170 mH")]
    L170 = 0x45,
    /// 0.175 mH
    #[strum(serialize = "0.175 mH")]
    L175 = 0x46,
    /// 0.180 mH
    #[strum(serialize = "0.180 mH")]
    L180 = 0x47,
    /// 0.185 mH
    #[strum(serialize = "0.185 mH")]
    L185 = 0x48,
    /// 0.190 mH
    #[strum(serialize = "0.190 mH")]
    L190 = 0x49,
    /// 0.195 mH
    #[strum(serialize = "0.195 mH")]
    L195 = 0x4A,
    /// 0.200 mH
    #[strum(serialize = "0.200 mH")]
    L200 = 0x4B,
    /// 0.205 mH
    #[strum(serialize = "0.205 mH")]
    L205 = 0x4C,
    /// 0.210 mH
    #[strum(serialize = "0.210 mH")]
    L210 = 0x4D,
    /// 0.215 mH
    #[strum(serialize = "0.215 mH")]
    L215 = 0x4E,
    /// 0.220 mH
    #[strum(serialize = "0.220 mH")]
    L220 = 0x4F,
    /// 0.225 mH
    #[strum(serialize = "0.225 mH")]
    L225 = 0x50,
    /// 0.230 mH
    #[strum(serialize = "0.230 mH")]
    L230 = 0x51,
    /// 0.235 mH
    #[strum(serialize = "0.235 mH")]
    L235 = 0x52,
    /// 0.240 mH
    #[strum(serialize = "0.240 mH")]
    L240 = 0x53,
    /// 0.245 mH
    #[strum(serialize = "0.245 mH")]
    L245 = 0x54,
    /// 0.250 mH
    #[strum(serialize = "0.250 mH")]
    L250 = 0x55,
    /// 0.255 mH
    #[strum(serialize = "0.255 mH")]
    L255 = 0x56,
    /// 0.260 mH
    #[strum(serialize = "0.260 mH")]
    L260 = 0x57,
    /// 0.265 mH
    #[strum(serialize = "0.265 mH")]
    L265 = 0x58,
    /// 0.270 mH
    #[strum(serialize = "0.270 mH")]
    L270 = 0x59,
    /// 0.275 mH
    #[strum(serialize = "0.275 mH")]
    L275 = 0x5A,
    /// 0.280 mH
    #[strum(serialize = "0.280 mH")]
    L280 = 0x5B,
    /// 0.285 mH
    #[strum(serialize = "0.285 mH")]
    L285 = 0x5C,
    /// 0.290 mH
    #[strum(serialize = "0.290 mH")]
    L290 = 0x5D,
    /// 0.295 mH
    #[strum(serialize = "0.295 mH")]
    L295 = 0x5E,
    /// 0.300 mH
    #[strum(serialize = "0.300 mH")]
    L300 = 0x5F,
    /// 0.305 mH
    #[strum(serialize = "0.305 mH")]
    L305 = 0x60,
    /// 0.310 mH
    #[strum(serialize = "0.310 mH")]
    L310 = 0x61,
    /// 0.315 mH
    #[strum(serialize = "0.315 mH")]
    L315 = 0x62,
    /// 0.320 mH
    #[strum(serialize = "0.320 mH")]
    L320 = 0x63,
    /// 0.325 mH
    #[strum(serialize = "0.325 mH")]
    L325 = 0x64,
    /// 0.330 mH
    #[strum(serialize = "0.330 mH")]
    L330 = 0x65,
    /// 0.335 mH
    #[strum(serialize = "0.335 mH")]
    L335 = 0x66,
    /// 0.340 mH
    #[strum(serialize = "0.340 mH")]
    L340 = 0x67,
    /// 0.345 mH
    #[strum(serialize = "0.345 mH")]
    L345 = 0x68,
    /// 0.350 mH
    #[strum(serialize = "0.350 mH")]
    L350 = 0x69,
    /// 0.355 mH
    #[strum(serialize = "0.355 mH")]
    L355 = 0x6A,
    /// 0.360 mH
    #[strum(serialize = "0.360 mH")]
    L360 = 0x6B,
    /// 0.365 mH
    #[strum(serialize = "0.365 mH")]
    L365 = 0x6C,
    /// 0.370 mH
    #[strum(serialize = "0.370 mH")]
    L370 = 0x6D,
    /// 0.375 mH
    #[strum(serialize = "0.375 mH")]
    L375 = 0x6E,
    /// 0.380 mH
    #[strum(serialize = "0.380 mH")]
    L380 = 0x6F,
    /// 0.385 mH
    #[strum(serialize = "0.385 mH")]
    L385 = 0x70,
    /// 0.390 mH
    #[strum(serialize = "0.390 mH")]
    L390 = 0x71,
    /// 0.395 mH
    #[strum(serialize = "0.395 mH")]
    L395 = 0x72,
    /// 0.400 mH
    #[strum(serialize = "0.400 mH")]
    L400 = 0x73,
    /// 0.405 mH
    #[strum(serialize = "0.405 mH")]
    L405 = 0x74,
    /// 0.410 mH
    #[strum(serialize = "0.410 mH")]
    L410 = 0x75,
    /// 0.415 mH
    #[strum(serialize = "0.415 mH")]
    L415 = 0x76,
    /// 0.420 mH
    #[strum(serialize = "0.420 mH")]
    L420 = 0x77,
    /// 0.425 mH
    #[strum(serialize = "0.425 mH")]
    L425 = 0x78,
    /// 0.430 mH
    #[strum(serialize = "0.430 mH")]
    L430 = 0x79,
    /// 0.435 mH
    #[strum(serialize = "0.435 mH")]
    L435 = 0x7A,
    /// 0.440 mH
    #[strum(serialize = "0.440 mH")]
    L440 = 0x7B,
    /// 0.445 mH
    #[strum(serialize = "0.445 mH")]
    L445 = 0x7C,
    /// 0.450 mH
    #[strum(serialize = "0.450 mH")]
    L450 = 0x7D,
    /// 0.455 mH
    #[strum(serialize = "0.455 mH")]
    L455 = 0x7E,
    /// 0.460 mH
    #[strum(serialize = "0.460 mH")]
    L460 = 0x7F,
    /// 0.465 mH
    #[strum(serialize = "0.465 mH")]
    L465 = 0x80,
    /// 0.470 mH
    #[strum(serialize = "0.470 mH")]
    L470 = 0x81,
    /// 0.475 mH
    #[strum(serialize = "0.475 mH")]
    L475 = 0x82,
    /// 0.480 mH
    #[strum(serialize = "0.480 mH")]
    L480 = 0x83,
    /// 0.485 mH
    #[strum(serialize = "0.485 mH")]
    L485 = 0x84,
    /// 0.490 mH
    #[strum(serialize = "0.490 mH")]
    L490 = 0x85,
    /// 0.495 mH
    #[strum(serialize = "0.495 mH")]
    L495 = 0x86,
    /// 0.500 mH
    #[strum(serialize = "0.500 mH")]
    L500 = 0x87,
    /// 0.510 mH
    #[strum(serialize = "0.510 mH")]
    L510 = 0x88,
    /// 0.520 mH
    #[strum(serialize = "0.520 mH")]
    L520 = 0x89,
    /// 0.530 mH
    #[strum(serialize = "0.530 mH")]
    L530 = 0x8A,
    /// 0.540 mH
    #[strum(serialize = "0.540 mH")]
    L540 = 0x8B,
    /// 0.550 mH
    #[strum(serialize = "0.550 mH")]
    L550 = 0x8C,
    /// 0.560 mH
    #[strum(serialize = "0.560 mH")]
    L560 = 0x8D,
    /// 0.570 mH
    #[strum(serialize = "0.570 mH")]
    L570 = 0x8E,
    /// 0.580 mH
    #[strum(serialize = "0.580 mH")]
    L580 = 0x8F,
    /// 0.590 mH
    #[strum(serialize = "0.590 mH")]
    L590 = 0x90,
    /// 0.600 mH
    #[strum(serialize = "0.600 mH")]
    L600 = 0x91,
    /// 0.610 mH
    #[strum(serialize = "0.610 mH")]
    L610 = 0x92,
    /// 0.620 mH
    #[strum(serialize = "0.620 mH")]
    L620 = 0x93,
    /// 0.630 mH
    #[strum(serialize = "0.630 mH")]
    L630 = 0x94,
    /// 0.640 mH
    #[strum(serialize = "0.640 mH")]
    L640 = 0x95,
    /// 0.650 mH
    #[strum(serialize = "0.650 mH")]
    L650 = 0x96,
    /// 0.660 mH
    #[strum(serialize = "0.660 mH")]
    L660 = 0x97,
    /// 0.670 mH
    #[strum(serialize = "0.670 mH")]
    L670 = 0x98,
    /// 0.680 mH
    #[strum(serialize = "0.680 mH")]
    L680 = 0x99,
    /// 0.690 mH
    #[strum(serialize = "0.690 mH")]
    L690 = 0x9A,
    /// 0.700 mH
    #[strum(serialize = "0.700 mH")]
    L700 = 0x9B,
    /// 0.720 mH
    #[strum(serialize = "0.720 mH")]
    L720 = 0x9C,
    /// 0.740 mH
    #[strum(serialize = "0.740 mH")]
    L740 = 0x9D,
    /// 0.760 mH
    #[strum(serialize = "0.760 mH")]
    L760 = 0x9E,
    /// 0.780 mH
    #[strum(serialize = "0.780 mH")]
    L780 = 0x9F,
    /// 0.800 mH
    #[strum(serialize = "0.800 mH")]
    L800 = 0xA0,
    /// 0.820 mH
    #[strum(serialize = "0.820 mH")]
    L820 = 0xA1,
    /// 0.840 mH
    #[strum(serialize = "0.840 mH")]
    L840 = 0xA2,
    /// 0.860 mH
    #[strum(serialize = "0.860 mH")]
    L860 = 0xA3,
    /// 0.880 mH
    #[strum(serialize = "0.880 mH")]
    L880 = 0xA4,
    /// 0.900 mH
    #[strum(serialize = "0.900 mH")]
    L900 = 0xA5,
    /// 0.920 mH
    #[strum(serialize = "0.920 mH")]
    L920 = 0xA6,
    /// 0.940 mH
    #[strum(serialize = "0.940 mH")]
    L940 = 0xA7,
    /// 0.960 mH
    #[strum(serialize = "0.960 mH")]
    L960 = 0xA8,
    /// 0.980 mH
    #[strum(serialize = "0.980 mH")]
    L980 = 0xA9,
    /// 1.000 mH
    #[strum(serialize = "1.000 mH")]
    L1000 = 0xAA,
    /// 1.050 mH
    #[strum(serialize = "1.050 mH")]
    L1050 = 0xAB,
    /// 1.100 mH
    #[strum(serialize = "1.100 mH")]
    L1100 = 0xAC,
    /// 1.150 mH
    #[strum(serialize = "1.150 mH")]
    L1150 = 0xAD,
    /// 1.200 mH
    #[strum(serialize = "1.200 mH")]
    L1200 = 0xAE,
    /// 1.250 mH
    #[strum(serialize = "1.250 mH")]
    L1250 = 0xAF,
    /// 1.300 mH
    #[strum(serialize = "1.300 mH")]
    L1300 = 0xB0,
    /// 1.350 mH
    #[strum(serialize = "1.350 mH")]
    L1350 = 0xB1,
    /// 1.400 mH
    #[strum(serialize = "1.400 mH")]
    L1400 = 0xB2,
    /// 1.450 mH
    #[strum(serialize = "1.450 mH")]
    L1450 = 0xB3,
    /// 1.500 mH
    #[strum(serialize = "1.500 mH")]
    L1500 = 0xB4,
    /// 1.550 mH
    #[strum(serialize = "1.550 mH")]
    L1550 = 0xB5,
    /// 1.600 mH
    #[strum(serialize = "1.600 mH")]
    L1600 = 0xB6,
    /// 1.650 mH
    #[strum(serialize = "1.650 mH")]
    L1650 = 0xB7,
    /// 1.700 mH
    #[strum(serialize = "1.700 mH")]
    L1700 = 0xB8,
    /// 1.750 mH
    #[strum(serialize = "1.750 mH")]
    L1750 = 0xB9,
    /// 1.800 mH
    #[strum(serialize = "1.800 mH")]
    L1800 = 0xBA,
    /// 1.850 mH
    #[strum(serialize = "1.850 mH")]
    L1850 = 0xBB,
    /// 1.900 mH
    #[strum(serialize = "1.900 mH")]
    L1900 = 0xBC,
    /// 1.950 mH
    #[strum(serialize = "1.950 mH")]
    L1950 = 0xBD,
    /// 2.000 mH
    #[strum(serialize = "2.000 mH")]
    L2000 = 0xBE,
    /// 2.050 mH
    #[strum(serialize = "2.050 mH")]
    L2050 = 0xBF,
    /// 2.100 mH
    #[strum(serialize = "2.100 mH")]
    L2100 = 0xC0,
    /// 2.200 mH
    #[strum(serialize = "2.200 mH")]
    L2200 = 0xC1,
    /// 2.300 mH
    #[strum(serialize = "2.300 mH")]
    L2300 = 0xC2,
    /// 2.400 mH
    #[strum(serialize = "2.400 mH")]
    L2400 = 0xC3,
    /// 2.500 mH
    #[strum(serialize = "2.500 mH")]
    L2500 = 0xC4,
    /// 2.600 mH
    #[strum(serialize = "2.600 mH")]
    L2600 = 0xC5,
    /// 2.700 mH
    #[strum(serialize = "2.700 mH")]
    L2700 = 0xC6,
    /// 2.800 mH
    #[strum(serialize = "2.800 mH")]
    L2800 = 0xC7,
    /// 2.900 mH
    #[strum(serialize = "2.900 mH")]
    L2900 = 0xC8,
    /// 3.000 mH
    #[strum(serialize = "3.000 mH")]
    L3000 = 0xC9,
    /// 3.200 mH
    #[strum(serialize = "3.200 mH")]
    L3200 = 0xCA,
    /// 3.400 mH
    #[strum(serialize = "3.400 mH")]
    L3400 = 0xCB,
    /// 3.600 mH
    #[strum(serialize = "3.600 mH")]
    L3600 = 0xCC,
    /// 3.800 mH
    #[strum(serialize = "3.800 mH")]
    L3800 = 0xCD,
    /// 4.000 mH
    #[strum(serialize = "4.000 mH")]
    L4000 = 0xCE,
    /// 4.200 mH
    #[strum(serialize = "4.200 mH")]
    L4200 = 0xCF,
    /// 4.400 mH
    #[strum(serialize = "4.400 mH")]
    L4400 = 0xD0,
    /// 4.600 mH
    #[strum(serialize = "4.600 mH")]
    L4600 = 0xD1,
    /// 4.800 mH
    #[strum(serialize = "4.800 mH")]
    L4800 = 0xD2,
    /// 5.000 mH
    #[strum(serialize = "5.000 mH")]
    L5000 = 0xD3,
    /// 5.200 mH
    #[strum(serialize = "5.200 mH")]
    L5200 = 0xD4,
    /// 5.400 mH
    #[strum(serialize = "5.400 mH")]
    L5400 = 0xD5,
    /// 5.600 mH
    #[strum(serialize = "5.600 mH")]
    L5600 = 0xD6,
    /// 5.800 mH
    #[strum(serialize = "5.800 mH")]
    L5800 = 0xD7,
    /// 6.000 mH
    #[strum(serialize = "6.000 mH")]
    L6000 = 0xD8,
    /// 6.200 mH
    #[strum(serialize = "6.200 mH")]
    L6200 = 0xD9,
    /// 6.400 mH
    #[strum(serialize = "6.400 mH")]
    L6400 = 0xDA,
    /// 6.600 mH
    #[strum(serialize = "6.600 mH")]
    L6600 = 0xDB,
    /// 6.800 mH
    #[strum(serialize = "6.800 mH")]
    L6800 = 0xDC,
    /// 7.000 mH
    #[strum(serialize = "7.000 mH")]
    L7000 = 0xDD,
    /// 7.200 mH
    #[strum(serialize = "7.200 mH")]
    L7200 = 0xDE,
    /// 7.400 mH
    #[strum(serialize = "7.400 mH")]
    L7400 = 0xDF,
    /// 7.600 mH
    #[strum(serialize = "7.600 mH")]
    L7600 = 0xE0,
    /// 7.800 mH
    #[strum(serialize = "7.800 mH")]
    L7800 = 0xE1,
    /// 8.000 mH
    #[strum(serialize = "8.000 mH")]
    L8000 = 0xE2,
    /// 8.200 mH
    #[strum(serialize = "8.200 mH")]
    L8200 = 0xE3,
    /// 8.400 mH
    #[strum(serialize = "8.400 mH")]
    L8400 = 0xE4,
    /// 8.600 mH
    #[strum(serialize = "8.600 mH")]
    L8600 = 0xE5,
    /// 8.800 mH
    #[strum(serialize = "8.800 mH")]
    L8800 = 0xE6,
    /// 9.000 mH
    #[strum(serialize = "9.000 mH")]
    L9000 = 0xE7,
    /// 9.200 mH
    #[strum(serialize = "9.200 mH")]
    L9200 = 0xE8,
    /// 9.400 mH
    #[strum(serialize = "9.400 mH")]
    L9400 = 0xE9,
    /// 9.600 mH
    #[strum(serialize = "9.600 mH")]
    L9600 = 0xEA,
    /// 9.800 mH
    #[strum(serialize = "9.800 mH")]
    L9800 = 0xEB,
    /// 10.000 mH
    #[strum(serialize = "10.000 mH")]
    L10000 = 0xEC,
    /// 10.500 mH
    #[strum(serialize = "10.500 mH")]
    L10500 = 0xED,
    /// 11.000 mH
    #[strum(serialize = "11.000 mH")]
    L11000 = 0xEE,
    /// 11.500 mH
    #[strum(serialize = "11.500 mH")]
    L11500 = 0xEF,
    /// 12.000 mH
    #[strum(serialize = "12.000 mH")]
    L12000 = 0xF0,
    /// 12.500 mH
    #[strum(serialize = "12.500 mH")]
    L12500 = 0xF1,
    /// 13.000 mH
    #[strum(serialize = "13.000 mH")]
    L13000 = 0xF2,
    /// 13.500 mH
    #[strum(serialize = "13.500 mH")]
    L13500 = 0xF3,
    /// 14.000 mH
    #[strum(serialize = "14.000 mH")]
    L14000 = 0xF4,
    /// 14.500 mH
    #[strum(serialize = "14.500 mH")]
    L14500 = 0xF5,
    /// 15.000 mH
    #[strum(serialize = "15.000 mH")]
    L15000 = 0xF6,
    /// 15.500 mH
    #[strum(serialize = "15.500 mH")]
    L15500 = 0xF7,
    /// 16.000 mH
    #[strum(serialize = "16.000 mH")]
    L16000 = 0xF8,
    /// 16.500 mH
    #[strum(serialize = "16.500 mH")]
    L16500 = 0xF9,
    /// 17.000 mH
    #[strum(serialize = "17.000 mH")]
    L17000 = 0xFA,
    /// 17.500 mH
    #[strum(serialize = "17.500 mH")]
    L17500 = 0xFB,
    /// 18.000 mH
    #[strum(serialize = "18.000 mH")]
    L18000 = 0xFC,
    /// 18.500 mH
    #[strum(serialize = "18.500 mH")]
    L18500 = 0xFD,
    /// 19.000 mH
    #[strum(serialize = "19.000 mH")]
    L19000 = 0xFE,
    /// 20.000 mH
    #[strum(serialize = "20.000 mH")]
    L20000 = 0xFF,
}

impl PartialOrd for MotorInductance {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        if *self == MotorInductance::SelfMeasurement || *other == MotorInductance::SelfMeasurement {
            None
        } else {
            (*self as u8).partial_cmp(&(*other as u8))
        }
    }
}

impl From<u8> for MotorInductance {
    fn from(value: u8) -> Self {
        // Safety: MotorInductance has a valid representation for all u8 values
        unsafe { core::mem::transmute(value) }
    }
}
