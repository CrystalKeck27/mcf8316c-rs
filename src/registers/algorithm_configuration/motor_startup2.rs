//! Section 7.7.1.4

use super::*;
use arbitrary_int::*;
use bitbybit::*;

/// Register to configure motor startup settings2
#[bitfield(u32, debug, default = 0x0)]
#[derive(PartialEq, Eq)]
pub struct MotorStartup2 {
    /// Open loop current limit
    #[bits(27..=30, rw)]
    pub ol_ilimit: CurrentSelection,
    /// Open loop acceleration coefficient A1
    #[bits(23..=26, rw)]
    pub ol_acc_a1: OpenLoopAccelerationA1,
    /// Open loop acceleration coefficient A2
    #[bits(19..=22, rw)]
    pub ol_acc_a2: OpenLoopAccelerationA2,
    /// Auto Handoff Enable.
    /// 0 = Disable Auto Handoff (and use opn_cl_handoff_thr),
    /// 1 = Enable Auto Handoff
    #[bit(18, rw)]
    pub auto_handoff_en: bool,
    /// Open to closed loop handoff threshold (% of MAX_SPEED)
    #[bits(13..=17, rw)]
    pub opn_cl_handoff_thr: OpenCloseLoopHandoffThreshold,
    /// Align angle
    #[bits(8..=12, rw)]
    pub align_angle: Option<AlignAngle>,
    /// Frequency of first cycle during start-up (% of MAX_SPEED)
    #[bits(4..=7, rw)]
    pub slow_first_cyc_freq: SlowFirstCycleFrequency,
    /// First cycle frequency in open loop for align, double align, and IPD start-up
    /// 0 = 0 Hz, 1 = Defined by slow_first_cyc_freq
    #[bit(3, rw)]
    pub first_cycle_freq_sel: bool,
    /// Ramp rate for reducing difference between estimated theta and open loop theta
    #[bits(0..=2, rw)]
    pub theta_error_ramp_rate: ThetaErrorRampRate,
}

impl Register for MotorStartup2 {
    fn address(&self) -> u12 {
        MOTOR_STARTUP2
    }

    fn value(&self) -> u32 {
        self.raw_value()
    }

    fn from_value(value: u32) -> Self {
        Self::new_with_raw_value(value)
    }
}

/// Open to closed loop handoff threshold (% of MAX_SPEED)
#[bitenum(u5, exhaustive = true)]
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, strum::Display)]
pub enum OpenCloseLoopHandoffThreshold {
    /// 1% of MAX_SPEED
    #[strum(to_string = "1%")]
    P1 = 0x0,
    /// 2% of MAX_SPEED
    #[strum(to_string = "2%")]
    P2 = 0x1,
    /// 3% of MAX_SPEED
    #[strum(to_string = "3%")]
    P3 = 0x2,
    /// 4% of MAX_SPEED
    #[strum(to_string = "4%")]
    P4 = 0x3,
    /// 5% of MAX_SPEED
    #[strum(to_string = "5%")]
    P5 = 0x4,
    /// 6% of MAX_SPEED
    #[strum(to_string = "6%")]
    P6 = 0x5,
    /// 7% of MAX_SPEED
    #[strum(to_string = "7%")]
    P7 = 0x6,
    /// 8% of MAX_SPEED
    #[strum(to_string = "8%")]
    P8 = 0x7,
    /// 9% of MAX_SPEED
    #[strum(to_string = "9%")]
    P9 = 0x8,
    /// 10% of MAX_SPEED
    #[strum(to_string = "10%")]
    P10 = 0x9,
    /// 11% of MAX_SPEED
    #[strum(to_string = "11%")]
    P11 = 0xA,
    /// 12% of MAX_SPEED
    #[strum(to_string = "12%")]
    P12 = 0xB,
    /// 13% of MAX_SPEED
    #[strum(to_string = "13%")]
    P13 = 0xC,
    /// 14% of MAX_SPEED
    #[strum(to_string = "14%")]
    P14 = 0xD,
    /// 15% of MAX_SPEED
    #[strum(to_string = "15%")]
    P15 = 0xE,
    /// 16% of MAX_SPEED
    #[strum(to_string = "16%")]
    P16 = 0xF,
    /// 17% of MAX_SPEED
    #[strum(to_string = "17%")]
    P17 = 0x10,
    /// 18% of MAX_SPEED
    #[strum(to_string = "18%")]
    P18 = 0x11,
    /// 19% of MAX_SPEED
    #[strum(to_string = "19%")]
    P19 = 0x12,
    /// 20% of MAX_SPEED
    #[strum(to_string = "20%")]
    P20 = 0x13,
    /// 22.5% of MAX_SPEED
    #[strum(to_string = "22.5%")]
    P22_5 = 0x14,
    /// 25% of MAX_SPEED
    #[strum(to_string = "25%")]
    P25 = 0x15,
    /// 27.5% of MAX_SPEED
    #[strum(to_string = "27.5%")]
    P27_5 = 0x16,
    /// 30% of MAX_SPEED
    #[strum(to_string = "30%")]
    P30 = 0x17,
    /// 32.5% of MAX_SPEED
    #[strum(to_string = "32.5%")]
    P32_5 = 0x18,
    /// 35% of MAX_SPEED
    #[strum(to_string = "35%")]
    P35 = 0x19,
    /// 37.5% of MAX_SPEED
    #[strum(to_string = "37.5%")]
    P37_5 = 0x1A,
    /// 40% of MAX_SPEED
    #[strum(to_string = "40%")]
    P40 = 0x1B,
    /// 42.5% of MAX_SPEED
    #[strum(to_string = "42.5%")]
    P42_5 = 0x1C,
    /// 45% of MAX_SPEED
    #[strum(to_string = "45%")]
    P45 = 0x1D,
    /// 47.5% of MAX_SPEED
    #[strum(to_string = "47.5%")]
    P47_5 = 0x1E,
    /// 50% of MAX_SPEED
    #[strum(to_string = "50%")]
    P50 = 0x1F,
}

// I have no idea why they did it this way
/// Align angle
#[bitenum(u5, exhaustive = false)]
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, strum::Display)]
pub enum AlignAngle {
    /// 0 degrees
    #[strum(to_string = "0°")]
    Deg0 = 0x0,
    /// 10 degrees
    #[strum(to_string = "10°")]
    Deg10 = 0x1,
    /// 20 degrees
    #[strum(to_string = "20°")]
    Deg20 = 0x2,
    /// 30 degrees
    #[strum(to_string = "30°")]
    Deg30 = 0x3,
    /// 45 degrees
    #[strum(to_string = "45°")]
    Deg45 = 0x4,
    /// 60 degrees
    #[strum(to_string = "60°")]
    Deg60 = 0x5,
    /// 70 degrees
    #[strum(to_string = "70°")]
    Deg70 = 0x6,
    /// 80 degrees
    #[strum(to_string = "80°")]
    Deg80 = 0x7,
    /// 90 degrees
    #[strum(to_string = "90°")]
    Deg90 = 0x8,
    /// 110 degrees
    #[strum(to_string = "110°")]
    Deg110 = 0x9,
    /// 120 degrees
    #[strum(to_string = "120°")]
    Deg120 = 0xA,
    /// 135 degrees
    #[strum(to_string = "135°")]
    Deg135 = 0xB,
    /// 150 degrees
    #[strum(to_string = "150°")]
    Deg150 = 0xC,
    /// 160 degrees
    #[strum(to_string = "160°")]
    Deg160 = 0xD,
    /// 170 degrees
    #[strum(to_string = "170°")]
    Deg170 = 0xE,
    /// 180 degrees
    #[strum(to_string = "180°")]
    Deg180 = 0xF,
    /// 190 degrees
    #[strum(to_string = "190°")]
    Deg190 = 0x10,
    /// 210 degrees
    #[strum(to_string = "210°")]
    Deg210 = 0x11,
    /// 225 degrees
    #[strum(to_string = "225°")]
    Deg225 = 0x12,
    /// 240 degrees
    #[strum(to_string = "240°")]
    Deg240 = 0x13,
    /// 250 degrees
    #[strum(to_string = "250°")]
    Deg250 = 0x14,
    /// 260 degrees
    #[strum(to_string = "260°")]
    Deg260 = 0x15,
    /// 270 degrees
    #[strum(to_string = "270°")]
    Deg270 = 0x16,
    /// 280 degrees
    #[strum(to_string = "280°")]
    Deg280 = 0x17,
    /// 290 degrees
    #[strum(to_string = "290°")]
    Deg290 = 0x18,
    /// 315 degrees
    #[strum(to_string = "315°")]
    Deg315 = 0x19,
    /// 330 degrees
    #[strum(to_string = "330°")]
    Deg330 = 0x1A,
    /// 340 degrees
    #[strum(to_string = "340°")]
    Deg340 = 0x1B,
    /// 350 degrees
    #[strum(to_string = "350°")]
    Deg350 = 0x1C,
    // Reserved
}

/// Frequency of first cycle during start-up (% of MAX_SPEED)
#[bitenum(u4, exhaustive = true)]
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, strum::Display)]
pub enum SlowFirstCycleFrequency {
    /// 1% of MAX_SPEED
    #[strum(to_string = "1%")]
    P1 = 0x0,
    /// 2% of MAX_SPEED
    #[strum(to_string = "2%")]
    P2 = 0x1,
    /// 3% of MAX_SPEED
    #[strum(to_string = "3%")]
    P3 = 0x2,
    /// 5% of MAX_SPEED
    #[strum(to_string = "5%")]
    P5 = 0x3,
    /// 7.5% of MAX_SPEED
    #[strum(to_string = "7.5%")]
    P7_5 = 0x4,
    /// 10% of MAX_SPEED
    #[strum(to_string = "10%")]
    P10 = 0x5,
    /// 12.5% of MAX_SPEED
    #[strum(to_string = "12.5%")]
    P12_5 = 0x6,
    /// 15% of MAX_SPEED
    #[strum(to_string = "15%")]
    P15 = 0x7,
    /// 17.5% of MAX_SPEED
    #[strum(to_string = "17.5%")]
    P17_5 = 0x8,
    /// 20% of MAX_SPEED
    #[strum(to_string = "20%")]
    P20 = 0x9,
    /// 25% of MAX_SPEED
    #[strum(to_string = "25%")]
    P25 = 0xA,
    /// 30% of MAX_SPEED
    #[strum(to_string = "30%")]
    P30 = 0xB,
    /// 35% of MAX_SPEED
    #[strum(to_string = "35%")]
    P35 = 0xC,
    /// 40% of MAX_SPEED
    #[strum(to_string = "40%")]
    P40 = 0xD,
    /// 45% of MAX_SPEED
    #[strum(to_string = "45%")]
    P45 = 0xE,
    /// 50% of MAX_SPEED
    #[strum(to_string = "50%")]
    P50 = 0xF,
}

/// Ramp rate for reducing difference between estimated theta and open
/// loop theta
#[bitenum(u3, exhaustive = true)]
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, strum::Display)]
pub enum ThetaErrorRampRate {
    /// 0.01 deg/ms
    #[strum(to_string = "0.01°/ms")]
    D0_01 = 0x0,
    /// 0.05 deg/ms
    #[strum(to_string = "0.05°/ms")]
    D0_05 = 0x1,
    /// 0.1 deg/ms
    #[strum(to_string = "0.1°/ms")]
    D0_1 = 0x2,
    /// 0.15 deg/ms
    #[strum(to_string = "0.15°/ms")]
    D0_15 = 0x3,
    /// 0.2 deg/ms
    #[strum(to_string = "0.2°/ms")]
    D0_2 = 0x4,
    /// 0.5 deg/ms
    #[strum(to_string = "0.5°/ms")]
    D0_5 = 0x5,
    /// 1 deg/ms
    #[strum(to_string = "1°/ms")]
    D1 = 0x6,
    /// 2 deg/ms
    #[strum(to_string = "2°/ms")]
    D2 = 0x7,
}
