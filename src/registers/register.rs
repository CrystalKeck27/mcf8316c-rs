use arbitrary_int::u12;

/// Trait implemented by all register structs and used by the driver to ease reading and writing to registers.
pub trait Register {
    /// 12-bit address of the register.
    const ADDRESS: u12;

    /// Returns the value to be sent on the i2c bus.
    fn value(&self) -> u32;

    /// Creates a new instance of the register with the given value.
    fn from_value(value: u32) -> Self;
}

/// Trait implemented by all register structs that can be used flexibly, allowing for dynamic address and value.
pub trait FlexibleRegister {
    /// 12-bit address of the register.
    fn address(&self) -> u12;

    /// Returns the value to be sent on the i2c bus.
    fn value(&self) -> u32;

    /// Creates a new instance of the register with the given address and value.
    fn compare_internal(&self, value: u32) -> bool;
}

impl<T> FlexibleRegister for T
where
    T: Register + PartialEq,
{
    fn address(&self) -> u12 {
        T::ADDRESS
    }

    fn value(&self) -> u32 {
        self.value()
    }

    fn compare_internal(&self, value: u32) -> bool {
        T::from_value(value) == *self
    }
}
