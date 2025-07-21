pub use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DeviceConfig1 {
    /// Selects between DAC2 and SOx channels
    pub dac_sox_sel: DacSoxSel,
    /// DAC1 and DAC2 enables.
    /// 0 = DACOUT1 and DACOUT2 on dedicated DAC pins disabled,
    /// 1 = DACOUT1 and DACOUT2 on dedicated DAC pins enabled
    pub dac_enable: bool,
    /// I2C target address
    pub i2c_target_address: u8,
    /// Slew rate control for I2C pins
    pub slew_rate_i2c_pins: SlewRate,
    /// Pull-up enable for nFAULT and FG pins.
    /// 0 = Disable, 1 = Enable
    pub pullup_enable: bool,
    /// Maximum DC bus voltage configuration
    pub bus_volt: MaxBusVoltage,
}

impl Register for DeviceConfig1 {
    const ADDRESS: u16 = DEVICE_CONFIG1; // Example address, replace with actual address
}

impl From<DeviceConfig1> for u32 {
    fn from(config: DeviceConfig1) -> Self {
        let mut value = 0;
        value |= (config.dac_sox_sel as u32) << 28;
        value |= (config.dac_enable as u32) << 27;
        value |= (config.i2c_target_address as u32 & 0x7F) << 20; // 7 bits
        value |= (config.slew_rate_i2c_pins as u32) << 3;
        value |= (config.pullup_enable as u32) << 2;
        value |= config.bus_volt as u32;
        value
    }
}

impl From<u32> for DeviceConfig1 {
    fn from(value: u32) -> Self {
        DeviceConfig1 {
            dac_sox_sel: DacSoxSel::from(((value >> 28) & 0x3) as u8),
            dac_enable: ((value >> 27) & 0x1) != 0,
            i2c_target_address: ((value >> 20) & 0x7F) as u8,
            slew_rate_i2c_pins: SlewRate::from(((value >> 3) & 0x3) as u8),
            pullup_enable: ((value >> 2) & 0x1) != 0,
            bus_volt: MaxBusVoltage::from((value & 0x3) as u8),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, strum::Display)]
#[repr(u8)]
pub enum DacSoxSel {
    /// DACOUT2
    #[strum(to_string = "DACOUT2")]
    DacOut2 = 0x0,
    /// SOA
    #[strum(to_string = "SOA")]
    Soa = 0x1,
    /// SOB
    #[strum(to_string = "SOB")]
    Sob = 0x2,
    /// SOC
    #[strum(to_string = "SOC")]
    Soc = 0x3,
}

impl From<u8> for DacSoxSel {
    fn from(value: u8) -> Self {
        match value {
            0x0 => DacSoxSel::DacOut2,
            0x1 => DacSoxSel::Soa,
            0x2 => DacSoxSel::Sob,
            0x3 => DacSoxSel::Soc,
            _ => panic!("Invalid DacSoxSel value"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, strum::Display)]
#[repr(u8)]
pub enum SlewRate {
    /// 4.8 mA
    #[strum(to_string = "4.8 mA")]
    M4_8 = 0x0,
    /// 3.9 mA
    #[strum(to_string = "3.9 mA")]
    M3_9 = 0x1,
    /// 1.86 mA
    #[strum(to_string = "1.86 mA")]
    M1_86 = 0x2,
    /// 30.8 mA
    #[strum(to_string = "30.8 mA")]
    M30_8 = 0x3,
}

impl From<u8> for SlewRate {
    fn from(value: u8) -> Self {
        match value {
            0x0 => SlewRate::M4_8,
            0x1 => SlewRate::M3_9,
            0x2 => SlewRate::M1_86,
            0x3 => SlewRate::M30_8,
            _ => panic!("Invalid SlewRate value"),
        }
    }
}

impl PartialOrd for SlewRate {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for SlewRate {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        self.to_amps().total_cmp(&other.to_amps())
    }
}

impl SlewRate {
    fn to_amps(self) -> f32 {
        match self {
            SlewRate::M4_8 => 4.8,
            SlewRate::M3_9 => 3.9,
            SlewRate::M1_86 => 1.86,
            SlewRate::M30_8 => 30.8,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, strum::Display)]
#[repr(u8)]
pub enum MaxBusVoltage {
    /// 15 V
    #[strum(to_string = "15 V")]
    V15 = 0x0,
    /// 30 V
    #[strum(to_string = "30 V")]
    V30 = 0x1,
    /// 60 V
    #[strum(to_string = "60 V")]
    V60 = 0x2,
    /// Not Defined
    #[strum(to_string = "Not Defined")]
    NotDefined = 0x3,
}

impl From<u8> for MaxBusVoltage {
    fn from(value: u8) -> Self {
        match value {
            0x0 => MaxBusVoltage::V15,
            0x1 => MaxBusVoltage::V30,
            0x2 => MaxBusVoltage::V60,
            0x3 => MaxBusVoltage::NotDefined,
            _ => panic!("Invalid MaxBusVoltage value"),
        }
    }
}

impl PartialOrd for MaxBusVoltage {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        match (self, other) {
            (MaxBusVoltage::NotDefined, _) | (_, MaxBusVoltage::NotDefined) => None,
            _ => Some((*self as u8).cmp(&(*other as u8))),
        }
    }
}
