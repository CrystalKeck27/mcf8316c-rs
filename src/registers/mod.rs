//! Protocol module for the MCF8316C-Q1 driver
//! 
//! Houses the internal register format of the MCF8316C-Q1
//! 

pub mod addresses;
pub mod algorithm_configuration;
pub mod common;
pub mod fault_configuration;
pub mod hardware_configuration;
pub mod internal_algorithm_configuration;
mod any_register;
mod register;

pub use register::Register;
pub use any_register::AnyRegister;
