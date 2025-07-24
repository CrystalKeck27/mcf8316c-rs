//! I2C driver for the TI MCF8316C BLDC motor driver
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

pub use protocol::device::MCF8316C;
