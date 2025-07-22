use super::*;
use bitbybit::*;

#[bitfield(u32, default = 0x0)]
#[derive(Debug, PartialEq, Eq)]
pub struct ClosedLoop3 {
    /// 8-bit values for motor BEMF Constant
    #[bits(23..=30, rw)]
    pub motor_bemf_const: MotorBemf,
    /// 10-bit value for current Iq and Id loop Kp. 0 = Auto
    #[bits(13..=22, rw)]
    pub curr_loop_kp: KVal,
    /// 10-bit value for current Iq and Id loop Ki. 0 = Auto
    #[bits(3..=12, rw)]
    pub curr_loop_ki: KVal,
    /// 3 MSB for speed loop Kp.
    #[bits(0..=2, rw)]
    pub spd_loop_kp: SpeedLoopKpHigh3,
}

impl Register for ClosedLoop3 {
    const ADDRESS: u16 = CLOSED_LOOP3;

    fn value(&self) -> u32 {
        self.raw_value()
    }
}

#[bitenum(u8, exhaustive = true)]
#[derive(Debug, PartialEq, Eq, strum::Display)]
pub enum MotorBemf {
    /// Self Measurement (see Motor Parameter Extraction Tool (MPET))
    #[strum(serialize = "Self Measurement")]
    SelfMeasurement = 0x0,
    /// 0.6 mV/Hz
    #[strum(serialize = "0.6 mV/Hz")]
    B0_6 = 0x1,
    /// 0.7 mV/Hz
    #[strum(serialize = "0.7 mV/Hz")]
    B0_7 = 0x2,
    /// 0.8 mV/Hz
    #[strum(serialize = "0.8 mV/Hz")]
    B0_8 = 0x3,
    /// 0_9 mV/Hz
    #[strum(serialize = "0.9 mV/Hz")]
    B0_9 = 0x4,
    /// 1.0 mV/Hz
    #[strum(serialize = "1.0 mV/Hz")]
    B1_0 = 0x5,
    /// 1.1 mV/Hz
    #[strum(serialize = "1.1 mV/Hz")]
    B1_1 = 0x6,
    /// 1.2 mV/Hz
    #[strum(serialize = "1.2 mV/Hz")]
    B1_2 = 0x7,
    /// 1.3 mV/Hz
    #[strum(serialize = "1.3 mV/Hz")]
    B1_3 = 0x8,
    /// 1.4 mV/Hz
    #[strum(serialize = "1.4 mV/Hz")]
    B1_4 = 0x9,
    /// 1.5 mV/Hz
    #[strum(serialize = "1.5 mV/Hz")]
    B1_5 = 0xA,
    /// 1.6 mV/Hz
    #[strum(serialize = "1.6 mV/Hz")]
    B1_6 = 0xB,
    /// 1.7 mV/Hz
    #[strum(serialize = "1.7 mV/Hz")]
    B1_7 = 0xC,
    /// 1.8 mV/Hz
    #[strum(serialize = "1.8 mV/Hz")]
    B1_8 = 0xD,
    /// 1.9 mV/Hz
    #[strum(serialize = "1.9 mV/Hz")]
    B1_9 = 0xE,
    /// 2.0 mV/Hz
    #[strum(serialize = "2.0 mV/Hz")]
    B2_0 = 0xF,
    /// 2.2 mV/Hz
    #[strum(serialize = "2.2 mV/Hz")]
    B2_2 = 0x10,
    /// 2.4 mV/Hz
    #[strum(serialize = "2.4 mV/Hz")]
    B2_4 = 0x11,
    /// 2.6 mV/Hz
    #[strum(serialize = "2.6 mV/Hz")]
    B2_6 = 0x12,
    /// 2.8 mV/Hz
    #[strum(serialize = "2.8 mV/Hz")]
    B2_8 = 0x13,
    /// 3.0 mV/Hz
    #[strum(serialize = "3.0 mV/Hz")]
    B3_0 = 0x14,
    /// 3.2 mV/Hz
    #[strum(serialize = "3.2 mV/Hz")]
    B3_2 = 0x15,
    /// 3.4 mV/Hz
    #[strum(serialize = "3.4 mV/Hz")]
    B3_4 = 0x16,
    /// 3.6 mV/Hz
    #[strum(serialize = "3.6 mV/Hz")]
    B3_6 = 0x17,
    /// 3.8 mV/Hz
    #[strum(serialize = "3.8 mV/Hz")]
    B3_8 = 0x18,
    /// 4.0 mV/Hz
    #[strum(serialize = "4.0 mV/Hz")]
    B4_0 = 0x19,
    /// 4.2 mV/Hz
    #[strum(serialize = "4.2 mV/Hz")]
    B4_2 = 0x1A,
    /// 4.4 mV/Hz
    #[strum(serialize = "4.4 mV/Hz")]
    B4_4 = 0x1B,
    /// 4.6 mV/Hz
    #[strum(serialize = "4.6 mV/Hz")]
    B4_6 = 0x1C,
    /// 4.8 mV/Hz
    #[strum(serialize = "4.8 mV/Hz")]
    B4_8 = 0x1D,
    /// 5.0 mV/Hz
    #[strum(serialize = "5.0 mV/Hz")]
    B5_0 = 0x1E,
    /// 5.2 mV/Hz
    #[strum(serialize = "5.2 mV/Hz")]
    B5_2 = 0x1F,
    /// 5.4 mV/Hz
    #[strum(serialize = "5.4 mV/Hz")]
    B5_4 = 0x20,
    /// 5.6 mV/Hz
    #[strum(serialize = "5.6 mV/Hz")]
    B5_6 = 0x21,
    /// 5.8 mV/Hz
    #[strum(serialize = "5.8 mV/Hz")]
    B5_8 = 0x22,
    /// 6.0 mV/Hz
    #[strum(serialize = "6.0 mV/Hz")]
    B6_0 = 0x23,
    /// 6.2 mV/Hz
    #[strum(serialize = "6.2 mV/Hz")]
    B6_2 = 0x24,
    /// 6.4 mV/Hz
    #[strum(serialize = "6.4 mV/Hz")]
    B6_4 = 0x25,
    /// 6.6 mV/Hz
    #[strum(serialize = "6.6 mV/Hz")]
    B6_6 = 0x26,
    /// 6.8 mV/Hz
    #[strum(serialize = "6.8 mV/Hz")]
    B6_8 = 0x27,
    /// 7.0 mV/Hz
    #[strum(serialize = "7.0 mV/Hz")]
    B7_0 = 0x28,
    /// 7.2 mV/Hz
    #[strum(serialize = "7.2 mV/Hz")]
    B7_2 = 0x29,
    /// 7.4 mV/Hz
    #[strum(serialize = "7.4 mV/Hz")]
    B7_4 = 0x2A,
    /// 7.6 mV/Hz
    #[strum(serialize = "7.6 mV/Hz")]
    B7_6 = 0x2B,
    /// 7.8 mV/Hz
    #[strum(serialize = "7.8 mV/Hz")]
    B7_8 = 0x2C,
    /// 8.0 mV/Hz
    #[strum(serialize = "8.0 mV/Hz")]
    B8_0 = 0x2D,
    /// 8.2 mV/Hz
    #[strum(serialize = "8.2 mV/Hz")]
    B8_2 = 0x2E,
    /// 8.4 mV/Hz
    #[strum(serialize = "8.4 mV/Hz")]
    B8_4 = 0x2F,
    /// 8.6 mV/Hz
    #[strum(serialize = "8.6 mV/Hz")]
    B8_6 = 0x30,
    /// 8.8 mV/Hz
    #[strum(serialize = "8.8 mV/Hz")]
    B8_8 = 0x31,
    /// 9.0 mV/Hz
    #[strum(serialize = "9.0 mV/Hz")]
    B9_0 = 0x32,
    /// 9.2 mV/Hz
    #[strum(serialize = "9.2 mV/Hz")]
    B9_2 = 0x33,
    /// 9.4 mV/Hz
    #[strum(serialize = "9.4 mV/Hz")]
    B9_4 = 0x34,
    /// 9.6 mV/Hz
    #[strum(serialize = "9.6 mV/Hz")]
    B9_6 = 0x35,
    /// 9.8 mV/Hz
    #[strum(serialize = "9.8 mV/Hz")]
    B9_8 = 0x36,
    /// 10.0 mV/Hz
    #[strum(serialize = "10.0 mV/Hz")]
    B10_0 = 0x37,
    /// 10.5 mV/Hz
    #[strum(serialize = "10.5 mV/Hz")]
    B10_5 = 0x38,
    /// 11.0 mV/Hz
    #[strum(serialize = "11.0 mV/Hz")]
    B11_0 = 0x39,
    /// 11.5 mV/Hz
    #[strum(serialize = "11.5 mV/Hz")]
    B11_5 = 0x3A,
    /// 12.0 mV/Hz
    #[strum(serialize = "12.0 mV/Hz")]
    B12_0 = 0x3B,
    /// 12.5 mV/Hz
    #[strum(serialize = "12.5 mV/Hz")]
    B12_5 = 0x3C,
    /// 13.0 mV/Hz
    #[strum(serialize = "13.0 mV/Hz")]
    B13_0 = 0x3D,
    /// 13.5 mV/Hz
    #[strum(serialize = "13.5 mV/Hz")]
    B13_5 = 0x3E,
    /// 14.0 mV/Hz
    #[strum(serialize = "14.0 mV/Hz")]
    B14_0 = 0x3F,
    /// 14.5 mV/Hz
    #[strum(serialize = "14.5 mV/Hz")]
    B14_5 = 0x40,
    /// 15.0 mV/Hz
    #[strum(serialize = "15.0 mV/Hz")]
    B15_0 = 0x41,
    /// 15.5 mV/Hz
    #[strum(serialize = "15.5 mV/Hz")]
    B15_5 = 0x42,
    /// 16.0 mV/Hz
    #[strum(serialize = "16.0 mV/Hz")]
    B16_0 = 0x43,
    /// 16.5 mV/Hz
    #[strum(serialize = "16.5 mV/Hz")]
    B16_5 = 0x44,
    /// 17.0 mV/Hz
    #[strum(serialize = "17.0 mV/Hz")]
    B17_0 = 0x45,
    /// 17.5 mV/Hz
    #[strum(serialize = "17.5 mV/Hz")]
    B17_5 = 0x46,
    /// 18.0 mV/Hz
    #[strum(serialize = "18.0 mV/Hz")]
    B18_0 = 0x47,
    /// 18.5 mV/Hz
    #[strum(serialize = "18.5 mV/Hz")]
    B18_5 = 0x48,
    /// 19.0 mV/Hz
    #[strum(serialize = "19.0 mV/Hz")]
    B19_0 = 0x49,
    /// 19.5 mV/Hz
    #[strum(serialize = "19.5 mV/Hz")]
    B19_5 = 0x4A,
    /// 20.0 mV/Hz
    #[strum(serialize = "20.0 mV/Hz")]
    B20_0 = 0x4B,
    /// 20.5 mV/Hz
    #[strum(serialize = "20.5 mV/Hz")]
    B20_5 = 0x4C,
    /// 21.0 mV/Hz
    #[strum(serialize = "21.0 mV/Hz")]
    B21_0 = 0x4D,
    /// 21.5 mV/Hz
    #[strum(serialize = "21.5 mV/Hz")]
    B21_5 = 0x4E,
    /// 22.0 mV/Hz
    #[strum(serialize = "22.0 mV/Hz")]
    B22_0 = 0x4F,
    /// 22.5 mV/Hz
    #[strum(serialize = "22.5 mV/Hz")]
    B22_5 = 0x50,
    /// 23.0 mV/Hz
    #[strum(serialize = "23.0 mV/Hz")]
    B23_0 = 0x51,
    /// 23.5 mV/Hz
    #[strum(serialize = "23.5 mV/Hz")]
    B23_5 = 0x52,
    /// 24.0 mV/Hz
    #[strum(serialize = "24.0 mV/Hz")]
    B24_0 = 0x53,
    /// 24.5 mV/Hz
    #[strum(serialize = "24.5 mV/Hz")]
    B24_5 = 0x54,
    /// 25.0 mV/Hz
    #[strum(serialize = "25.0 mV/Hz")]
    B25_0 = 0x55,
    /// 25.5 mV/Hz
    #[strum(serialize = "25.5 mV/Hz")]
    B25_5 = 0x56,
    /// 26.0 mV/Hz
    #[strum(serialize = "26.0 mV/Hz")]
    B26_0 = 0x57,
    /// 26.5 mV/Hz
    #[strum(serialize = "26.5 mV/Hz")]
    B26_5 = 0x58,
    /// 27.0 mV/Hz
    #[strum(serialize = "27.0 mV/Hz")]
    B27_0 = 0x59,
    /// 27.5 mV/Hz
    #[strum(serialize = "27.5 mV/Hz")]
    B27_5 = 0x5A,
    /// 28.0 mV/Hz
    #[strum(serialize = "28.0 mV/Hz")]
    B28_0 = 0x5B,
    /// 28.5 mV/Hz
    #[strum(serialize = "28.5 mV/Hz")]
    B28_5 = 0x5C,
    /// 29.0 mV/Hz
    #[strum(serialize = "29.0 mV/Hz")]
    B29_0 = 0x5D,
    /// 29.5 mV/Hz
    #[strum(serialize = "29.5 mV/Hz")]
    B29_5 = 0x5E,
    /// 30.0 mV/Hz
    #[strum(serialize = "30.0 mV/Hz")]
    B30_0 = 0x5F,
    /// 30.5 mV/Hz
    #[strum(serialize = "30.5 mV/Hz")]
    B30_5 = 0x60,
    /// 31.0 mV/Hz
    #[strum(serialize = "31.0 mV/Hz")]
    B31_0 = 0x61,
    /// 31.5 mV/Hz
    #[strum(serialize = "31.5 mV/Hz")]
    B31_5 = 0x62,
    /// 32.0 mV/Hz
    #[strum(serialize = "32.0 mV/Hz")]
    B32_0 = 0x63,
    /// 32.5 mV/Hz
    #[strum(serialize = "32.5 mV/Hz")]
    B32_5 = 0x64,
    /// 33.0 mV/Hz
    #[strum(serialize = "33.0 mV/Hz")]
    B33_0 = 0x65,
    /// 33.5 mV/Hz
    #[strum(serialize = "33.5 mV/Hz")]
    B33_5 = 0x66,
    /// 34.0 mV/Hz
    #[strum(serialize = "34.0 mV/Hz")]
    B34_0 = 0x67,
    /// 34.5 mV/Hz
    #[strum(serialize = "34.5 mV/Hz")]
    B34_5 = 0x68,
    /// 35.0 mV/Hz
    #[strum(serialize = "35.0 mV/Hz")]
    B35_0 = 0x69,
    /// 35.5 mV/Hz
    #[strum(serialize = "35.5 mV/Hz")]
    B35_5 = 0x6A,
    /// 36.0 mV/Hz
    #[strum(serialize = "36.0 mV/Hz")]
    B36_0 = 0x6B,
    /// 36.5 mV/Hz
    #[strum(serialize = "36.5 mV/Hz")]
    B36_5 = 0x6C,
    /// 37.0 mV/Hz
    #[strum(serialize = "37.0 mV/Hz")]
    B37_0 = 0x6D,
    /// 37.5 mV/Hz
    #[strum(serialize = "37.5 mV/Hz")]
    B37_5 = 0x6E,
    /// 38.0 mV/Hz
    #[strum(serialize = "38.0 mV/Hz")]
    B38_0 = 0x6F,
    /// 38.5 mV/Hz
    #[strum(serialize = "38.5 mV/Hz")]
    B38_5 = 0x70,
    /// 39.0 mV/Hz
    #[strum(serialize = "39.0 mV/Hz")]
    B39_0 = 0x71,
    /// 39.5 mV/Hz
    #[strum(serialize = "39.5 mV/Hz")]
    B39_5 = 0x72,
    /// 40.0 mV/Hz
    #[strum(serialize = "40.0 mV/Hz")]
    B40_0 = 0x73,
    /// 40.5 mV/Hz
    #[strum(serialize = "40.5 mV/Hz")]
    B40_5 = 0x74,
    /// 41.0 mV/Hz
    #[strum(serialize = "41.0 mV/Hz")]
    B41_0 = 0x75,
    /// 41.5 mV/Hz
    #[strum(serialize = "41.5 mV/Hz")]
    B41_5 = 0x76,
    /// 42.0 mV/Hz
    #[strum(serialize = "42.0 mV/Hz")]
    B42_0 = 0x77,
    /// 42.5 mV/Hz
    #[strum(serialize = "42.5 mV/Hz")]
    B42_5 = 0x78,
    /// 43.0 mV/Hz
    #[strum(serialize = "43.0 mV/Hz")]
    B43_0 = 0x79,
    /// 43.5 mV/Hz
    #[strum(serialize = "43.5 mV/Hz")]
    B43_5 = 0x7A,
    /// 44.0 mV/Hz
    #[strum(serialize = "44.0 mV/Hz")]
    B44_0 = 0x7B,
    /// 44.5 mV/Hz
    #[strum(serialize = "44.5 mV/Hz")]
    B44_5 = 0x7C,
    /// 45.0 mV/Hz
    #[strum(serialize = "45.0 mV/Hz")]
    B45_0 = 0x7D,
    /// 45.5 mV/Hz
    #[strum(serialize = "45.5 mV/Hz")]
    B45_5 = 0x7E,
    /// 46.0 mV/Hz
    #[strum(serialize = "46.0 mV/Hz")]
    B46_0 = 0x7F,
    /// 46.5 mV/Hz
    #[strum(serialize = "46.5 mV/Hz")]
    B46_5 = 0x80,
    /// 47.0 mV/Hz
    #[strum(serialize = "47.0 mV/Hz")]
    B47_0 = 0x81,
    /// 47.5 mV/Hz
    #[strum(serialize = "47.5 mV/Hz")]
    B47_5 = 0x82,
    /// 48.0 mV/Hz
    #[strum(serialize = "48.0 mV/Hz")]
    B48_0 = 0x83,
    /// 48.5 mV/Hz
    #[strum(serialize = "48.5 mV/Hz")]
    B48_5 = 0x84,
    /// 49.0 mV/Hz
    #[strum(serialize = "49.0 mV/Hz")]
    B49_0 = 0x85,
    /// 49.5 mV/Hz
    #[strum(serialize = "49.5 mV/Hz")]
    B49_5 = 0x86,
    /// 50.0 mV/Hz
    #[strum(serialize = "50.0 mV/Hz")]
    B50 = 0x87,
    /// 51.0 mV/Hz
    #[strum(serialize = "51.0 mV/Hz")]
    B51 = 0x88,
    /// 52.0 mV/Hz
    #[strum(serialize = "52.0 mV/Hz")]
    B52 = 0x89,
    /// 53.0 mV/Hz
    #[strum(serialize = "53.0 mV/Hz")]
    B53 = 0x8A,
    /// 54.0 mV/Hz
    #[strum(serialize = "54.0 mV/Hz")]
    B54 = 0x8B,
    /// 55.0 mV/Hz
    #[strum(serialize = "55.0 mV/Hz")]
    B55 = 0x8C,
    /// 56.0 mV/Hz
    #[strum(serialize = "56.0 mV/Hz")]
    B56 = 0x8D,
    /// 57.0 mV/Hz
    #[strum(serialize = "57.0 mV/Hz")]
    B57 = 0x8E,
    /// 58.0 mV/Hz
    #[strum(serialize = "58.0 mV/Hz")]
    B58 = 0x8F,
    /// 59.0 mV/Hz
    #[strum(serialize = "59.0 mV/Hz")]
    B59 = 0x90,
    /// 60.0 mV/Hz
    #[strum(serialize = "60.0 mV/Hz")]
    B60 = 0x91,
    /// 61.0 mV/Hz
    #[strum(serialize = "61.0 mV/Hz")]
    B61 = 0x92,
    /// 62.0 mV/Hz
    #[strum(serialize = "62.0 mV/Hz")]
    B62 = 0x93,
    /// 63.0 mV/Hz
    #[strum(serialize = "63.0 mV/Hz")]
    B63 = 0x94,
    /// 64.0 mV/Hz
    #[strum(serialize = "64.0 mV/Hz")]
    B64 = 0x95,
    /// 65.0 mV/Hz
    #[strum(serialize = "65.0 mV/Hz")]
    B65 = 0x96,
    /// 66.0 mV/Hz
    #[strum(serialize = "66.0 mV/Hz")]
    B66 = 0x97,
    /// 67.0 mV/Hz
    #[strum(serialize = "67.0 mV/Hz")]
    B67 = 0x98,
    /// 68.0 mV/Hz
    #[strum(serialize = "68.0 mV/Hz")]
    B68 = 0x99,
    /// 69.0 mV/Hz
    #[strum(serialize = "69.0 mV/Hz")]
    B69 = 0x9A,
    /// 70.0 mV/Hz
    #[strum(serialize = "70.0 mV/Hz")]
    B70 = 0x9B,
    /// 72.0 mV/Hz
    #[strum(serialize = "72.0 mV/Hz")]
    B72 = 0x9C,
    /// 74.0 mV/Hz
    #[strum(serialize = "74.0 mV/Hz")]
    B74 = 0x9D,
    /// 76.0 mV/Hz
    #[strum(serialize = "76.0 mV/Hz")]
    B76 = 0x9E,
    /// 78.0 mV/Hz
    #[strum(serialize = "78.0 mV/Hz")]
    B78 = 0x9F,
    /// 80.0 mV/Hz
    #[strum(serialize = "80.0 mV/Hz")]
    B80 = 0xA0,
    /// 82.0 mV/Hz
    #[strum(serialize = "82.0 mV/Hz")]
    B82 = 0xA1,
    /// 84.0 mV/Hz
    #[strum(serialize = "84.0 mV/Hz")]
    B84 = 0xA2,
    /// 86.0 mV/Hz
    #[strum(serialize = "86.0 mV/Hz")]
    B86 = 0xA3,
    /// 88.0 mV/Hz
    #[strum(serialize = "88.0 mV/Hz")]
    B88 = 0xA4,
    /// 90.0 mV/Hz
    #[strum(serialize = "90.0 mV/Hz")]
    B90 = 0xA5,
    /// 92.0 mV/Hz
    #[strum(serialize = "92.0 mV/Hz")]
    B92 = 0xA6,
    /// 94.0 mV/Hz
    #[strum(serialize = "94.0 mV/Hz")]
    B94 = 0xA7,
    /// 96.0 mV/Hz
    #[strum(serialize = "96.0 mV/Hz")]
    B96 = 0xA8,
    /// 98.0 mV/Hz
    #[strum(serialize = "98.0 mV/Hz")]
    B98 = 0xA9,
    /// 100.0 mV/Hz
    #[strum(serialize = "100.0 mV/Hz")]
    B100 = 0xAA,
    /// 105.0 mV/Hz
    #[strum(serialize = "105.0 mV/Hz")]
    B105 = 0xAB,
    /// 110.0 mV/Hz
    #[strum(serialize = "110.0 mV/Hz")]
    B110 = 0xAC,
    /// 115.0 mV/Hz
    #[strum(serialize = "115.0 mV/Hz")]
    B115 = 0xAD,
    /// 120.0 mV/Hz
    #[strum(serialize = "120.0 mV/Hz")]
    B120 = 0xAE,
    /// 125.0 mV/Hz
    #[strum(serialize = "125.0 mV/Hz")]
    B125 = 0xAF,
    /// 130.0 mV/Hz
    #[strum(serialize = "130.0 mV/Hz")]
    B130 = 0xB0,
    /// 135.0 mV/Hz
    #[strum(serialize = "135.0 mV/Hz")]
    B135 = 0xB1,
    /// 140.0 mV/Hz
    #[strum(serialize = "140.0 mV/Hz")]
    B140 = 0xB2,
    /// 145.0 mV/Hz
    #[strum(serialize = "145.0 mV/Hz")]
    B145 = 0xB3,
    /// 150.0 mV/Hz
    #[strum(serialize = "150.0 mV/Hz")]
    B150 = 0xB4,
    /// 155.0 mV/Hz
    #[strum(serialize = "155.0 mV/Hz")]
    B155 = 0xB5,
    /// 160.0 mV/Hz
    #[strum(serialize = "160.0 mV/Hz")]
    B160 = 0xB6,
    /// 165.0 mV/Hz
    #[strum(serialize = "165.0 mV/Hz")]
    B165 = 0xB7,
    /// 170.0 mV/Hz
    #[strum(serialize = "170.0 mV/Hz")]
    B170 = 0xB8,
    /// 175.0 mV/Hz
    #[strum(serialize = "175.0 mV/Hz")]
    B175 = 0xB9,
    /// 180.0 mV/Hz
    #[strum(serialize = "180.0 mV/Hz")]
    B180 = 0xBA,
    /// 185.0 mV/Hz
    #[strum(serialize = "185.0 mV/Hz")]
    B185 = 0xBB,
    /// 190.0 mV/Hz
    #[strum(serialize = "190.0 mV/Hz")]
    B190 = 0xBC,
    /// 195.0 mV/Hz
    #[strum(serialize = "195.0 mV/Hz")]
    B195 = 0xBD,
    /// 200.0 mV/Hz
    #[strum(serialize = "200.0 mV/Hz")]
    B200 = 0xBE,
    /// 205.0 mV/Hz
    #[strum(serialize = "205.0 mV/Hz")]
    B205 = 0xBF,
    /// 210.0 mV/Hz
    #[strum(serialize = "210.0 mV/Hz")]
    B210 = 0xC0,
    /// 220.0 mV/Hz
    #[strum(serialize = "220.0 mV/Hz")]
    B220 = 0xC1,
    /// 230.0 mV/Hz
    #[strum(serialize = "230.0 mV/Hz")]
    B230 = 0xC2,
    /// 240.0 mV/Hz
    #[strum(serialize = "240.0 mV/Hz")]
    B240 = 0xC3,
    /// 250.0 mV/Hz
    #[strum(serialize = "250.0 mV/Hz")]
    B250 = 0xC4,
    /// 260.0 mV/Hz
    #[strum(serialize = "260.0 mV/Hz")]
    B260 = 0xC5,
    /// 270.0 mV/Hz
    #[strum(serialize = "270.0 mV/Hz")]
    B270 = 0xC6,
    /// 280.0 mV/Hz
    #[strum(serialize = "280.0 mV/Hz")]
    B280 = 0xC7,
    /// 290.0 mV/Hz
    #[strum(serialize = "290.0 mV/Hz")]
    B290 = 0xC8,
    /// 300.0 mV/Hz
    #[strum(serialize = "300.0 mV/Hz")]
    B300 = 0xC9,
    /// 320.0 mV/Hz
    #[strum(serialize = "320.0 mV/Hz")]
    B320 = 0xCA,
    /// 340.0 mV/Hz
    #[strum(serialize = "340.0 mV/Hz")]
    B340 = 0xCB,
    /// 360.0 mV/Hz
    #[strum(serialize = "360.0 mV/Hz")]
    B360 = 0xCC,
    /// 380.0 mV/Hz
    #[strum(serialize = "380.0 mV/Hz")]
    B380 = 0xCD,
    /// 400.0 mV/Hz
    #[strum(serialize = "400.0 mV/Hz")]
    B400 = 0xCE,
    /// 420.0 mV/Hz
    #[strum(serialize = "420.0 mV/Hz")]
    B420 = 0xCF,
    /// 440.0 mV/Hz
    #[strum(serialize = "440.0 mV/Hz")]
    B440 = 0xD0,
    /// 460.0 mV/Hz
    #[strum(serialize = "460.0 mV/Hz")]
    B460 = 0xD1,
    /// 480.0 mV/Hz
    #[strum(serialize = "480.0 mV/Hz")]
    B480 = 0xD2,
    /// 500.0 mV/Hz
    #[strum(serialize = "500.0 mV/Hz")]
    B500 = 0xD3,
    /// 520.0 mV/Hz
    #[strum(serialize = "520.0 mV/Hz")]
    B520 = 0xD4,
    /// 540.0 mV/Hz
    #[strum(serialize = "540.0 mV/Hz")]
    B540 = 0xD5,
    /// 560.0 mV/Hz
    #[strum(serialize = "560.0 mV/Hz")]
    B560 = 0xD6,
    /// 580.0 mV/Hz
    #[strum(serialize = "580.0 mV/Hz")]
    B580 = 0xD7,
    /// 600.0 mV/Hz
    #[strum(serialize = "600.0 mV/Hz")]
    B600 = 0xD8,
    /// 620.0 mV/Hz
    #[strum(serialize = "620.0 mV/Hz")]
    B620 = 0xD9,
    /// 640.0 mV/Hz
    #[strum(serialize = "640.0 mV/Hz")]
    B640 = 0xDA,
    /// 660.0 mV/Hz
    #[strum(serialize = "660.0 mV/Hz")]
    B660 = 0xDB,
    /// 680.0 mV/Hz
    #[strum(serialize = "680.0 mV/Hz")]
    B680 = 0xDC,
    /// 700.0 mV/Hz
    #[strum(serialize = "700.0 mV/Hz")]
    B700 = 0xDD,
    /// 720.0 mV/Hz
    #[strum(serialize = "720.0 mV/Hz")]
    B720 = 0xDE,
    /// 740.0 mV/Hz
    #[strum(serialize = "740.0 mV/Hz")]
    B740 = 0xDF,
    /// 760.0 mV/Hz
    #[strum(serialize = "760.0 mV/Hz")]
    B760 = 0xE0,
    /// 780.0 mV/Hz
    #[strum(serialize = "780.0 mV/Hz")]
    B780 = 0xE1,
    /// 800.0 mV/Hz
    #[strum(serialize = "800.0 mV/Hz")]
    B800 = 0xE2,
    /// 820.0 mV/Hz
    #[strum(serialize = "820.0 mV/Hz")]
    B820 = 0xE3,
    /// 840.0 mV/Hz
    #[strum(serialize = "840.0 mV/Hz")]
    B840 = 0xE4,
    /// 860.0 mV/Hz
    #[strum(serialize = "860.0 mV/Hz")]
    B860 = 0xE5,
    /// 880.0 mV/Hz
    #[strum(serialize = "880.0 mV/Hz")]
    B880 = 0xE6,
    /// 900.0 mV/Hz
    #[strum(serialize = "900.0 mV/Hz")]
    B900 = 0xE7,
    /// 920.0 mV/Hz
    #[strum(serialize = "920.0 mV/Hz")]
    B920 = 0xE8,
    /// 940.0 mV/Hz
    #[strum(serialize = "940.0 mV/Hz")]
    B940 = 0xE9,
    /// 960.0 mV/Hz
    #[strum(serialize = "960.0 mV/Hz")]
    B960 = 0xEA,
    /// 980.0 mV/Hz
    #[strum(serialize = "980.0 mV/Hz")]
    B980 = 0xEB,
    /// 1000.0 mV/Hz
    #[strum(serialize = "1000.0 mV/Hz")]
    B1000 = 0xEC,
    /// 1050.0 mV/Hz
    #[strum(serialize = "1050.0 mV/Hz")]
    B1050 = 0xED,
    /// 1100.0 mV/Hz
    #[strum(serialize = "1100.0 mV/Hz")]
    B1100 = 0xEE,
    /// 1150.0 mV/Hz
    #[strum(serialize = "1150.0 mV/Hz")]
    B1150 = 0xEF,
    /// 1200.0 mV/Hz
    #[strum(serialize = "1200.0 mV/Hz")]
    B1200 = 0xF0,
    /// 1250.0 mV/Hz
    #[strum(serialize = "1250.0 mV/Hz")]
    B1250 = 0xF1,
    /// 1300.0 mV/Hz
    #[strum(serialize = "1300.0 mV/Hz")]
    B1300 = 0xF2,
    /// 1350.0 mV/Hz
    #[strum(serialize = "1350.0 mV/Hz")]
    B1350 = 0xF3,
    /// 1400.0 mV/Hz
    #[strum(serialize = "1400.0 mV/Hz")]
    B1400 = 0xF4,
    /// 1450.0 mV/Hz
    #[strum(serialize = "1450.0 mV/Hz")]
    B1450 = 0xF5,
    /// 1500.0 mV/Hz
    #[strum(serialize = "1500.0 mV/Hz")]
    B1500 = 0xF6,
    /// 1550.0 mV/Hz
    #[strum(serialize = "1550.0 mV/Hz")]
    B1550 = 0xF7,
    /// 1600.0 mV/Hz
    #[strum(serialize = "1600.0 mV/Hz")]
    B1600 = 0xF8,
    /// 1650.0 mV/Hz
    #[strum(serialize = "1650.0 mV/Hz")]
    B1650 = 0xF9,
    /// 1700.0 mV/Hz
    #[strum(serialize = "1700.0 mV/Hz")]
    B1700 = 0xFA,
    /// 1750.0 mV/Hz
    #[strum(serialize = "1750.0 mV/Hz")]
    B1750 = 0xFB,
    /// 1800.0 mV/Hz
    #[strum(serialize = "1800.0 mV/Hz")]
    B1800 = 0xFC,
    /// 1850.0 mV/Hz
    #[strum(serialize = "1850.0 mV/Hz")]
    B1850 = 0xFD,
    /// 1900.0 mV/Hz
    #[strum(serialize = "1900.0 mV/Hz")]
    B1900 = 0xFE,
    /// 2000.0 mV/Hz
    #[strum(serialize = "2000.0 mV/Hz")]
    B2000 = 0xFF,
}

impl From<u8> for MotorBemf {
    fn from(value: u8) -> Self {
        // Safety: MotorBemf has a valid representation for all u8 values
        unsafe { core::mem::transmute(value) }
    }
}
