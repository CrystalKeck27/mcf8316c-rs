//! Section 7.7.3.2

use super::*;
use arbitrary_int::*;
use bitbybit::*;

/// Register to configure device
#[bitfield(u32, default = 0x0)]
#[derive(Debug, PartialEq, Eq)]
pub struct DeviceConfig1 {
    /// Selects between DAC2 and SOx channels
    #[bits(28..=29, rw)]
    pub dac_sox_sel: DacSoxSel,
    /// DAC1 and DAC2 enables.
    /// 0 = DACOUT1 and DACOUT2 on dedicated DAC pins disabled,
    /// 1 = DACOUT1 and DACOUT2 on dedicated DAC pins enabled
    #[bit(27, rw)]
    pub dac_enable: bool,
    /// I2C target address
    #[bits(20..=26, rw)]
    pub i2c_target_address: u7,
    /// Slew rate control for I2C pins
    #[bits(3..=4, rw)]
    pub slew_rate_i2c_pins: SlewRate,
    /// Pull-up enable for nFAULT and FG pins.
    /// 0 = Disable, 1 = Enable
    #[bit(2, rw)]
    pub pullup_enable: bool,
    /// Maximum DC bus voltage configuration
    #[bits(0..=1, rw)]
    pub bus_volt: MaxBusVoltage,
}

impl Register for DeviceConfig1 {
    const ADDRESS: u12 = DEVICE_CONFIG1;

    fn value(&self) -> u32 {
        self.raw_value()
    }
}

/// Selects between DAC2 and SOx channels
#[bitenum(u2, exhaustive = true)]
#[derive(Debug, PartialEq, Eq, strum::Display)]
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

/// Slew rate control for I2C pins
#[bitenum(u2, exhaustive = true)]
#[derive(Debug, PartialEq, Eq, strum::Display)]
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

impl PartialOrd for SlewRate {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for SlewRate {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        self.to_milliamps().total_cmp(&other.to_milliamps())
    }
}

impl SlewRate {
    fn to_milliamps(self) -> f32 {
        match self {
            SlewRate::M4_8 => 4.8,
            SlewRate::M3_9 => 3.9,
            SlewRate::M1_86 => 1.86,
            SlewRate::M30_8 => 30.8,
        }
    }
}

/// Maximum DC bus voltage configuration
#[bitenum(u2, exhaustive = true)]
#[derive(Debug, PartialEq, Eq, strum::Display)]
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

impl PartialOrd for MaxBusVoltage {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        match (self, other) {
            (MaxBusVoltage::NotDefined, _) | (_, MaxBusVoltage::NotDefined) => None,
            _ => Some((*self as u8).cmp(&(*other as u8))),
        }
    }
}
