pub use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ClosedLoop1 {
    /// Enable overmodulation.
    /// 0 = Disable overmodulation, 1 = Enable overmodulation
    pub overmodulation_enable: bool,
    /// Closed loop acceleration.
    pub cl_acc: ClosedLoopAcceleration,
    /// Closed loop deceleration configuration.
    /// 0 = Closed loop deceleration defined by cl_dec,
    /// 1 = Closed loop deceleration defined by cl_acc
    pub cl_dec_config: bool,
    /// Closed loop deceleration.
    /// This setting is used only if AVS is disabled and cl_dec_config is set to 0.
    pub cl_dec: ClosedLoopAcceleration,
    /// PWM output frequency.
    pub pwm_freq_out: PwmOutputFrequency,
    /// PWM modulation.
    /// 0 = Continuous Space Vector Modulation,
    /// 1 = Discontinuous Space Vector Modulation
    pub pwm_mode: bool,
    /// FG select
    pub fg_sel: FgSelect,
    /// FG Division factor
    pub fg_div: FgDiv,
    /// FG output configuration.
    /// 0 = FG active as long as motor is driven,
    /// 1 = FG active till BEMF drops below BEMF threshold defined by FG_BEMF_THR
    pub fg_config: bool,
    /// FG output BEMF threshold
    pub fg_bemf_thr: FgBemfThreshold,
    /// AVS enable.
    /// 0 = Disable, 1 = Enable
    pub avs_en: bool,
    /// Deadtime compensation enable.
    /// 0 = Disable, 1 = Enable
    pub deadtime_comp_en: bool,
    /// Speed loop disable (or torque mode enable).
    /// 0 = Speed loop enable (Torque mode disable),
    /// 1 = Speed loop disable (Torque mode enable)
    pub speed_loop_dis: bool,
}

impl Register for ClosedLoop1 {
    const ADDRESS: u16 = CLOSED_LOOP1; // Example address, replace with actual address
}

impl From<ClosedLoop1> for u32 {
    fn from(config: ClosedLoop1) -> Self {
        let mut value = 0;
        value |= (config.overmodulation_enable as u32) << 30;
        value |= (config.cl_acc as u32) << 25;
        value |= (config.cl_dec_config as u32) << 24;
        value |= (config.cl_dec as u32) << 19;
        value |= (config.pwm_freq_out as u32) << 15;
        value |= (config.pwm_mode as u32) << 14;
        value |= (config.fg_sel as u32) << 12;
        value |= (config.fg_div as u32) << 8;
        value |= (config.fg_config as u32) << 7;
        value |= (config.fg_bemf_thr as u32) << 4;
        value |= (config.avs_en as u32) << 3;
        value |= (config.deadtime_comp_en as u32) << 2;
        value |= (config.speed_loop_dis as u32) << 1;
        value
    }
}

impl From<u32> for ClosedLoop1 {
    fn from(value: u32) -> Self {
        // Safety: The values are masked to ensure they fit within the expected ranges.
        unsafe {
            ClosedLoop1 {
                overmodulation_enable: (value >> 30) & 0x01 != 0,
                cl_acc: ClosedLoopAcceleration::try_from((value >> 25) as u8 & 0x1F)
                    .unwrap_unchecked(),
                cl_dec_config: (value >> 24) & 0x01 != 0,
                cl_dec: ClosedLoopAcceleration::try_from((value >> 19) as u8 & 0x1F)
                    .unwrap_unchecked(),
                pwm_freq_out: PwmOutputFrequency::try_from((value >> 15) as u8 & 0x0F)
                    .unwrap_unchecked(),
                pwm_mode: (value >> 14) & 0x01 != 0,
                fg_sel: FgSelect::from((value >> 12) as u8 & 0x03),
                fg_div: FgDiv::try_from((value >> 8) as u8 & 0x0F).unwrap_unchecked(),
                fg_config: (value >> 7) & 0x01 != 0,
                fg_bemf_thr: FgBemfThreshold::from((value >> 4) as u8 & 0x07),
                avs_en: (value >> 3) & 0x01 != 0,
                deadtime_comp_en: (value >> 2) & 0x01 != 0,
                speed_loop_dis: (value >> 1) & 0x01 != 0,
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, strum::Display)]
#[repr(u8)]
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

impl TryFrom<u8> for ClosedLoopAcceleration {
    type Error = BitCountError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x00 => Ok(ClosedLoopAcceleration::Hz0_5),
            0x01 => Ok(ClosedLoopAcceleration::Hz1),
            0x02 => Ok(ClosedLoopAcceleration::Hz2_5),
            0x03 => Ok(ClosedLoopAcceleration::Hz5),
            0x04 => Ok(ClosedLoopAcceleration::Hz7_5),
            0x05 => Ok(ClosedLoopAcceleration::Hz10),
            0x06 => Ok(ClosedLoopAcceleration::Hz20),
            0x07 => Ok(ClosedLoopAcceleration::Hz40),
            0x08 => Ok(ClosedLoopAcceleration::Hz60),
            0x09 => Ok(ClosedLoopAcceleration::Hz80),
            0x0A => Ok(ClosedLoopAcceleration::Hz100),
            0x0B => Ok(ClosedLoopAcceleration::Hz200),
            0x0C => Ok(ClosedLoopAcceleration::Hz300),
            0x0D => Ok(ClosedLoopAcceleration::Hz400),
            0x0E => Ok(ClosedLoopAcceleration::Hz500),
            0x0F => Ok(ClosedLoopAcceleration::Hz600),
            0x10 => Ok(ClosedLoopAcceleration::Hz700),
            0x11 => Ok(ClosedLoopAcceleration::Hz800),
            0x12 => Ok(ClosedLoopAcceleration::Hz900),
            0x13 => Ok(ClosedLoopAcceleration::Hz1000),
            0x14 => Ok(ClosedLoopAcceleration::Hz2000),
            0x15 => Ok(ClosedLoopAcceleration::Hz4000),
            0x16 => Ok(ClosedLoopAcceleration::Hz6000),
            0x17 => Ok(ClosedLoopAcceleration::Hz8000),
            0x18 => Ok(ClosedLoopAcceleration::Hz10000),
            0x19 => Ok(ClosedLoopAcceleration::Hz20000),
            0x1A => Ok(ClosedLoopAcceleration::Hz30000),
            0x1B => Ok(ClosedLoopAcceleration::Hz40000),
            0x1C => Ok(ClosedLoopAcceleration::Hz50000),
            0x1D => Ok(ClosedLoopAcceleration::Hz60000),
            0x1E => Ok(ClosedLoopAcceleration::Hz70000),
            0x1F => Ok(ClosedLoopAcceleration::NoLimit),
            _ => Err(BitCountError::new(5, value.ilog2() as u8)),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, strum::Display)]
#[repr(u8)]
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

impl TryFrom<u8> for PwmOutputFrequency {
    type Error = BitCountError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x00 => Ok(PwmOutputFrequency::Khz10),
            0x01 => Ok(PwmOutputFrequency::Khz15),
            0x02 => Ok(PwmOutputFrequency::Khz20),
            0x03 => Ok(PwmOutputFrequency::Khz25),
            0x04 => Ok(PwmOutputFrequency::Khz30),
            0x05 => Ok(PwmOutputFrequency::Khz35),
            0x06 => Ok(PwmOutputFrequency::Khz40),
            0x07 => Ok(PwmOutputFrequency::Khz45),
            0x08 => Ok(PwmOutputFrequency::Khz50),
            0x09 => Ok(PwmOutputFrequency::Khz55),
            0x0A => Ok(PwmOutputFrequency::Khz60),
            _ => Err(BitCountError::new(4, value.ilog2() as u8)),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, strum::Display)]
#[repr(u8)]
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

impl From<u8> for FgSelect {
    fn from(value: u8) -> Self {
        match value {
            0x0 => FgSelect::OpenClosed,
            0x1 => FgSelect::ClosedLoop,
            0x2 => FgSelect::OpenLoopFirstRun,
            _ => panic!("Invalid FgSelect value: {}", value),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, strum::Display)]
#[repr(u8)]
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

impl TryFrom<u8> for FgDiv {
    type Error = BitCountError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x0 => Ok(FgDiv::Div1),
            0x1 => Ok(FgDiv::TheCoolerDiv1),
            0x2 => Ok(FgDiv::Div2),
            0x3 => Ok(FgDiv::Div3),
            0x4 => Ok(FgDiv::Div4),
            0x5 => Ok(FgDiv::Div5),
            0x6 => Ok(FgDiv::Div6),
            0x7 => Ok(FgDiv::Div7),
            0x8 => Ok(FgDiv::Div8),
            0x9 => Ok(FgDiv::Div9),
            0xA => Ok(FgDiv::Div10),
            0xB => Ok(FgDiv::Div11),
            0xC => Ok(FgDiv::Div12),
            0xD => Ok(FgDiv::Div13),
            0xE => Ok(FgDiv::Div14),
            0xF => Ok(FgDiv::Div15),
            _ => Err(BitCountError::new(4, value.ilog2() as u8)),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, strum::Display)]
#[repr(u8)]
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

impl From<u8> for FgBemfThreshold {
    fn from(value: u8) -> Self {
        match value {
            0x0 => FgBemfThreshold::Mv1,
            0x1 => FgBemfThreshold::Mv2,
            0x2 => FgBemfThreshold::Mv5,
            0x3 => FgBemfThreshold::Mv10,
            0x4 => FgBemfThreshold::Mv20,
            0x5 => FgBemfThreshold::Mv30,
            _ => panic!("Invalid FgBemfThreshold value: {}", value),
        }
    }
}
