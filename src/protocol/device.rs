use embedded_hal::i2c::SevenBitAddress;
use split_owned::SplitOwned;
use thiserror::Error;
use arbitrary_int::u12;

use crate::registers::FlexibleRegister;

use super::{super::registers::Register, control_word::*};

/// MCF8316C-Q1 driver.
#[derive(Debug)]
pub struct MCF8316C<I2C: embedded_hal::i2c::I2c<SevenBitAddress>> {
    /// I2C Driver implementation field
    pub i2c: I2C,
    /// 7-bit address of the MCF8316C-Q1 device
    pub address: SevenBitAddress,
}

impl<I2C: embedded_hal::i2c::I2c<SevenBitAddress>> MCF8316C<I2C> {
    /// Creates a new instance of the MCF8316C-Q1 driver with the address set to 0x00.
    pub fn new(i2c: I2C) -> Self {
        MCF8316C {
            i2c,
            address: SevenBitAddress::from(0x00),
        }
    }

    /// Creates a new instance of the MCF8316C-Q1 driver with the specified I2C address.
    pub fn with_i2c_address(i2c: I2C, address: u8) -> Self {
        MCF8316C {
            i2c,
            address: SevenBitAddress::from(address),
        }
    }

    /// Creates a packet that would set the data at the specified address.
    pub fn create_write_u16_packet(&mut self, address: u12, data: u16) -> [u8; 6] {
        let control_word = ControlWord::new(
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
    pub fn create_write_u32_packet(&mut self, address: u12, data: u32) -> [u8; 8] {
        let control_word = ControlWord::new(
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
    pub fn create_write_u64_packet(&mut self, address: u12, data: u64) -> [u8; 12] {
        let control_word = ControlWord::new(
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
    pub fn write_u16(&mut self, address: u12, data: u16) -> Result<(), I2C::Error> {
        let packet = self.create_write_u16_packet(address, data);
        self.i2c.write(self.address, &packet)
    }

    /// Writes data to the specified address.
    pub fn write_u32(&mut self, address: u12, data: u32) -> Result<(), I2C::Error> {
        let packet = self.create_write_u32_packet(address, data);
        self.i2c.write(self.address, &packet)
    }

    /// Writes data to the specified address.
    pub fn write_u64(&mut self, address: u12, data: u64) -> Result<(), I2C::Error> {
        let packet = self.create_write_u64_packet(address, data);
        self.i2c.write(self.address, &packet)
    }

    /// Writes data to the specified register.
    pub fn write<T: FlexibleRegister>(&mut self, data: &T) -> Result<(), I2C::Error> {
        self.write_u32(data.address(), data.value())
    }

    /// Writes data to the specified register and verifies the write by reading back the value.
    pub fn write_verify<T: FlexibleRegister + ?Sized>(&mut self, data: &T) -> Result<(), ReadError<I2C::Error>> {
        self.write_u32(data.address(), data.value())?;
        data.compare_internal(self.read_u32(data.address())?)
            .then_some(())
            .ok_or(ReadError::CRCMismatch)
    }

    /// Reads data from the specified address.
    pub fn read_u16(&mut self, address: u12) -> Result<u16, ReadError<I2C::Error>> {
        let control_word = ControlWord::new(
            true,              // Read operation
            true,              // CRC enabled
            DataLength::Len16, // Data length
            address,
        );
        let control_word = control_word.to_bytes();
        let mut data_and_crc = [0u8; 3];
        self.i2c.write_read(self.address, &control_word, &mut data_and_crc)?;

        let (data, crc) = data_and_crc.split_owned::<2, 1>();
        let crc = crc[0];

        let first_byte_tx = self.address << 1;
        let first_byte_rx = first_byte_tx | 1;

        let crc_calculator = crc::Crc::<u8>::new(&CRC_8_CCIT);
        let mut digest = crc_calculator.digest();
        digest.update(&[first_byte_tx]);
        digest.update(&control_word);
        digest.update(&[first_byte_rx]);
        digest.update(&data[..2]);
        if digest.finalize() != crc {
            Err(ReadError::CRCMismatch)
        } else {
            Ok(u16::from_le_bytes(data))
        }
    }

    /// Reads data from the specified address.
    pub fn read_u32(&mut self, address: u12) -> Result<u32, ReadError<I2C::Error>> {
        let control_word = ControlWord::new(
            true,              // Read operation
            true,              // CRC enabled
            DataLength::Len32, // Data length
            address,
        );
        let control_word = control_word.to_bytes();
        let mut data_and_crc = [0u8; 5];
        self.i2c.write_read(self.address, &control_word, &mut data_and_crc)?;

        let (data, crc) = data_and_crc.split_owned::<4, 1>();
        let crc = crc[0];

        let first_byte_tx = self.address << 1;
        let first_byte_rx = first_byte_tx | 1;

        let crc_calculator = crc::Crc::<u8>::new(&CRC_8_CCIT);
        let mut digest = crc_calculator.digest();
        digest.update(&[first_byte_tx]);
        digest.update(&control_word);
        digest.update(&[first_byte_rx]);
        digest.update(&data);
        if digest.finalize() != crc {
            Err(ReadError::CRCMismatch)
        } else {
            Ok(u32::from_le_bytes(data))
        }
    }

    /// Reads data from the specified address.
    pub fn read_u64(&mut self, address: u12) -> Result<u64, ReadError<I2C::Error>> {
        let control_word = ControlWord::new(
            true,              // Read operation
            true,              // CRC enabled
            DataLength::Len64, // Data length
            address,
        );
        let control_word = control_word.to_bytes();
        let mut data_and_crc = [0u8; 9];
        self.i2c.write_read(self.address, &control_word, &mut data_and_crc)?;

        let (data, crc) = data_and_crc.split_owned::<8, 1>();
        let crc = crc[0];

        let first_byte_tx = self.address << 1;
        let first_byte_rx = first_byte_tx | 1;

        let crc_calculator = crc::Crc::<u8>::new(&CRC_8_CCIT);
        let mut digest = crc_calculator.digest();
        digest.update(&[first_byte_tx]);
        digest.update(&control_word);
        digest.update(&[first_byte_rx]);
        digest.update(&data);
        if digest.finalize() != crc {
            Err(ReadError::CRCMismatch)
        } else {
            Ok(u64::from_le_bytes(data))
        }
    }

    /// Reads a register value.
    pub fn read<T: Register>(&mut self) -> Result<T, ReadError<I2C::Error>> {
        let value = self.read_u32(T::ADDRESS)?;
        Ok(T::from_value(value))
    }
}

/// Error type for reading from the MCF8316C-Q1 device.
#[derive(Error, Debug)]
pub enum ReadError<T> {
    /// I2C communication error.
    /// If you are getting a lot of these (especially no-acks), you might want to verify that your
    /// I2C implementation supports clock stretching and that it is enabled.
    #[error("I2C error: {0}")]
    I2CError(#[from] T),
    /// CRC Mismatch.
    /// Data was likely corrupted in transit. Consider retrying the transaction.
    #[error("CRC mismatch")]
    CRCMismatch,
}
