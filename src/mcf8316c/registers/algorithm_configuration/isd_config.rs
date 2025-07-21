pub use super::*;

/// Data model for the Initial Speed Detection (ISD) configuration register.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IsdConfig {
    /// ISD Enable.
    /// 1 = Enable ISD, 0 = Disable ISD
    pub isd_en: bool,
    /// Brake Enable.
    /// 1 = Enable Brake, 0 = Disable Brake
    pub brake_en: bool,
    /// Hi-Z Enable.
    /// 1 = Enable High Impedance, 0 = Disable High Impedance
    pub hiz_en: bool,
    /// Reverse Drive Enable.
    /// 1 = Enable Reverse Drive, 0 = Disable Reverse Drive
    pub rvs_dr_en: bool,
    /// Resynchronization Enable.
    /// 1 = Enable Resynchronization, 0 = Disable Resynchronization
    pub resync_en: bool,
    /// Minimum Speed threshold to resynchronize to close loop (% of MAX_SPEED)
    pub fw_drv_resyn_thr: ResyncThreshold,
    /// Brake mode
    /// 0 = All high-side FETs on, 1 = All low-side FETs on
    pub brk_mode: bool,
    /// Brake config
    /// 0 = Brake time is used to exit Brake state,
    /// 1 = Brake current threshold and Brake time are used to exit Brake state
    pub brk_config: bool,
    /// Brake current threshold
    pub brk_curr_thr: BrakeCurrentThreshold,
    /// Brake time
    pub brk_time: TimeFormatA,
    /// Hi-Z time
    pub hiz_time: TimeFormatA,
    /// BEMF threshold to detect if motor is stationary
    pub stat_detect_thr: BemfStationaryVoltageThreshold,
    /// Speed threshold used to transition th open loop during reverse drive (% of MAX_SPEED)
    pub rev_drv_handoff_thr: ReverseDriveHandoffThreshold,
    /// Open loop current limit during reverse drive
    pub rev_drv_open_loop_current: ReverseDriveOpenLoopCurrent,
}

impl Register for IsdConfig {
    const ADDRESS: u16 = ISD_CONFIG; // Example address, replace with actual address
}

impl From<IsdConfig> for u32 {
    fn from(config: IsdConfig) -> Self {
        let mut value = 0;
        value |= (config.isd_en as u32) << 30;
        value |= (config.brake_en as u32) << 29;
        value |= (config.hiz_en as u32) << 28;
        value |= (config.rvs_dr_en as u32) << 27;
        value |= (config.resync_en as u32) << 26;
        value |= (config.fw_drv_resyn_thr as u32) << 22;
        value |= (config.brk_mode as u32) << 21;
        value |= (config.brk_config as u32) << 20;
        value |= (config.brk_curr_thr as u32) << 17;
        value |= (config.brk_time as u32) << 13;
        value |= (config.hiz_time as u32) << 9;
        value |= (config.stat_detect_thr as u32) << 6;
        value |= (config.rev_drv_handoff_thr as u32) << 2;
        value |= config.rev_drv_open_loop_current as u32;
        value
    }
}

impl From<u32> for IsdConfig {
    fn from(value: u32) -> Self {
        IsdConfig {
            isd_en: (value >> 30) & 0x1 != 0,
            brake_en: (value >> 29) & 0x1 != 0,
            hiz_en: (value >> 28) & 0x1 != 0,
            rvs_dr_en: (value >> 27) & 0x1 != 0,
            resync_en: (value >> 26) & 0x1 != 0,
            fw_drv_resyn_thr: ResyncThreshold::from(((value >> 22) & 0xF) as u8),
            brk_mode: (value >> 21) & 0x1 != 0,
            brk_config: (value >> 20) & 0x1 != 0,
            brk_curr_thr: BrakeCurrentThreshold::from(((value >> 17) & 0x7) as u8),
            brk_time: TimeFormatA::from(((value >> 13) & 0xF) as u8),
            hiz_time: TimeFormatA::from(((value >> 9) & 0xF) as u8),
            stat_detect_thr: BemfStationaryVoltageThreshold::from(((value >> 6) & 0x7) as u8),
            rev_drv_handoff_thr: ReverseDriveHandoffThreshold::from(((value >> 2) & 0xF) as u8),
            rev_drv_open_loop_current: ReverseDriveOpenLoopCurrent::from((value & 0x3) as u8),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, strum::Display)]
#[repr(u8)]
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

impl From<u8> for ResyncThreshold {
    fn from(value: u8) -> Self {
        match value {
            0x0 => ResyncThreshold::P5,
            0x1 => ResyncThreshold::P10,
            0x2 => ResyncThreshold::P15,
            0x3 => ResyncThreshold::P20,
            0x4 => ResyncThreshold::P25,
            0x5 => ResyncThreshold::P30,
            0x6 => ResyncThreshold::P35,
            0x7 => ResyncThreshold::P40,
            0x8 => ResyncThreshold::P45,
            0x9 => ResyncThreshold::P50,
            0xA => ResyncThreshold::P55,
            0xB => ResyncThreshold::P60,
            0xC => ResyncThreshold::P70,
            0xD => ResyncThreshold::P80,
            0xE => ResyncThreshold::P90,
            0xF => ResyncThreshold::P100,
            _ => panic!("Invalid ResyncThreshold value: {}", value),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, strum::Display)]
#[repr(u8)]
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

impl From<u8> for BrakeCurrentThreshold {
    fn from(value: u8) -> Self {
        match value {
            0x0 => BrakeCurrentThreshold::A0_1,
            0x1 => BrakeCurrentThreshold::A0_2,
            0x2 => BrakeCurrentThreshold::A0_3,
            0x3 => BrakeCurrentThreshold::A0_5,
            0x4 => BrakeCurrentThreshold::A1_0,
            0x5 => BrakeCurrentThreshold::A2_0,
            0x6 => BrakeCurrentThreshold::A4_0,
            0x7 => BrakeCurrentThreshold::A8_0,
            _ => panic!("Invalid BrakeCurrentThreshold value: {}", value),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, strum::Display)]
#[repr(u8)]
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

impl From<u8> for TimeFormatA {
    fn from(value: u8) -> Self {
        match value {
            0x0 => TimeFormatA::MS10,
            0x1 => TimeFormatA::MS50,
            0x2 => TimeFormatA::MS100,
            0x3 => TimeFormatA::MS200,
            0x4 => TimeFormatA::MS300,
            0x5 => TimeFormatA::MS400,
            0x6 => TimeFormatA::MS500,
            0x7 => TimeFormatA::MS750,
            0x8 => TimeFormatA::S1,
            0x9 => TimeFormatA::S2,
            0xA => TimeFormatA::S3,
            0xB => TimeFormatA::S4,
            0xC => TimeFormatA::S5,
            0xD => TimeFormatA::S7_5,
            0xE => TimeFormatA::S10,
            0xF => TimeFormatA::S15,
            _ => panic!("Invalid TimeFormatA value: {}", value),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, strum::Display)]
#[repr(u8)]
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

impl From<u8> for BemfStationaryVoltageThreshold {
    fn from(value: u8) -> Self {
        match value {
            0x0 => BemfStationaryVoltageThreshold::MV50,
            0x1 => BemfStationaryVoltageThreshold::MV75,
            0x2 => BemfStationaryVoltageThreshold::MV100,
            0x3 => BemfStationaryVoltageThreshold::MV250,
            0x4 => BemfStationaryVoltageThreshold::MV500,
            0x5 => BemfStationaryVoltageThreshold::MV750,
            0x6 => BemfStationaryVoltageThreshold::MV1000,
            0x7 => BemfStationaryVoltageThreshold::MV1500,
            _ => panic!("Invalid BemfStationaryVoltageThreshold value: {}", value),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, strum::Display)]
#[repr(u8)]
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

impl From<u8> for ReverseDriveHandoffThreshold {
    fn from(value: u8) -> Self {
        match value {
            0x0 => ReverseDriveHandoffThreshold::P2_5,
            0x1 => ReverseDriveHandoffThreshold::P5,
            0x2 => ReverseDriveHandoffThreshold::P7_5,
            0x3 => ReverseDriveHandoffThreshold::P10,
            0x4 => ReverseDriveHandoffThreshold::P12_5,
            0x5 => ReverseDriveHandoffThreshold::P15,
            0x6 => ReverseDriveHandoffThreshold::P20,
            0x7 => ReverseDriveHandoffThreshold::P25,
            0x8 => ReverseDriveHandoffThreshold::P30,
            0x9 => ReverseDriveHandoffThreshold::P40,
            0xA => ReverseDriveHandoffThreshold::P50,
            0xB => ReverseDriveHandoffThreshold::P60,
            0xC => ReverseDriveHandoffThreshold::P70,
            0xD => ReverseDriveHandoffThreshold::P80,
            0xE => ReverseDriveHandoffThreshold::P90,
            0xF => ReverseDriveHandoffThreshold::P100,
            _ => panic!("Invalid ReverseDriveHandoffThreshold value: {}", value),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, strum::Display)]
#[repr(u8)]
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

impl From<u8> for ReverseDriveOpenLoopCurrent {
    fn from(value: u8) -> Self {
        match value {
            0x0 => ReverseDriveOpenLoopCurrent::A1_5,
            0x1 => ReverseDriveOpenLoopCurrent::A2_5,
            0x2 => ReverseDriveOpenLoopCurrent::A3_5,
            0x3 => ReverseDriveOpenLoopCurrent::A5_0,
            _ => panic!("Invalid ReverseDriveOpenLoopCurrent value: {}", value),
        }
    }
}