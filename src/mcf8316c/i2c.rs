use arbitrary_int::*;

/// Alias for the CRC algorithm used by the MCF8316C-Q1.
/// The library has the correct name, but the datasheet for the MCF8316C-Q1
/// refers to it as CCIT. See section 7.6.2.6 of the MCF8316C-Q1 datasheet.
pub const CRC_8_CCIT: crc::Algorithm<u8> = crc::CRC_8_I_432_1;

// 24-bit control word format
// | OP_R/W | CRC_EN |   DLEN    |  MEM_SEC  | MEM_PAGE  | MEM_ADDR |
// |--------|--------|-----------|-----------|-----------|----------|
// |  CW23  |  CW22  | CW21-CW20 | CW19-CW16 | CW15-CW12 | CW11-CW0 |

/// Represents a control word for the MCF8316C-Q1 I2C communication.
/// The word is either followed by a read operation or a data word and crc.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ControlWord {
    /// # Read/Write
    /// R/W bit gives information on whether this is a read (1b) operation or write (0b)
    /// operation. For write operation, MCF8316C-Q1 will expect data bytes to be sent after the 24-bit control word.
    /// For read operation, MCF8316C-Q1 will expect an I2C read request with repeated start or normal start after the
    /// 24-bit control word.
    pub op_rw: bool,
    /// # Cyclic Redundancy Check(CRC) Enable
    /// MCF8316C-Q1 supports CRC to verify the data integrity.
    /// This bit controls whether the CRC feature is enabled or not.
    /// 
    /// ## WARNING
    /// MCF8316C-Q1 will compute CRC using the same polynomial internally and if there is a mismatch, the write request is **discarded**.
    /// **No Error is returned**.
    pub crc_en: bool,
    /// # Data Length
    /// DLEN field determines the length of the data that will be sent by external MCU to
    /// MCF8316C-Q1. MCF8316C-Q1 protocol supports three data lengths: 16-bit, 32-bit and 64-bit.
    pub dlen: DataLength,
    /// # Memory Section
    /// Each memory location in MCF8316C-Q1 is addressed using three separate
    /// entities in the control word – Memory Section, Memory Page, Memory Address. Memory Section is a 4-bit field
    /// which denotes the memory section to which the memory location belongs like RAM, ROM etc.
    pub mem_sec: u4,
    /// # Memory Page
    /// Memory page is a 4-bit field which denotes the memory page to which the
    /// memory location belongs.
    pub mem_page: u4,
    /// # Memory Address
    /// Memory address is the last 12-bits of the address. The complete 22-bit
    /// address is constructed internally by MCF8316C-Q1 using all three fields – Memory Section, Memory Page,
    /// Memory Address. For memory locations 0x000000-0x000800, memory section is 0x0, memory page is 0x0
    /// and memory address is the lowest 12 bits(0x000 for 0x000000, 0x080 for 0x000080 and 0x800 for 0x000800).
    /// All relevant memory locations (EEPROM and RAM variables) have MEM_SEC and MEM_PAGE values both
    /// corresponding to 0x0. All other MEM_SEC, MEM_PAGE values are reserved and not for external use.
    pub mem_addr: u12,
}

impl ControlWord {
    /// Creates a new control word with the specified parameters.
    pub fn new(is_read: bool, crc_en: bool, dlen: DataLength, mem_addr: u16) -> Self {
        ControlWord {
            op_rw: is_read,
            crc_en,
            dlen,
            mem_sec: u4::new(255),
            mem_page: u4::new(0),
            mem_addr: u12::new(mem_addr),
        }
    }

    /// Converts the control word into a 24-bit array.
    pub fn to_bytes(&self) -> [u8; 3] {
        let mut bytes = [0; 3];
        self.to_bytes_in_place(&mut bytes);
        bytes
    }

    /// Converts the control word into a 24-bit array and writes it to the provided byte slice.
    ///
    /// # Panics
    /// Panics if the provided byte slice is less than 3 bytes long.
    pub fn to_bytes_in_place(&self, bytes: &mut [u8]) {
        bytes[0] = ((self.op_rw as u8) << 7)
            | ((self.crc_en as u8) << 6)
            | ((self.dlen as u8) << 4)
            | (self.mem_sec.as_u8() & 0x0F);
        bytes[1] = ((self.mem_page.as_u8() & 0x0F) << 4) | ((self.mem_addr.as_u16() >> 8) & 0x0F) as u8;
        bytes[2] = (self.mem_addr.as_u16() & 0xFF) as u8;
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum DataLength {
    Len16 = DLEN_16,
    Len32 = DLEN_32,
    Len64 = DLEN_64,
}

pub const DLEN_16: u8 = 0b00;
pub const DLEN_32: u8 = 0b01;
pub const DLEN_64: u8 = 0b10;
pub const DLEN_RESERVED: u8 = 0b11;

pub struct DataAndCrc {
    pub data: Vec<u8>,
    pub crc: u8,
}
