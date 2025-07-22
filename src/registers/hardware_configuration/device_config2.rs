use super::*;
use arbitrary_int::*;
use bitbybit::*;

#[bitfield(u32)]
#[derive(Debug, PartialEq, Eq)]
pub struct DeviceConfig2 {
    /// Input frequency on speed pin for frequency based motor control that
    /// corresponds to 100% duty cycle. Input duty cycle = Input frequency /
    /// INPUT_MAXIMUM_FREQ
    #[bits(16..=30, rw)]
    pub input_maximum_freq: u15,
    /// Device enters sleep mode when input source is held at or below the
    /// sleep entry threshold for SLEEP_ENTRY_TIME
    #[bits(14..=15, rw)]
    pub sleep_entry_time: SleepEntryTime,
    /// Adjust CSA gain dynamically for optimal resolution across current levels.
    /// 0 = Dynamic CSA gain is disabled, 1 = Dynamic CSA gain is enabled
    #[bit(13, rw)]
    pub dynamic_csa_gain_en: bool,
    /// Adjust voltage gain dynamically for optimal voltage resolution across voltage levels.
    /// 0 = Dynamic voltage gain is disabled, 1 = Dynamic voltage gain is enabled
    #[bit(12, rw)]
    pub dynamic_voltage_gain_en: bool,
    /// Device mode select.
    /// 0 = Standby Mode, 1 = Sleep Mode
    #[bit(11, rw)]
    pub dev_mode: bool,
    /// Clock source
    #[bits(9..=10, rw)]
    pub clk_sel: Option<ClockSource>,
    /// Enable external clock mode.
    /// 0 = Disable, 1 = Enable
    #[bit(8, rw)]
    pub ext_clk_en: bool,
    /// External clock configuration
    #[bits(5..=7, rw)]
    pub ext_clk_config: ExternalClockFrequency,
    /// Enable external watchdog.
    /// 0 = Disable, 1 = Enable
    #[bit(4, rw)]
    pub ext_wdt_en: bool,
    /// Time between watchdog tickles (GPIO/I2C)
    #[bits(2..=3, rw)]
    pub ext_wdt_config: ExternalWatchdogConfig,
    /// External watchdog input mode.
    /// 0 = Watchdog tickle over I2C, 1 = Watchdog tickle over GPIO
    #[bit(1, rw)]
    pub ext_wdt_input_mode: bool,
    /// External watchdog fault mode.
    /// 0 = Report Only, 1 = Latch with FETs in Hi-Z
    #[bit(0, rw)]
    pub ext_wdt_fault_mode: bool,
}

impl Register for DeviceConfig2 {
    const ADDRESS: u16 = DEVICE_CONFIG2; // Example address, replace with actual address
}

#[bitenum(u2, exhaustive = true)]
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, strum::Display)]
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

#[bitenum(u2, exhaustive = false)]
#[derive(Debug, PartialEq, Eq, strum::Display)]
#[repr(u8)]
pub enum ClockSource {
    /// Internal Oscillator
    #[strum(to_string = "Internal Oscillator")]
    Internal = 0x0,
    /// Crude Oscillator - WDT
    #[strum(to_string = "Crude Oscillator - WDT")]
    CrudeWdt = 0x1,
    /// External Clock Input
    #[strum(to_string = "External Clock Input")]
    External = 0x3,
}

#[bitenum(u3, exhaustive = true)]
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, strum::Display)]
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

/// Time between watchdog tickles (GPIO/I2C)
#[bitenum(u2, exhaustive = true)]
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, strum::Display)]
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
