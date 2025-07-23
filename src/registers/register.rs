pub trait Register {
    const ADDRESS: u16;

    fn value(&self) -> u32;
}
