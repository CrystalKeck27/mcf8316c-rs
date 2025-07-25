//! Kx values for PI loops

use arbitrary_int::*;
use bitbybit::*;

/// Kp for the current loop.
pub type CurrentKpVal = KVal<CurrentKp>;
/// Ki for the current loop.
pub type CurrentKiVal = KVal<CurrentKi>;
/// Kp for the speed loop.
pub type SpeedKpVal = KVal<SpeedKp>;
/// Ki for the speed loop.
pub type SpeedKiVal = KVal<SpeedKi>;

/// 10-bit value for PI loop. K_x = Multiplier (constant per field) * value / 10^scale
#[derive(Debug, PartialEq, Eq)]
pub struct KVal<T: KValMultiplier> {
    inner: u10,
    _marker: core::marker::PhantomData<T>,
}

/// Trait to provide the predefined multiplier for each KVal type.
pub trait KValMultiplier {
    /// The scale shift applied to the value.
    const SCALE_SHIFT: i8;
}

impl<T: KValMultiplier> KVal<T> {
    /// Processor automatically determines the KVal
    pub const AUTO: Self = KVal::<T>::new_with_raw_value(u10::new(0));

    /// Create a new KVal from the given raw value.
    pub const fn new_with_raw_value(raw_value: u10) -> Self {
        KVal {
            inner: raw_value,
            _marker: core::marker::PhantomData,
        }
    }

    /// Returns the raw value of the KVal.
    pub const fn raw_value(&self) -> u10 {
        self.inner
    }

    /// Retuns the scaling factor bits
    pub const fn scale(&self) -> u2 {
        u2::extract_u16(self.inner.value(), 8)
    }

    /// Sets the scaling factor bits.
    pub const fn set_scale(&mut self, scale: u2) {
        self.inner = u10::new((self.inner.value() & 0xFF) | (scale.value() as u16) << 8);
    }

    /// Returns the value bits
    pub const fn value(&self) -> u8 {
        (self.inner.value() & 0xFF) as u8
    }

    /// Sets the value bits.
    pub const fn set_value(&mut self, value: u8) {
        self.inner = u10::new((self.inner.value() & 0x0300) | (value as u16));
    }

    /// Returns the value represented by the internal bits.
    pub fn calculated_value(&self) -> f32 {
        let value = self.value() as f32;
        let scale = 10f32.powi(self.scale().value() as i32 + T::SCALE_SHIFT as i32);
        value / scale
    }
}

/// Unit struct for the constant multiplier of the Kp for the current loop.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CurrentKp;
/// Unit struct for the constant multiplier of the Ki for the current loop.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CurrentKi;
/// Unit struct for the constant multiplier of the Kp for the speed loop.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SpeedKp;
/// Unit struct for the constant multiplier of the Ki for the speed loop.
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

/// 3 MSB for speed loop Kp.
#[bitfield(u3)]
#[derive(Debug, PartialEq, Eq)]
pub struct SpeedLoopKpHigh3 {
    /// Inner value
    #[bits(0..=2, rw)]
    inner: u3,
}

/// 7 LSB for speed loop Kp.
#[bitfield(u7)]
#[derive(Debug, PartialEq, Eq)]
pub struct SpeedLoopKpLow7 {
    /// Inner value
    #[bits(0..=6, rw)]
    inner: u7,
}

impl KVal<SpeedKp> {
    /// Create a new KVal from the high and low parts of the speed loop Kp.
    pub fn from_high_low(high: SpeedLoopKpHigh3, low: SpeedLoopKpLow7) -> Self {
        KVal::<SpeedKp>::new_with_raw_value(u10::extract_u16(
            (u16::from(high.inner()) << 7) | u16::from(low.inner()),
            0,
        ))
    }
}
