use arbitrary_int::*;
use bitbybit::bitfield;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct PercentAsU8 {
    pub inner: u8,
}

impl PercentAsU8 {
    pub const fn new(value: u8) -> Self {
        PercentAsU8 { inner: value }
    }

    pub const fn new_with_raw_value(value: u8) -> Self {
        PercentAsU8 { inner: value }
    }

    pub const fn raw_value(&self) -> u8 {
        self.inner
    }
}

impl From<PercentAsU8> for f32 {
    fn from(percent: PercentAsU8) -> Self {
        percent.inner as f32 / 2.55
    }
}

#[bitfield(u5)]
#[derive(Debug, PartialEq, Eq)]
pub struct DutyAHigh5 {
    #[bits(0..=4, rw)]
    inner: u5,
}

#[bitfield(u3)]
#[derive(Debug, PartialEq, Eq)]
pub struct DutyALow3 {
    #[bits(0..=2, rw)]
    inner: u3,
}

impl PercentAsU8 {
    pub fn combine_a(high: DutyAHigh5, low: DutyALow3) -> Self {
        PercentAsU8::new_with_raw_value(
            (u8::from(high.inner()) << 3) | u8::from(low.inner()),
        )
    }

    pub fn split_a(&self) -> (DutyAHigh5, DutyALow3) {
        let high = DutyAHigh5::new_with_raw_value(u5::extract_u8(self.inner, 3));
        let low = DutyALow3::new_with_raw_value(u3::extract_u8(self.inner, 0));
        (high, low)
    }
}

#[bitfield(u4)]
#[derive(Debug, PartialEq, Eq)]
pub struct DutyEHigh4 {
    #[bits(0..=3, rw)]
    inner: u4,
}

#[bitfield(u4)]
#[derive(Debug, PartialEq, Eq)]
pub struct DutyELow4 {
    #[bits(0..=3, rw)]
    inner: u4,
}

impl PercentAsU8 {
    pub fn combine_e(high: DutyEHigh4, low: DutyELow4) -> Self {
        PercentAsU8::new_with_raw_value(
            (u8::from(high.inner()) << 4) | u8::from(low.inner()),
        )
    }

    pub fn split_e(&self) -> (DutyEHigh4, DutyELow4) {
        let high = DutyEHigh4::new_with_raw_value(u4::extract_u8(self.inner, 4));
        let low = DutyELow4::new_with_raw_value(u4::extract_u8(self.inner, 0));
        (high, low)
    }
}

#[bitfield(u7)]
#[derive(Debug, PartialEq, Eq)]
pub struct RefBHigh7 {
    #[bits(0..=6, rw)]
    inner: u7,
}

#[bitfield(u1)]
#[derive(Debug, PartialEq, Eq)]
pub struct RefBLow1 {
    #[bits(0..=0, rw)]
    inner: u1,
}

impl PercentAsU8 {
    pub fn combine_b(high: RefBHigh7, low: RefBLow1) -> Self {
        PercentAsU8::new_with_raw_value(
            (u8::from(high.inner()) << 1) | u8::from(low.inner()),
        )
    }

    pub fn split_b(&self) -> (RefBHigh7, RefBLow1) {
        let high = RefBHigh7::new_with_raw_value(u7::extract_u8(self.inner, 1));
        let low = RefBLow1::new_with_raw_value(u1::extract_u8(self.inner, 0));
        (high, low)
    }
}
