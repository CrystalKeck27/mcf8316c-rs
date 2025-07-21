
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct PercentAsU8 {
    pub inner: u8,
}

impl PercentAsU8 {
    pub fn new(value: u8) -> Self {
        PercentAsU8 { inner: value }
    }
}

impl From<PercentAsU8> for f32 {
    fn from(percent: PercentAsU8) -> Self {
        percent.inner as f32 / 2.55
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DutyAHigh5(pub u8);
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DutyALow3(pub u8);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DutyEHigh4(pub u8);
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DutyELow4(pub u8);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RefBHigh7(pub u8);
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RefBLow1(pub u8);