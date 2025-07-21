use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DeviceConfig2 {
    /// Input frequency on speed pin for frequency based motor control that
    /// corresponds to 100% duty cycle. Input duty cycle = Input frequency /
    /// INPUT_MAXIMUM_FREQ
    pub input_maximum_freq: u16,
    /// Device enters sleep mode when input source is held at or below the
    /// sleep entry threshold for SLEEP_ENTRY_TIME
    pub sleep_entry_time: SleepEntryTime,
    /// Adjust CSA gain dynamically for optimal resolution across current levels.
    /// 0 = Dynamic CSA gain is disabled, 1 = Dynamic CSA gain is enabled
    pub dynamic_csa_gain_en: bool,
    /// Adjust voltage gain dynamically for optimal voltage resolution across voltage levels.
    /// 0 = Dynamic voltage gain is disabled, 1 = Dynamic voltage gain is enabled
    pub dynamic_voltage_gain_en: bool,
    /// Device mode select.
    /// 0 = Standby Mode, 1 = Sleep Mode
    pub dev_mode: bool,
    /// Clock source
    pub clk_sel: ClockSource,
    /// Enable external clock mode.
    /// 0 = Disable, 1 = Enable
    pub ext_clk_en: bool,
    /// External clock configuration
    pub ext_clk_config: ExternalClockFrequency,
    /// Enable external watchdog.
    /// 0 = Disable, 1 = Enable
    pub ext_wdt_en: bool,
    /// Time between watchdog tickles (GPIO/I2C)
    pub ext_wdt_config: ExternalWatchdogConfig,
    /// External watchdog input mode.
    /// 0 = Watchdog tickle over I2C, 1 = Watchdog tickle over GPIO
    pub ext_wdt_input_mode: bool,
    /// External watchdog fault mode.
    /// 0 = Report Only, 1 = Latch with FETs in Hi-Z
    pub ext_wdt_fault_mode: bool,
}

impl Register for DeviceConfig2 {
    const ADDRESS: u16 = DEVICE_CONFIG2; // Example address, replace with actual address
}

impl From<DeviceConfig2> for u32 {
    fn from(config: DeviceConfig2) -> Self {
        let mut value = 0;
        value |= (config.input_maximum_freq as u32 & 0x7FFF) << 16; // 15 bits
        value |= (config.sleep_entry_time as u32) << 14;
        value |= (config.dynamic_csa_gain_en as u32) << 13;
        value |= (config.dynamic_voltage_gain_en as u32) << 12;
        value |= (config.dev_mode as u32) << 11;
        value |= (config.clk_sel as u32) << 9;
        value |= (config.ext_clk_en as u32) << 8;
        value |= (config.ext_clk_config as u32) << 5;
        value |= (config.ext_wdt_en as u32) << 4;
        value |= (config.ext_wdt_config as u32) << 2;
        value |= (config.ext_wdt_input_mode as u32) << 1;
        value |= config.ext_wdt_fault_mode as u32;
        value
    }
}

impl From<u32> for DeviceConfig2 {
    fn from(value: u32) -> Self {
        DeviceConfig2 {
            input_maximum_freq: ((value >> 16) & 0x7FFF) as u16,
            sleep_entry_time: SleepEntryTime::from(((value >> 14) & 0x3) as u8),
            dynamic_csa_gain_en: ((value >> 13) & 0x1) != 0,
            dynamic_voltage_gain_en: ((value >> 12) & 0x1) != 0,
            dev_mode: ((value >> 11) & 0x1) != 0,
            clk_sel: ClockSource::from(((value >> 9) & 0x3) as u8),
            ext_clk_en: ((value >> 8) & 0x1) != 0,
            ext_clk_config: ExternalClockFrequency::from(((value >> 5) & 0x7) as u8),
            ext_wdt_en: ((value >> 4) & 0x1) != 0,
            ext_wdt_config: ExternalWatchdogConfig::from(((value >> 2) & 0x3) as u8),
            ext_wdt_input_mode: ((value >> 1) & 0x1) != 0,
            ext_wdt_fault_mode: (value & 0x1) != 0,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, strum::Display)]
#[repr(u8)]
pub enum SleepEntryTime {
    /// Sleep entry when SPEED pin remains low for 50μs
    #[strum(to_string = "50µs")]
    Us50 = 0x0,
    /// Sleep entry when SPEED pin remains low for 200µs
    #[strum(to_string = "200µs")]
    Us200 = 0x1,
    /// Sleep entry when SPEED pin remains low for 20ms
    #[strum(to_string = "20ms")]
    Ms20 = 0x2,
    /// Sleep entry when SPEED pin remains low for 200ms
    #[strum(to_string = "200ms")]
    Ms200 = 0x3,
}

impl From<u8> for SleepEntryTime {
    fn from(value: u8) -> Self {
        match value {
            0x0 => SleepEntryTime::Us50,
            0x1 => SleepEntryTime::Us200,
            0x2 => SleepEntryTime::Ms20,
            0x3 => SleepEntryTime::Ms200,
            _ => panic!("Invalid SleepEntryTime value"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, strum::Display)]
#[repr(u8)]
pub enum ClockSource {
    /// Internal Oscillator
    #[strum(to_string = "Internal Oscillator")]
    Internal = 0x0,
    /// Crude Oscillator - WDT
    #[strum(to_string = "Crude Oscillator - WDT")]
    CrudeWdt = 0x1,
    /// Not Applicable
    #[strum(to_string = "Not Applicable")]
    NotApplicable = 0x2,
    /// External Clock Input
    #[strum(to_string = "External Clock Input")]
    External = 0x3,
}

impl From<u8> for ClockSource {
    fn from(value: u8) -> Self {
        match value {
            0x0 => ClockSource::Internal,
            0x1 => ClockSource::CrudeWdt,
            0x2 => ClockSource::NotApplicable,
            0x3 => ClockSource::External,
            _ => panic!("Invalid ClockSource value"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, strum::Display)]
#[repr(u8)]
pub enum ExternalClockFrequency {
    /// 8 kHz
    #[strum(to_string = "8 kHz")]
    KHz8 = 0x0,
    /// 16 kHz
    #[strum(to_string = "16 kHz")]
    KHz16 = 0x1,
    /// 32 kHz
    #[strum(to_string = "32 kHz")]
    KHz32 = 0x2,
    /// 64 kHz
    #[strum(to_string = "64 kHz")]
    KHz64 = 0x3,
    /// 128 kHz
    #[strum(to_string = "128 kHz")]
    KHz128 = 0x4,
    /// 256 kHz
    #[strum(to_string = "256 kHz")]
    KHz256 = 0x5,
    /// 512 kHz
    #[strum(to_string = "512 kHz")]
    KHz512 = 0x6,
    /// 1024 kHz
    #[strum(to_string = "1024 kHz")]
    KHz1024 = 0x7,
}

impl From<u8> for ExternalClockFrequency {
    fn from(value: u8) -> Self {
        match value {
            0x0 => ExternalClockFrequency::KHz8,
            0x1 => ExternalClockFrequency::KHz16,
            0x2 => ExternalClockFrequency::KHz32,
            0x3 => ExternalClockFrequency::KHz64,
            0x4 => ExternalClockFrequency::KHz128,
            0x5 => ExternalClockFrequency::KHz256,
            0x6 => ExternalClockFrequency::KHz512,
            0x7 => ExternalClockFrequency::KHz1024,
            _ => panic!("Invalid ExternalClockFrequency value"),
        }
    }
}

/// Time between watchdog tickles (GPIO/I2C)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, strum::Display)]
#[repr(u8)]
pub enum ExternalWatchdogConfig {
    /// 100ms/1s
    #[strum(to_string = "100ms/1s")]
    X1 = 0x0,
    /// 200ms/2s
    #[strum(to_string = "200ms/2s")]
    X2 = 0x1,
    /// 500ms/5s
    #[strum(to_string = "500ms/5s")]
    X5 = 0x2,
    /// 1000ms/10s
    #[strum(to_string = "1000ms/10s")]
    X10 = 0x3,
}

impl From<u8> for ExternalWatchdogConfig {
    fn from(value: u8) -> Self {
        match value {
            0x0 => ExternalWatchdogConfig::X1,
            0x1 => ExternalWatchdogConfig::X2,
            0x2 => ExternalWatchdogConfig::X5,
            0x3 => ExternalWatchdogConfig::X10,
            _ => panic!("Invalid ExternalWatchdogConfig value"),
        }
    }
}
