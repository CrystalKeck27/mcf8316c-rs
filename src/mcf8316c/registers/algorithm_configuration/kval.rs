
/// 10-bit value for PI loop. K_x = Multiplier (constant per field) * value / 10^scale
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct KVal<T: KValMultiplier> {
    inner: [u8; 2],
    _marker: core::marker::PhantomData<T>,
}

pub trait KValMultiplier {
    const SCALE_SHIFT: i8;
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

impl<T: KValMultiplier> KVal<T> {
    /// Creates a new KVal.
    ///
    /// # Panics
    /// Panics if scale uses more than 2 bits (0-3).
    pub fn new(scale: u8, value: u8) -> Self {
        assert!(scale <= 0x3, "Scale must be between 0 and 3 (0x3)");
        let mut inner = [0; 2];
        inner[0] = value;
        inner[1] = scale;
        KVal {
            inner,
            _marker: core::marker::PhantomData,
        }
    }

    pub fn scale_bits(&self) -> u8 {
        self.inner[1]
    }

    pub fn value_bits(&self) -> u8 {
        self.inner[0]
    }

    /// Returns the value as a floating-point number, or None if set to auto.
    pub fn value(&self) -> Option<f32> {
        if self.value_bits() == 0 && self.scale_bits() == 0 {
            return None;
        }
        let scale = self.scale_bits() as i32 + T::SCALE_SHIFT as i32;
        let divisor = 10f32.powi(scale);

        Some((self.value_bits() as f32) / divisor)
    }

    #[allow(dead_code)]
    const AUTO: Self = KVal {
        inner: [0, 0],
        _marker: core::marker::PhantomData,
    };
}

impl<T: KValMultiplier> From<KVal<T>> for u16 {
    fn from(kval: KVal<T>) -> Self {
        u16::from_le_bytes(kval.inner)
    }
}

impl<T: KValMultiplier> TryFrom<u16> for KVal<T> {
    type Error = &'static str;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        if value > 0x3FF {
            return Err("Value must be between 0 and 1023 (0x3FF)");
        }
        let scale = (value >> 10) as u8;
        let val = (value & 0x3FF) as u8;
        Ok(KVal::new(scale, val))
    }
}

impl<T: KValMultiplier + PartialEq> PartialOrd for KVal<T> {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        match (self.value(), other.value()) {
            (Some(a), Some(b)) => a.partial_cmp(&b),
            _ => None, // Auto cannot be compared to anything
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SpeedLoopKpHigh3(pub u8);
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SpeedLoopKpLow7(pub u8);

impl KVal<SpeedKp> {
    pub fn from_high_low(high: SpeedLoopKpHigh3, low: SpeedLoopKpLow7) -> Self {
        let x: u16 = ((u16::from(high.0) & 0x07) << 7) | (u16::from(low.0) & 0x7F);
        // Safety: The value is guaranteed to be in the range 0-1023 (0x3FF)
        unsafe { x.try_into().unwrap_unchecked() }
    }
}
