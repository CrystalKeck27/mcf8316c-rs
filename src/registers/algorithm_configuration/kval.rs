use arbitrary_int::*;
use bitbybit::*;

/// 10-bit value for PI loop. K_x = Multiplier (constant per field) * value / 10^scale
#[bitfield(u10)]
#[derive(Debug, PartialEq, Eq)]
pub struct KVal {
    #[bits(8..=9)]
    scale: u2, // 2 bits for scale (0-3)
    #[bits(0..=7)]
    value: u8, // 8 bits for value (0-255)
}

pub trait KValMultiplier {
    const SCALE_SHIFT: i8;
}

impl KVal {
    pub const AUTO: Self = KVal::new_with_raw_value(u10::new(0));
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CurrentKp;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CurrentKi;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SpeedKp;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SpeedKi;

impl KValMultiplier for CurrentKp {
    const SCALE_SHIFT: i8 = 0;
}

impl KValMultiplier for CurrentKi {
    const SCALE_SHIFT: i8 = -3;
}

impl KValMultiplier for SpeedKp {
    const SCALE_SHIFT: i8 = 2;
}

impl KValMultiplier for SpeedKi {
    const SCALE_SHIFT: i8 = 1;
}

#[bitfield(u3)]
#[derive(Debug, PartialEq, Eq)]
pub struct SpeedLoopKpHigh3 {
    #[bits(0..=2, rw)]
    inner: u3,
}

#[bitfield(u7)]
#[derive(Debug, PartialEq, Eq)]
pub struct SpeedLoopKpLow7 {
    #[bits(0..=6, rw)]
    inner: u7,
}

impl KVal {
    pub fn from_high_low(high: SpeedLoopKpHigh3, low: SpeedLoopKpLow7) -> Self {
        KVal::new_with_raw_value(u10::extract_u16(
            (u16::from(high.inner()) << 7) | u16::from(low.inner()),
            0,
        ))
    }
}
