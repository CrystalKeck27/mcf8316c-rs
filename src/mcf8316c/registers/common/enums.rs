#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, strum::Display)]
#[repr(u8)]
pub enum CurrentSelection {
    /// 0.125A
    #[strum(to_string = "0.125 A")]
    A0_125 = 0x0,
    /// 0.25A
    #[strum(to_string = "0.25 A")]
    A0_25 = 0x1,
    /// 0.5A
    #[strum(to_string = "0.5 A")]
    A0_5 = 0x2,
    /// 1.0A
    #[strum(to_string = "1.0 A")]
    A1_0 = 0x3,
    /// 1.5A
    #[strum(to_string = "1.5 A")]
    A1_5 = 0x4,
    /// 2.0A
    #[strum(to_string = "2.0 A")]
    A2_0 = 0x5,
    /// 2.5A
    #[strum(to_string = "2.5 A")]
    A2_5 = 0x6,
    /// 3.0A
    #[strum(to_string = "3.0 A")]
    A3_0 = 0x7,
    /// 3.5A
    #[strum(to_string = "3.5 A")]
    A3_5 = 0x8,
    /// 4.0A
    #[strum(to_string = "4.0 A")]
    A4_0 = 0x9,
    /// 4.5A
    #[strum(to_string = "4.5 A")]
    A4_5 = 0xA,
    /// 5.0A
    #[strum(to_string = "5.0 A")]
    A5_0 = 0xB,
    /// 5.5A
    #[strum(to_string = "5.5 A")]
    A5_5 = 0xC,
    /// 6.0A
    #[strum(to_string = "6.0 A")]
    A6_0 = 0xD,
    /// 7.0A
    #[strum(to_string = "7.0 A")]
    A7_0 = 0xE,
    /// 8.0A
    #[strum(to_string = "8.0 A")]
    A8_0 = 0xF,
}

impl From<u8> for CurrentSelection {
    fn from(value: u8) -> Self {
        match value {
            0x0 => CurrentSelection::A0_125,
            0x1 => CurrentSelection::A0_25,
            0x2 => CurrentSelection::A0_5,
            0x3 => CurrentSelection::A1_0,
            0x4 => CurrentSelection::A1_5,
            0x5 => CurrentSelection::A2_0,
            0x6 => CurrentSelection::A2_5,
            0x7 => CurrentSelection::A3_0,
            0x8 => CurrentSelection::A3_5,
            0x9 => CurrentSelection::A4_0,
            0xA => CurrentSelection::A4_5,
            0xB => CurrentSelection::A5_0,
            0xC => CurrentSelection::A5_5,
            0xD => CurrentSelection::A6_0,
            0xE => CurrentSelection::A7_0,
            0xF => CurrentSelection::A8_0,
            _ => panic!("Invalid value for CurrentSelection: {}", value),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, strum::Display)]
#[repr(u8)]
pub enum OpenLoopAccelerationA1 {
    /// 0.01 Hz/s
    #[strum(to_string = "0.01 Hz/s")]
    A0_01 = 0x0,
    /// 0.05 Hz/s
    #[strum(to_string = "0.05 Hz/s")]
    A0_05 = 0x1,
    /// 1.0 Hz/s
    #[strum(to_string = "1.0 Hz/s")]
    A1_0 = 0x2,
    /// 2.5 Hz/s
    #[strum(to_string = "2.5 Hz/s")]
    A2_5 = 0x3,
    /// 5.0 Hz/s
    #[strum(to_string = "5.0 Hz/s")]
    A5_0 = 0x4,
    /// 10 Hz/s
    #[strum(to_string = "10 Hz/s")]
    A10 = 0x5,
    /// 25 Hz/s
    #[strum(to_string = "25 Hz/s")]
    A25 = 0x6,
    /// 50 Hz/s
    #[strum(to_string = "50 Hz/s")]
    A50 = 0x7,
    /// 75 Hz/s
    #[strum(to_string = "75 Hz/s")]
    A75 = 0x8,
    /// 100 Hz/s
    #[strum(to_string = "100 Hz/s")]
    A100 = 0x9,
    /// 250 Hz/s
    #[strum(to_string = "250 Hz/s")]
    A250 = 0xA,
    /// 500 Hz/s
    #[strum(to_string = "500 Hz/s")]
    A500 = 0xB,
    /// 750 Hz/s
    #[strum(to_string = "750 Hz/s")]
    A750 = 0xC,
    /// 1000 Hz/s
    #[strum(to_string = "1000 Hz/s")]
    A1000 = 0xD,
    /// 5000 Hz/s
    #[strum(to_string = "5000 Hz/s")]
    A5000 = 0xE,
    /// 10000 Hz/s
    #[strum(to_string = "10000 Hz/s")]
    A10000 = 0xF,
}

impl From<u8> for OpenLoopAccelerationA1 {
    fn from(value: u8) -> Self {
        match value {
            0x0 => OpenLoopAccelerationA1::A0_01,
            0x1 => OpenLoopAccelerationA1::A0_05,
            0x2 => OpenLoopAccelerationA1::A1_0,
            0x3 => OpenLoopAccelerationA1::A2_5,
            0x4 => OpenLoopAccelerationA1::A5_0,
            0x5 => OpenLoopAccelerationA1::A10,
            0x6 => OpenLoopAccelerationA1::A25,
            0x7 => OpenLoopAccelerationA1::A50,
            0x8 => OpenLoopAccelerationA1::A75,
            0x9 => OpenLoopAccelerationA1::A100,
            0xA => OpenLoopAccelerationA1::A250,
            0xB => OpenLoopAccelerationA1::A500,
            0xC => OpenLoopAccelerationA1::A750,
            0xD => OpenLoopAccelerationA1::A1000,
            0xE => OpenLoopAccelerationA1::A5000,
            0xF => OpenLoopAccelerationA1::A10000,
            _ => panic!("Invalid value for OpenLoopAccelerationA1: {}", value),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, strum::Display)]
#[repr(u8)]
pub enum OpenLoopAccelerationA2 {
    /// 0.01 Hz/s²
    #[strum(to_string = "0.00 Hz/s²")]
    A0 = 0x0,
    /// 0.05 Hz/s²
    #[strum(to_string = "0.05 Hz/s²")]
    A0_05 = 0x1,
    /// 1.0 Hz/s²
    #[strum(to_string = "1.0 Hz/s²")]
    A1_0 = 0x2,
    /// 2.5 Hz/s²
    #[strum(to_string = "2.5 Hz/s²")]
    A2_5 = 0x3,
    /// 5.0 Hz/s²
    #[strum(to_string = "5.0 Hz/s²")]
    A5_0 = 0x4,
    /// 10 Hz/s²
    #[strum(to_string = "10 Hz/s²")]
    A10 = 0x5,
    /// 25 Hz/s²
    #[strum(to_string = "25 Hz/s²")]
    A25 = 0x6,
    /// 50 Hz/s²
    #[strum(to_string = "50 Hz/s²")]
    A50 = 0x7,
    /// 75 Hz/s²
    #[strum(to_string = "75 Hz/s²")]
    A75 = 0x8,
    /// 100 Hz/s²
    #[strum(to_string = "100 Hz/s²")]
    A100 = 0x9,
    /// 250 Hz/s²
    #[strum(to_string = "250 Hz/s²")]
    A250 = 0xA,
    /// 500 Hz/s²
    #[strum(to_string = "500 Hz/s²")]
    A500 = 0xB,
    /// 750 Hz/s²
    #[strum(to_string = "750 Hz/s²")]
    A750 = 0xC,
    /// 1000 Hz/s²
    #[strum(to_string = "1000 Hz/s²")]
    A1000 = 0xD,
    /// 5000 Hz/s²
    #[strum(to_string = "5000 Hz/s²")]
    A5000 = 0xE,
    /// 10000 Hz/s²
    #[strum(to_string = "10000 Hz/s²")]
    A10000 = 0xF,
}

impl From<u8> for OpenLoopAccelerationA2 {
    fn from(value: u8) -> Self {
        match value {
            0x0 => OpenLoopAccelerationA2::A0,
            0x1 => OpenLoopAccelerationA2::A0_05,
            0x2 => OpenLoopAccelerationA2::A1_0,
            0x3 => OpenLoopAccelerationA2::A2_5,
            0x4 => OpenLoopAccelerationA2::A5_0,
            0x5 => OpenLoopAccelerationA2::A10,
            0x6 => OpenLoopAccelerationA2::A25,
            0x7 => OpenLoopAccelerationA2::A50,
            0x8 => OpenLoopAccelerationA2::A75,
            0x9 => OpenLoopAccelerationA2::A100,
            0xA => OpenLoopAccelerationA2::A250,
            0xB => OpenLoopAccelerationA2::A500,
            0xC => OpenLoopAccelerationA2::A750,
            0xD => OpenLoopAccelerationA2::A1000,
            0xE => OpenLoopAccelerationA2::A5000,
            0xF => OpenLoopAccelerationA2::A10000,
            _ => panic!("Invalid value for OpenLoopAccelerationA2: {}", value),
        }
    }
}