//! # I2C driver for the TI MCF8316C-Q1 BLDC motor driver
//!
//! This library is heavily based on the official MCF8316C-Q1 datasheet provided by Texas Instruments.
//! The current version of the datasheet is the August 2023 version and it can be found at:
//! [MCF8316C-Q1 Datasheet](https://www.ti.com/lit/pdf/SLLSFV2).
//! 
//! Most of the struct, function, and constant names as well as documentation are taken directly from the datasheet.
//! Basically, if they suck, it's not my fault. Relevant section references can be found in module level
//! documentation as well as some struct documentation.
//! 
//! ## Warning
//! This chip uses I2C clock stretching. Make sure that your I2C implementation supports clock stretching.
//! 

#![no_std]
#![deny(missing_docs)]
#![deny(warnings)]
#![deny(missing_copy_implementations)]
#![deny(missing_debug_implementations)]
#![deny(clippy::missing_panics_doc)]
#![deny(clippy::undocumented_unsafe_blocks)]
#![deny(clippy::std_instead_of_core)]
#![deny(clippy::fallible_impl_from)]

pub mod protocol;
pub mod registers;

pub use protocol::MCF8316C;
