use super::*;

const GD_CONFIG2_RESET: u32 = 0b_00000001_01000000_00000000_00000000;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GdConfig2 {
    parity: bool,
    /// Buck power sequencing disable.
    /// 0 = Buck power sequencing is enabled,
    /// 1 = Buck power sequencing is disabled
    pub buck_ps_dis: bool,
    /// Buck current limit.
    /// 0 = Buck regulator current limit is set to 600 mA,
    /// 1 = Buck regulator current limit is set to 150 mA
    pub buck_cl: bool,
    /// Buck voltage
    pub buck_sel: BuckVoltage,
    /// Minimum ON time for low side MOSFET
    pub min_on_time: MinOnTime,
}

impl Register for GdConfig2 {
    const ADDRESS: u16 = GD_CONFIG2; // Example address, replace with actual address
}

impl From<GdConfig2> for u32 {
    fn from(config: GdConfig2) -> Self {
        let mut value = GD_CONFIG2_RESET;
        value |= (config.parity as u32) << 31;
        value |= (config.buck_ps_dis as u32) << 24;
        value |= (config.buck_cl as u32) << 23;
        value |= (config.buck_sel as u32) << 21;
        value |= (config.min_on_time as u32) << 17;
        value
    }
}

impl From<u32> for GdConfig2 {
    fn from(value: u32) -> Self {
        GdConfig2 {
            parity: (value >> 31) & 0x1 != 0,
            buck_ps_dis: (value >> 24) & 0x1 != 0,
            buck_cl: (value >> 23) & 0x1 != 0,
            buck_sel: BuckVoltage::from(((value >> 21) & 0x3) as u8),
            min_on_time: MinOnTime::from(((value >> 17) & 0x7) as u8),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, strum::Display)]
pub enum BuckVoltage {
    /// 3.3V
    #[strum(to_string = "3.3 V")]
    V3_3 = 0x0,
    /// 5.0V
    #[strum(to_string = "5.0 V")]
    V5_0 = 0x1,
    /// 4.0V
    #[strum(to_string = "4.0 V")]
    V4_0 = 0x2,
    /// 5.7V
    #[strum(to_string = "5.7 V")]
    V5_7 = 0x3,
}

impl PartialOrd for BuckVoltage {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for BuckVoltage {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        self.to_voltage().partial_cmp(&other.to_voltage()).unwrap()
    }
}

impl BuckVoltage {
    pub fn to_voltage(self) -> f32 {
        match self {
            BuckVoltage::V3_3 => 3.3,
            BuckVoltage::V5_0 => 5.0,
            BuckVoltage::V4_0 => 4.0,
            BuckVoltage::V5_7 => 5.7,
        }
    }
}

impl From<u8> for BuckVoltage {
    fn from(value: u8) -> Self {
        match value {
            0x0 => BuckVoltage::V3_3,
            0x1 => BuckVoltage::V5_0,
            0x2 => BuckVoltage::V4_0,
            0x3 => BuckVoltage::V5_7,
            _ => panic!("Invalid value for BuckVoltage: {}", value),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, strum::Display)]
pub enum MinOnTime {
    /// 0μs
    #[strum(to_string = "0µs")]
    Us0 = 0x0,
    /// Automatic based on Slew rate
    #[strum(to_string = "Auto")]
    Auto = 0x1,
    /// 0.5μs
    #[strum(to_string = "0.5µs")]
    Us0_5 = 0x2,
    /// 0.75μs
    #[strum(to_string = "0.75µs")]
    Us0_75 = 0x3,
    /// 1.0μs
    #[strum(to_string = "1.0µs")]
    Us1_0 = 0x4,
    /// 1.25μs
    #[strum(to_string = "1.25µs")]
    Us1_25 = 0x5,
    /// 1.5μs
    #[strum(to_string = "1.5µs")]
    Us1_5 = 0x6,
    /// 2.0μs
    #[strum(to_string = "2.0µs")]
    Us2_0 = 0x7,
}

impl From<u8> for MinOnTime {
    fn from(value: u8) -> Self {
        match value {
            0x0 => MinOnTime::Us0,
            0x1 => MinOnTime::Auto,
            0x2 => MinOnTime::Us0_5,
            0x3 => MinOnTime::Us0_75,
            0x4 => MinOnTime::Us1_0,
            0x5 => MinOnTime::Us1_25,
            0x6 => MinOnTime::Us1_5,
            0x7 => MinOnTime::Us2_0,
            _ => panic!("Invalid value for MinOnTime: {}", value),
        }
    }
}

impl PartialOrd for MinOnTime {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        match (self, other) {
            (MinOnTime::Auto, _) | (_, MinOnTime::Auto) => None,
            _ => Some((*self as u8).cmp(&(*other as u8))),
        }
    }
}