//! Section 7.7.3.6

use super::*;
use arbitrary_int::*;
use bitbybit::*;

/// Reset value for GD_CONFIG2 register
pub const GD_CONFIG2_RESET: u32 = 0b_00000001_01000000_00000000_00000000;

/// Register to configure gated driver settings2
#[bitfield(u32, default = GD_CONFIG2_RESET)]
#[derive(Debug, PartialEq, Eq)]
pub struct GdConfig2 {
    // #[bit(31, rw)]
    // parity: bool,
    /// Buck power sequencing disable.
    /// 0 = Buck power sequencing is enabled,
    /// 1 = Buck power sequencing is disabled
    ///
    /// # This bit is write 1 to clear
    /// The value sent over the i2c bus is inverted from whatever is set here.
    /// This should make its behavior consistent with the other registers
    /// with the notable exception that you cannot set the value to 1.
    #[bit(24, rw)]
    pub buck_ps_dis: bool,
    /// Buck current limit.
    /// 0 = Buck regulator current limit is set to 600 mA,
    /// 1 = Buck regulator current limit is set to 150 mA
    #[bit(23, rw)]
    pub buck_cl: bool,
    /// Buck voltage
    #[bits(21..=22, rw)]
    pub buck_sel: BuckVoltage,
    /// Minimum ON time for low side MOSFET
    #[bits(17..=19, rw)]
    pub min_on_time: MinOnTime,
}

impl Register for GdConfig2 {
    const ADDRESS: u12 = GD_CONFIG2;

    fn value(&self) -> u32 {
        let mut value = self.raw_value();
        // calculate parity before flipping the buck_ps_dis bit
        if value.count_ones() % 2 == 1 {
            // If the parity bit is not set, we set it to 1
            value |= 0x8000_0000; // Set the parity bit
        }
        // TODO: Verify that this is the correct way to handle the buck_ps_dis bit
        value ^ (1 << 24) // Invert the buck_ps_dis bit
    }
}

/// Buck voltage
#[bitenum(u2, exhaustive = true)]
#[derive(Debug, PartialEq, Eq, strum::Display)]
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
    /// Convert the enum variant to its corresponding voltage value.
    pub fn to_voltage(self) -> f32 {
        match self {
            BuckVoltage::V3_3 => 3.3,
            BuckVoltage::V5_0 => 5.0,
            BuckVoltage::V4_0 => 4.0,
            BuckVoltage::V5_7 => 5.7,
        }
    }
}

/// Minimum ON time for low side MOSFET
#[bitenum(u3, exhaustive = true)]
#[derive(Debug, PartialEq, Eq, strum::Display)]
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

impl PartialOrd for MinOnTime {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        match (self, other) {
            (MinOnTime::Auto, _) | (_, MinOnTime::Auto) => None,
            _ => Some((*self as u8).cmp(&(*other as u8))),
        }
    }
}
