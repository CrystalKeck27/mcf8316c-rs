use arbitrary_int::*;
use bitbybit::bitfield;

/// Represents a percentage value as an 8-bit unsigned integer.
/// The value is allowed to be in the full range of 0-255,
/// where 0 represents 0% and 255 represents 100%.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct PercentAsU8 {
    /// The inner value representing the percentage.
    pub inner: u8,
}

impl PercentAsU8 {
    /// Creates a new `PercentAsU8` instance with the given value.
    pub const fn new(value: u8) -> Self {
        PercentAsU8 { inner: value }
    }

    /// Creates a new `PercentAsU8` instance with the given value.
    pub const fn new_with_raw_value(value: u8) -> Self {
        PercentAsU8 { inner: value }
    }

    /// Returns the raw value as an 8-bit unsigned integer.
    pub const fn raw_value(&self) -> u8 {
        self.inner
    }
}

impl From<PercentAsU8> for f32 {
    fn from(percent: PercentAsU8) -> Self {
        percent.inner as f32 / 2.55
    }
}

/// Most significant 5 bits of the duty cycle A value.
#[bitfield(u5)]
#[derive(Debug, PartialEq, Eq)]
pub struct DutyAHigh5 {
    /// Inner value
    #[bits(0..=4, rw)]
    inner: u5,
}

/// Least significant 3 bits of the duty cycle A value.
#[bitfield(u3)]
#[derive(Debug, PartialEq, Eq)]
pub struct DutyALow3 {
    /// Inner value
    #[bits(0..=2, rw)]
    inner: u3,
}

impl PercentAsU8 {
    /// Combines the high and low parts of the duty cycle A into a single `PercentAsU8`.
    pub fn combine_duty_a(high: DutyAHigh5, low: DutyALow3) -> Self {
        PercentAsU8::new_with_raw_value((u8::from(high.inner()) << 3) | u8::from(low.inner()))
    }

    /// Splits the `PercentAsU8` into its high and low parts for duty cycle A.
    pub fn split_duty_a(&self) -> (DutyAHigh5, DutyALow3) {
        let high = DutyAHigh5::new_with_raw_value(u5::extract_u8(self.inner, 3));
        let low = DutyALow3::new_with_raw_value(u3::extract_u8(self.inner, 0));
        (high, low)
    }
}

/// Most significant 4 bits of the duty cycle E value.
#[bitfield(u4)]
#[derive(Debug, PartialEq, Eq)]
pub struct DutyEHigh4 {
    /// Inner value
    #[bits(0..=3, rw)]
    inner: u4,
}

/// Least significant 4 bits of the duty cycle E value.
#[bitfield(u4)]
#[derive(Debug, PartialEq, Eq)]
pub struct DutyELow4 {
    /// Inner value
    #[bits(0..=3, rw)]
    inner: u4,
}

impl PercentAsU8 {
    /// Combines the high and low parts of the duty cycle E into a single `PercentAsU8`.
    pub fn combine_duty_e(high: DutyEHigh4, low: DutyELow4) -> Self {
        PercentAsU8::new_with_raw_value((u8::from(high.inner()) << 4) | u8::from(low.inner()))
    }

    /// Splits the `PercentAsU8` into its high and low parts for duty cycle E.
    pub fn split_duty_e(&self) -> (DutyEHigh4, DutyELow4) {
        let high = DutyEHigh4::new_with_raw_value(u4::extract_u8(self.inner, 4));
        let low = DutyELow4::new_with_raw_value(u4::extract_u8(self.inner, 0));
        (high, low)
    }
}

/// Most significant 7 bits of the reference B value.
#[bitfield(u7)]
#[derive(Debug, PartialEq, Eq)]
pub struct RefBHigh7 {
    /// Inner value
    #[bits(0..=6, rw)]
    inner: u7,
}

/// Least significant 1 bit of the reference B value.
#[bitfield(u1)]
#[derive(Debug, PartialEq, Eq)]
pub struct RefBLow1 {
    /// Inner value
    #[bits(0..=0, rw)]
    inner: u1,
}

impl PercentAsU8 {
    /// Combines the high and low parts of the reference B into a single `PercentAsU8`.
    pub fn combine_ref_b(high: RefBHigh7, low: RefBLow1) -> Self {
        PercentAsU8::new_with_raw_value((u8::from(high.inner()) << 1) | u8::from(low.inner()))
    }

    /// Splits the `PercentAsU8` into its high and low parts for reference B.
    pub fn split_ref_b(&self) -> (RefBHigh7, RefBLow1) {
        let high = RefBHigh7::new_with_raw_value(u7::extract_u8(self.inner, 1));
        let low = RefBLow1::new_with_raw_value(u1::extract_u8(self.inner, 0));
        (high, low)
    }
}
