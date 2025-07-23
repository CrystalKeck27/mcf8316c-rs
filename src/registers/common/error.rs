/// Error thrown when a value has more bits than expected.
/// 
/// If this error is thrown, it must be because the value has more bits than the maximum allowed.
/// Not because the value has the correct number of bits but is not valid.
/// 
/// For example, if a value is valid in the range of 0-6, and the number passed is 9, that is a BitCountError.
/// If the value is 7, that is not a BitCountError, but something else.
/// 
/// If a function returns only a BitCountError, it is expected that any value with the correct number of bits is valid.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BitCountError {
    pub max_bits: u8,
    pub received_bits: u8,
}

impl BitCountError {
    pub fn new(max_bits: u8, received_bits: u8) -> Self {
        BitCountError {
            max_bits,
            received_bits,
        }
    }
}

impl core::fmt::Display for BitCountError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "BitCountError: expected at most {} bits, received {} bits",
            self.max_bits, self.received_bits
        )
    }
}

impl core::error::Error for BitCountError {}
