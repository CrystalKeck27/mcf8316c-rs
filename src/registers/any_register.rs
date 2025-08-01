use arbitrary_int::u12;

use super::{algorithm_configuration::*, fault_configuration::*, hardware_configuration::*, internal_algorithm_configuration::*};

/// Enumeration of all registers in the MCF8316C-Q1
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AnyRegister {
    /// ISD Configuration
    IsdConfig(isd_config::IsdConfig),
    /// Reverse Drive Configuration
    RevDriveConfig(rev_drive_config::RevDriveConfig),
    /// Motor Startup Configuration 1
    MotorStartup1(motor_startup1::MotorStartup1),
    /// Motor Startup Configuration 2
    MotorStartup2(motor_startup2::MotorStartup2),
    /// Closed Loop Configuration 1
    ClosedLoop1(closed_loop1::ClosedLoop1),
    /// Closed Loop Configuration 2
    ClosedLoop2(closed_loop2::ClosedLoop2),
    /// Closed Loop Configuration 3
    ClosedLoop3(closed_loop3::ClosedLoop3),
    /// Closed Loop Configuration 4
    ClosedLoop4(closed_loop4::ClosedLoop4),
    /// Reference Profiles 1
    RefProfiles1(ref_profiles1::RefProfiles1),
    /// Reference Profiles 2
    RefProfiles2(ref_profiles2::RefProfiles2),
    /// Reference Profiles 3
    RefProfiles3(ref_profiles3::RefProfiles3),
    /// Reference Profiles 4
    RefProfiles4(ref_profiles4::RefProfiles4),
    /// Reference Profiles 5
    RefProfiles5(ref_profiles5::RefProfiles5),
    /// Reference Profiles 6
    RefProfiles6(ref_profiles6::RefProfiles6),
    /// Fault Configuration 1
    FaultConfig1(fault_config1::FaultConfig1),
    /// Fault Configuration 2
    FaultConfig2(fault_config2::FaultConfig2),
    /// Hardware Pin Configuration
    PinConfig(pin_config::PinConfig),
    /// Device Configuration 1
    DeviceConfig1(device_config1::DeviceConfig1),
    /// Device Configuration 2
    DeviceConfig2(device_config2::DeviceConfig2),
    /// Peripheral Configuration 1
    PeriConfig1(peri_config1::PeriConfig1),
    /// Gate Drive Configuration 1
    GdConfig1(gd_config1::GdConfig1),
    /// Gate Drive Configuration 2
    GdConfig2(gd_config2::GdConfig2),
    /// Internal Algorithm Configuration 1
    IntAlgo1(int_algo1::IntAlgo1),
    /// Internal Algorithm Configuration 2
    IntAlgo2(int_algo2::IntAlgo2),
}

impl AnyRegister {
    /// Returns the address of the register
    pub fn address(&self) -> u12 {
        match self {
            AnyRegister::IsdConfig(_) => isd_config::IsdConfig::ADDRESS,
            AnyRegister::RevDriveConfig(_) => rev_drive_config::RevDriveConfig::ADDRESS,
            AnyRegister::MotorStartup1(_) => motor_startup1::MotorStartup1::ADDRESS,
            AnyRegister::MotorStartup2(_) => motor_startup2::MotorStartup2::ADDRESS,
            AnyRegister::ClosedLoop1(_) => closed_loop1::ClosedLoop1::ADDRESS,
            AnyRegister::ClosedLoop2(_) => closed_loop2::ClosedLoop2::ADDRESS,
            AnyRegister::ClosedLoop3(_) => closed_loop3::ClosedLoop3::ADDRESS,
            AnyRegister::ClosedLoop4(_) => closed_loop4::ClosedLoop4::ADDRESS,
            AnyRegister::RefProfiles1(_) => ref_profiles1::RefProfiles1::ADDRESS,
            AnyRegister::RefProfiles2(_) => ref_profiles2::RefProfiles2::ADDRESS,
            AnyRegister::RefProfiles3(_) => ref_profiles3::RefProfiles3::ADDRESS,
            AnyRegister::RefProfiles4(_) => ref_profiles4::RefProfiles4::ADDRESS,
            AnyRegister::RefProfiles5(_) => ref_profiles5::RefProfiles5::ADDRESS,
            AnyRegister::RefProfiles6(_) => ref_profiles6::RefProfiles6::ADDRESS,
            AnyRegister::FaultConfig1(_) => fault_config1::FaultConfig1::ADDRESS,
            AnyRegister::FaultConfig2(_) => fault_config2::FaultConfig2::ADDRESS,
            AnyRegister::PinConfig(_) => pin_config::PinConfig::ADDRESS,
            AnyRegister::DeviceConfig1(_) => device_config1::DeviceConfig1::ADDRESS,
            AnyRegister::DeviceConfig2(_) => device_config2::DeviceConfig2::ADDRESS,
            AnyRegister::PeriConfig1(_) => peri_config1::PeriConfig1::ADDRESS,
            AnyRegister::GdConfig1(_) => gd_config1::GdConfig1::ADDRESS,
            AnyRegister::GdConfig2(_) => gd_config2::GdConfig2::ADDRESS,
            AnyRegister::IntAlgo1(_) => int_algo1::IntAlgo1::ADDRESS,
            AnyRegister::IntAlgo2(_) => int_algo2::IntAlgo2::ADDRESS,
        }
    }

    /// Returns the value to be sent on the i2c bus.
    pub fn value(&self) -> u32 {
        match self {
            AnyRegister::IsdConfig(reg) => reg.value(),
            AnyRegister::RevDriveConfig(reg) => reg.value(),
            AnyRegister::MotorStartup1(reg) => reg.value(),
            AnyRegister::MotorStartup2(reg) => reg.value(),
            AnyRegister::ClosedLoop1(reg) => reg.value(),
            AnyRegister::ClosedLoop2(reg) => reg.value(),
            AnyRegister::ClosedLoop3(reg) => reg.value(),
            AnyRegister::ClosedLoop4(reg) => reg.value(),
            AnyRegister::RefProfiles1(reg) => reg.value(),
            AnyRegister::RefProfiles2(reg) => reg.value(),
            AnyRegister::RefProfiles3(reg) => reg.value(),
            AnyRegister::RefProfiles4(reg) => reg.value(),
            AnyRegister::RefProfiles5(reg) => reg.value(),
            AnyRegister::RefProfiles6(reg) => reg.value(),
            AnyRegister::FaultConfig1(reg) => reg.value(),
            AnyRegister::FaultConfig2(reg) => reg.value(),
            AnyRegister::PinConfig(reg) => reg.value(),
            AnyRegister::DeviceConfig1(reg) => reg.value(),
            AnyRegister::DeviceConfig2(reg) => reg.value(),
            AnyRegister::PeriConfig1(reg) => reg.value(),
            AnyRegister::GdConfig1(reg) => reg.value(),
            AnyRegister::GdConfig2(reg) => reg.value(),
            AnyRegister::IntAlgo1(reg) => reg.value(),
            AnyRegister::IntAlgo2(reg) => reg.value(),
        }
    }

    /// Checks if the register matches the given value.
    pub fn readback_matches(&self, other: u32) -> bool {
        match self {
            AnyRegister::IsdConfig(reg) => *reg == isd_config::IsdConfig::from_value(other),
            AnyRegister::RevDriveConfig(reg) => *reg == rev_drive_config::RevDriveConfig::from_value(other),
            AnyRegister::MotorStartup1(reg) => *reg == motor_startup1::MotorStartup1::from_value(other),
            AnyRegister::MotorStartup2(reg) => *reg == motor_startup2::MotorStartup2::from_value(other),
            AnyRegister::ClosedLoop1(reg) => *reg == closed_loop1::ClosedLoop1::from_value(other),
            AnyRegister::ClosedLoop2(reg) => *reg == closed_loop2::ClosedLoop2::from_value(other),
            AnyRegister::ClosedLoop3(reg) => *reg == closed_loop3::ClosedLoop3::from_value(other),
            AnyRegister::ClosedLoop4(reg) => *reg == closed_loop4::ClosedLoop4::from_value(other),
            AnyRegister::RefProfiles1(reg) => *reg == ref_profiles1::RefProfiles1::from_value(other),
            AnyRegister::RefProfiles2(reg) => *reg == ref_profiles2::RefProfiles2::from_value(other),
            AnyRegister::RefProfiles3(reg) => *reg == ref_profiles3::RefProfiles3::from_value(other),
            AnyRegister::RefProfiles4(reg) => *reg == ref_profiles4::RefProfiles4::from_value(other),
            AnyRegister::RefProfiles5(reg) => *reg == ref_profiles5::RefProfiles5::from_value(other),
            AnyRegister::RefProfiles6(reg) => *reg == ref_profiles6::RefProfiles6::from_value(other),
            AnyRegister::FaultConfig1(reg) => *reg == fault_config1::FaultConfig1::from_value(other),
            AnyRegister::FaultConfig2(reg) => *reg == fault_config2::FaultConfig2::from_value(other),
            AnyRegister::PinConfig(reg) => *reg == pin_config::PinConfig::from_value(other),
            AnyRegister::DeviceConfig1(reg) => *reg == device_config1::DeviceConfig1::from_value(other),
            AnyRegister::DeviceConfig2(reg) => *reg == device_config2::DeviceConfig2::from_value(other),
            AnyRegister::PeriConfig1(reg) => *reg == peri_config1::PeriConfig1::from_value(other),
            AnyRegister::GdConfig1(reg) => *reg == gd_config1::GdConfig1::from_value(other),
            AnyRegister::GdConfig2(reg) => *reg == gd_config2::GdConfig2::from_value(other),
            AnyRegister::IntAlgo1(reg) => *reg == int_algo1::IntAlgo1::from_value(other),
            AnyRegister::IntAlgo2(reg) => *reg == int_algo2::IntAlgo2::from_value(other),
        }
    }
}