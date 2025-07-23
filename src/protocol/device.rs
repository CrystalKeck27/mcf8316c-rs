use embedded_hal::i2c::SevenBitAddress;

use crate::protocol::control_word::{DataLength, CRC_8_CCIT};

use super::{super::registers::register::Register, control_word};

pub struct MCF8316C<I2C: embedded_hal::i2c::I2c<SevenBitAddress>> {
    pub i2c: I2C,
    pub address: SevenBitAddress,
}

impl<I2C: embedded_hal::i2c::I2c<SevenBitAddress>> MCF8316C<I2C> {
    /// Creates a new instance of the MCF8316C driver.
    pub fn new(i2c: I2C) -> Self {
        MCF8316C {
            i2c,
            address: SevenBitAddress::from(0x00),
        }
    }

    pub fn with_i2c_address(i2c: I2C, address: u8) -> Self {
        MCF8316C {
            i2c,
            address: SevenBitAddress::from(address),
        }
    }

    /// Creates a packet that would set the data at the specified address.
    pub fn create_write_u16_packet(&mut self, address: u16, data: u16) -> [u8; 6] {
        let control_word = control_word::ControlWord::new(
            false,             // Write operation
            true,              // CRC enabled
            DataLength::Len16, // Data length
            address,
        );
        let mut packet = [0u8; 6];
        control_word.to_bytes_in_place(&mut packet[..3]);
        packet[3..5].copy_from_slice(&data.to_le_bytes());

        let first_byte = self.address << 1;

        let crc = crc::Crc::<u8>::new(&CRC_8_CCIT);
        let mut digest = crc.digest();
        digest.update(&[first_byte]);
        digest.update(&packet[..5]);
        packet[5] = digest.finalize();

        packet
    }

    /// Creates a packet that would set the data at the specified address.
    pub fn create_write_u32_packet(&mut self, address: u16, data: u32) -> [u8; 8] {
        let control_word = control_word::ControlWord::new(
            false,             // Write operation
            true,              // CRC enabled
            DataLength::Len32, // Data length
            address,
        );
        let mut packet = [0u8; 8];
        control_word.to_bytes_in_place(&mut packet[..3]);
        packet[3..7].copy_from_slice(&data.to_le_bytes());

        let first_byte = self.address << 1;

        let crc = crc::Crc::<u8>::new(&CRC_8_CCIT);
        let mut digest = crc.digest();
        digest.update(&[first_byte]);
        digest.update(&packet[..7]);
        packet[7] = digest.finalize();

        packet
    }

    /// Creates a packet that would set the data at the specified address.
    pub fn create_write_u64_packet(&mut self, address: u16, data: u64) -> [u8; 12] {
        let control_word = control_word::ControlWord::new(
            false,             // Write operation
            true,              // CRC enabled
            DataLength::Len64, // Data length
            address,
        );
        let mut packet = [0u8; 12];
        control_word.to_bytes_in_place(&mut packet[..3]);
        packet[3..11].copy_from_slice(&data.to_le_bytes());

        let first_byte = self.address << 1;

        let crc = crc::Crc::<u8>::new(&CRC_8_CCIT);
        let mut digest = crc.digest();
        digest.update(&[first_byte]);
        digest.update(&packet[..9]);
        packet[11] = digest.finalize();

        packet
    }

    /// Writes data to the specified address.
    pub fn write_u16(&mut self, address: u16, data: u16) -> Result<(), I2C::Error> {
        let packet = self.create_write_u16_packet(address, data);
        self.i2c.write(self.address, &packet)
    }

    /// Writes data to the specified address.
    pub fn write_u32(&mut self, address: u16, data: u32) -> Result<(), I2C::Error> {
        let packet = self.create_write_u32_packet(address, data);
        self.i2c.write(self.address, &packet)
    }

    /// Writes data to the specified address.
    pub fn write_u64(&mut self, address: u16, data: u64) -> Result<(), I2C::Error> {
        let packet = self.create_write_u64_packet(address, data);
        self.i2c.write(self.address, &packet)
    }

    /// Writes data to the specified register.
    pub fn write<T: Register + Into<u32>>(&mut self, data: T) -> Result<(), I2C::Error> {
        self.write_u32(T::ADDRESS, data.into())
    }

    /// Reads data from the specified address.
    pub fn read_u16(&mut self, address: u16) -> Result<u16, I2C::Error> {
        let control_word = control_word::ControlWord::new(
            true,              // Read operation
            true,              // CRC enabled
            DataLength::Len16, // Data length
            address,
        );
        let control_word = control_word.to_bytes();
        let mut data = [0u8; 3];
        // self.i2c.write_read(self.address, &packet, &mut data)?;
        self.i2c.write(self.address, &control_word)?;
        std::thread::sleep(core::time::Duration::from_millis(10)); // Wait for the device to respond (only needed in development)
        self.i2c.read(self.address, &mut data)?;

        let first_byte_tx = self.address << 1;
        let first_byte_rx = first_byte_tx | 1;

        let crc = crc::Crc::<u8>::new(&CRC_8_CCIT);
        let mut digest = crc.digest();
        digest.update(&[first_byte_tx]);
        digest.update(&control_word);
        digest.update(&[first_byte_rx]);
        digest.update(&data[..2]);
        if digest.finalize() != data[2] {
            // return Err(embedded_hal::i2c::Error::Other("CRC mismatch"));
        }

        // Safety: We always have enough data to fill the u32
        Ok(u16::from_le_bytes(unsafe {
            data[0..2].try_into().unwrap_unchecked()
        }))
    }

    /// Reads data from the specified address.
    pub fn read_u32(&mut self, address: u16) -> Result<u32, I2C::Error> {
        let control_word = control_word::ControlWord::new(
            true,              // Read operation
            true,              // CRC enabled
            DataLength::Len32, // Data length
            address,
        );
        let control_word = control_word.to_bytes();
        let mut data = [0u8; 5];
        // self.i2c.write_read(self.address, &packet, &mut data)?;
        self.i2c.write(self.address, &control_word)?;
        std::thread::sleep(core::time::Duration::from_millis(10)); // Wait for the device to respond (only needed in development)
        self.i2c.read(self.address, &mut data)?;

        let first_byte_tx = self.address << 1;
        let first_byte_rx = first_byte_tx | 1;

        let crc = crc::Crc::<u8>::new(&CRC_8_CCIT);
        let mut digest = crc.digest();
        digest.update(&[first_byte_tx]);
        digest.update(&control_word);
        digest.update(&[first_byte_rx]);
        digest.update(&data[..4]);
        if digest.finalize() != data[4] {
            // return Err(embedded_hal::i2c::Error::Other("CRC mismatch"));
        }

        // Safety: We always have enough data to fill the u32
        Ok(u32::from_le_bytes(unsafe {
            data[0..4].try_into().unwrap_unchecked()
        }))
    }

    /// Reads data from the specified address.
    pub fn read_u64(&mut self, address: u16) -> Result<u64, I2C::Error> {
        let control_word = control_word::ControlWord::new(
            true,              // Read operation
            true,              // CRC enabled
            DataLength::Len64, // Data length
            address,
        );
        let control_word = control_word.to_bytes();
        let mut data = [0u8; 9];
        // self.i2c.write_read(self.address, &packet, &mut data)?;
        self.i2c.write(self.address, &control_word)?;
        std::thread::sleep(core::time::Duration::from_millis(10)); // Wait for the device to respond (only needed in development)
        self.i2c.read(self.address, &mut data)?;

        let first_byte_tx = self.address << 1;
        let first_byte_rx = first_byte_tx | 1;

        let crc = crc::Crc::<u8>::new(&CRC_8_CCIT);
        let mut digest = crc.digest();
        digest.update(&[first_byte_tx]);
        digest.update(&control_word);
        digest.update(&[first_byte_rx]);
        digest.update(&data[..8]);
        if digest.finalize() != data[8] {
            // return Err(embedded_hal::i2c::Error::Other("CRC mismatch"));
        }

        // Safety: We always have enough data to fill the u64
        Ok(u64::from_le_bytes(unsafe {
            data[0..8].try_into().unwrap_unchecked()
        }))
    }

    pub fn read<T: Register + From<u32>>(&mut self) -> Result<T, I2C::Error> {
        let value = self.read_u32(T::ADDRESS)?;
        Ok(T::from(value))
    }
}
