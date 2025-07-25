use arbitrary_int::u12;

/// Trait implemented by all register structs and used by the driver to ease reading and writing to registers.
pub trait Register {
    /// 12-bit address of the register.
    const ADDRESS: u12;

    /// Returns the value to be sent on the i2c bus.
    fn value(&self) -> u32;
}
