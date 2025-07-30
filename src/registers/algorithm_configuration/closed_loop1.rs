//! Section 7.7.1.5

use super::*;
use arbitrary_int::*;
use bitbybit::*;

/// Register to configure close loop settings1
#[bitfield(u32, debug, default = 0x0)]
#[derive(PartialEq, Eq)]
pub struct ClosedLoop1 {
    /// Enable overmodulation.
    /// 0 = Disable overmodulation, 1 = Enable overmodulation
    #[bit(30, rw)]
    pub overmodulation_enable: bool,
    /// Closed loop acceleration.
    #[bits(25..=29, rw)]
    pub cl_acc: ClosedLoopAcceleration,
    /// Closed loop deceleration configuration.
    /// 0 = Closed loop deceleration defined by cl_dec,
    /// 1 = Closed loop deceleration defined by cl_acc
    #[bit(24, rw)]
    pub cl_dec_config: bool,
    /// Closed loop deceleration.
    /// This setting is used only if AVS is disabled and cl_dec_config is set to 0.
    #[bits(19..=23, rw)]
    pub cl_dec: ClosedLoopAcceleration,
    /// PWM output frequency.
    #[bits(15..=18, rw)]
    pub pwm_freq_out: Option<PwmOutputFrequency>,
    /// PWM modulation.
    /// 0 = Continuous Space Vector Modulation,
    /// 1 = Discontinuous Space Vector Modulation
    #[bit(14, rw)]
    pub pwm_mode: bool,
    /// FG select
    #[bits(12..=13, rw)]
    pub fg_sel: Option<FgSelect>,
    /// FG Division factor
    #[bits(8..=11, rw)]
    pub fg_div: FgDiv,
    /// FG output configuration.
    /// 0 = FG active as long as motor is driven,
    /// 1 = FG active till BEMF drops below BEMF threshold defined by FG_BEMF_THR
    #[bit(7, rw)]
    pub fg_config: bool,
    /// FG output BEMF threshold
    #[bits(4..=6, rw)]
    pub fg_bemf_thr: Option<FgBemfThreshold>,
    /// AVS enable.
    /// 0 = Disable, 1 = Enable
    #[bit(3, rw)]
    pub avs_en: bool,
    /// Deadtime compensation enable.
    /// 0 = Disable, 1 = Enable
    #[bit(2, rw)]
    pub deadtime_comp_en: bool,
    /// Speed loop disable (or torque mode enable).
    /// 0 = Speed loop enable (Torque mode disable),
    /// 1 = Speed loop disable (Torque mode enable)
    #[bit(1, rw)]
    pub speed_loop_dis: bool,
}

impl Register for ClosedLoop1 {
    fn address(&self) -> u12 {
        CLOSED_LOOP1
    }

    fn value(&self) -> u32 {
        self.raw_value()
    }

    fn from_value(value: u32) -> Self {
        Self::new_with_raw_value(value)
    }
}

/// Closed loop acceleration.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, strum::Display)]
#[bitenum(u5, exhaustive = true)]
pub enum ClosedLoopAcceleration {
    /// 0.5 Hz/s
    #[strum(serialize = "0.5 Hz/s")]
    Hz0_5 = 0x00,
    /// 1 Hz/s
    #[strum(serialize = "1 Hz/s")]
    Hz1 = 0x01,
    /// 2.5 Hz/s
    #[strum(serialize = "2.5 Hz/s")]
    Hz2_5 = 0x02,
    /// 5 Hz/s
    #[strum(serialize = "5 Hz/s")]
    Hz5 = 0x03,
    /// 7.5 Hz/s
    #[strum(serialize = "7.5 Hz/s")]
    Hz7_5 = 0x04,
    /// 10 Hz/s
    #[strum(serialize = "10 Hz/s")]
    Hz10 = 0x05,
    /// 20 Hz/s
    #[strum(serialize = "20 Hz/s")]
    Hz20 = 0x06,
    /// 40 Hz/s
    #[strum(serialize = "40 Hz/s")]
    Hz40 = 0x07,
    /// 60 Hz/s
    #[strum(serialize = "60 Hz/s")]
    Hz60 = 0x08,
    /// 80 Hz/s
    #[strum(serialize = "80 Hz/s")]
    Hz80 = 0x09,
    /// 100 Hz/s
    #[strum(serialize = "100 Hz/s")]
    Hz100 = 0x0A,
    /// 200 Hz/s
    #[strum(serialize = "200 Hz/s")]
    Hz200 = 0x0B,
    /// 300 Hz/s
    #[strum(serialize = "300 Hz/s")]
    Hz300 = 0x0C,
    /// 400 Hz/s
    #[strum(serialize = "400 Hz/s")]
    Hz400 = 0x0D,
    /// 500 Hz/s
    #[strum(serialize = "500 Hz/s")]
    Hz500 = 0x0E,
    /// 600 Hz/s
    #[strum(serialize = "600 Hz/s")]
    Hz600 = 0x0F,
    /// 700 Hz/s
    #[strum(serialize = "700 Hz/s")]
    Hz700 = 0x10,
    /// 800 Hz/s
    #[strum(serialize = "800 Hz/s")]
    Hz800 = 0x11,
    /// 900 Hz/s
    #[strum(serialize = "900 Hz/s")]
    Hz900 = 0x12,
    /// 1000 Hz/s
    #[strum(serialize = "1000 Hz/s")]
    Hz1000 = 0x13,
    /// 2000 Hz/s
    #[strum(serialize = "2000 Hz/s")]
    Hz2000 = 0x14,
    /// 4000 Hz/s
    #[strum(serialize = "4000 Hz/s")]
    Hz4000 = 0x15,
    /// 6000 Hz/s
    #[strum(serialize = "6000 Hz/s")]
    Hz6000 = 0x16,
    /// 8000 Hz/s
    #[strum(serialize = "8000 Hz/s")]
    Hz8000 = 0x17,
    /// 10000 Hz/s
    #[strum(serialize = "10000 Hz/s")]
    Hz10000 = 0x18,
    /// 20000 Hz/s
    #[strum(serialize = "20000 Hz/s")]
    Hz20000 = 0x19,
    /// 30000 Hz/s
    #[strum(serialize = "30000 Hz/s")]
    Hz30000 = 0x1A,
    /// 40000 Hz/s
    #[strum(serialize = "40000 Hz/s")]
    Hz40000 = 0x1B,
    /// 50000 Hz/s
    #[strum(serialize = "50000 Hz/s")]
    Hz50000 = 0x1C,
    /// 60000 Hz/s
    #[strum(serialize = "60000 Hz/s")]
    Hz60000 = 0x1D,
    /// 70000 Hz/s
    #[strum(serialize = "70000 Hz/s")]
    Hz70000 = 0x1E,
    /// No limit
    #[strum(serialize = "No Limit")]
    NoLimit = 0x1F,
}

/// PWM output frequency.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, strum::Display)]
#[bitenum(u4, exhaustive = false)]
pub enum PwmOutputFrequency {
    /// 10 kHz
    #[strum(serialize = "10 kHz")]
    Khz10 = 0x00,
    /// 15 kHz
    #[strum(serialize = "15 kHz")]
    Khz15 = 0x01,
    /// 20 kHz
    #[strum(serialize = "20 kHz")]
    Khz20 = 0x02,
    /// 25 kHz
    #[strum(serialize = "25 kHz")]
    Khz25 = 0x03,
    /// 30 kHz
    #[strum(serialize = "30 kHz")]
    Khz30 = 0x04,
    /// 35 kHz
    #[strum(serialize = "35 kHz")]
    Khz35 = 0x05,
    /// 40 kHz
    #[strum(serialize = "40 kHz")]
    Khz40 = 0x06,
    /// 45 kHz
    #[strum(serialize = "45 kHz")]
    Khz45 = 0x07,
    /// 50 kHz
    #[strum(serialize = "50 kHz")]
    Khz50 = 0x08,
    /// 55 kHz
    #[strum(serialize = "55 kHz")]
    Khz55 = 0x09,
    /// 60 kHz
    #[strum(serialize = "60 kHz")]
    Khz60 = 0x0A,
    // Not Applicable
}

/// FG select
#[derive(Debug, PartialEq, Eq, strum::Display)]
#[bitenum(u2, exhaustive = false)]
pub enum FgSelect {
    /// Output FG in open loop and closed loop
    #[strum(serialize = "Open and Closed Loop")]
    OpenClosed = 0x0,
    /// Output FG in only closed loop
    #[strum(serialize = "Closed Loop Only")]
    ClosedLoop = 0x1,
    /// Output FG in open loop for the first motor run after power-up/wake-up
    #[strum(serialize = "Open Loop First Run")]
    OpenLoopFirstRun = 0x2,
}

/// FG Division factor
#[derive(Debug, PartialEq, Eq, strum::Display)]
#[bitenum(u4, exhaustive = true)]
pub enum FgDiv {
    /// Divide by 1 (2-pole motor mechanical speed)
    #[strum(serialize = "Divide by 1")]
    Div1 = 0x0,
    // There is no difference between these two
    /// Divide by 1 (2-pole motor mechanical speed)
    #[strum(serialize = "Divide by 1")]
    TheCoolerDiv1 = 0x1,
    /// Divide by 2 (4-pole motor mechanical speed)
    #[strum(serialize = "Divide by 2")]
    Div2 = 0x2,
    /// Divide by 3 (6-pole motor mechanical speed)
    #[strum(serialize = "Divide by 3")]
    Div3 = 0x3,
    /// Divide by 4 (8-pole motor mechanical speed)
    #[strum(serialize = "Divide by 4")]
    Div4 = 0x4,
    /// Divide by 5 (10-pole motor mechanical speed)
    #[strum(serialize = "Divide by 5")]
    Div5 = 0x5,
    /// Divide by 6 (12-pole motor mechanical speed)
    #[strum(serialize = "Divide by 6")]
    Div6 = 0x6,
    /// Divide by 7 (14-pole motor mechanical speed)
    #[strum(serialize = "Divide by 7")]
    Div7 = 0x7,
    /// Divide by 8 (16-pole motor mechanical speed)
    #[strum(serialize = "Divide by 8")]
    Div8 = 0x8,
    /// Divide by 9 (18-pole motor mechanical speed)
    #[strum(serialize = "Divide by 9")]
    Div9 = 0x9,
    /// Divide by 10 (20-pole motor mechanical speed)
    #[strum(serialize = "Divide by 10")]
    Div10 = 0xA,
    /// Divide by 11 (22-pole motor mechanical speed)
    #[strum(serialize = "Divide by 11")]
    Div11 = 0xB,
    /// Divide by 12 (24-pole motor mechanical speed)
    #[strum(serialize = "Divide by 12")]
    Div12 = 0xC,
    /// Divide by 13 (26-pole motor mechanical speed)
    #[strum(serialize = "Divide by 13")]
    Div13 = 0xD,
    /// Divide by 14 (28-pole motor mechanical speed)
    #[strum(serialize = "Divide by 14")]
    Div14 = 0xE,
    /// Divide by 15 (30-pole motor mechanical speed)
    #[strum(serialize = "Divide by 15")]
    Div15 = 0xF,
}

/// FG output BEMF threshold
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, strum::Display)]
#[bitenum(u3, exhaustive = false)]
pub enum FgBemfThreshold {
    /// 1 mV
    #[strum(serialize = "1 mV")]
    Mv1 = 0x0,
    /// 2 mV
    #[strum(serialize = "2 mV")]
    Mv2 = 0x1,
    /// 5 mV
    #[strum(serialize = "5 mV")]
    Mv5 = 0x2,
    /// 10 mV
    #[strum(serialize = "10 mV")]
    Mv10 = 0x3,
    /// 20 mV
    #[strum(serialize = "20 mV")]
    Mv20 = 0x4,
    /// 30 mV
    #[strum(serialize = "30 mV")]
    Mv30 = 0x5,
    // Not Applicable
}
