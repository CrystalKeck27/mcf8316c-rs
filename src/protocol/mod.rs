//! Protocol module for the MCF8316C-Q1 driver
//! 
//! Houses the structures used to communicate with the MCF8316C-Q1
//! 
//! Section 7.6
//! 

mod control_word;
mod device;

pub use control_word::*;
pub use device::*;
