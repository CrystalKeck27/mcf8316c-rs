#[derive(Debug, Clone, Copy, strum::Display)]
#[repr(u8)]
pub enum LockModeRaw {
    /// Lock detection causes latched fault; nFAULT active; Gate driver is tristated
    #[strum(to_string = "Lock detection causes latched fault; nFAULT active; Gate driver is tristated")]
    TristateNoRetry = 0x0,
    /// Lock detection causes latched fault; nFAULT active; Gate driver is tristated
    #[strum(to_string = "Lock detection causes latched fault; nFAULT active; Gate driver is tristated")]
    TristateNoRetry2 = 0x1,
    /// Lock detection causes latched fault; nFAULT active; Gate driver is in high side brake mode (All high side FETs are turned ON)
    #[strum(to_string = "Lock detection causes latched fault; nFAULT active; Gate driver is in high side brake mode (All high side FETs are turned ON)")]
    HighSideBrakeNoRetry = 0x2,
    /// Lock detection causes latched fault; nFAULT active; Gate driver is in low side brake mode (All low side FETs are turned ON)
    #[strum(to_string = "Lock detection causes latched fault; nFAULT active; Gate driver is in low side brake mode (All low side FETs are turned ON)")]
    LowSideBrakeNoRetry = 0x3,
    /// Fault automatically cleared after LCK_RETRY time. Number of retries limited to AUTO_RETRY_TIMES. If number of retries exceed AUTO_RETRY_TIMES, fault is latched; Gate driver is tristated; nFAULT active
    #[strum(to_string = "Fault automatically cleared after LCK_RETRY time. Number of retries limited to AUTO_RETRY_TIMES. If number of retries exceed AUTO_RETRY_TIMES, fault is latched; Gate driver is tristated; nFAULT active")]
    TristateAutoRetry = 0x4,
    /// Fault automatically cleared after LCK_RETRY time. Number of retries limited to AUTO_RETRY_TIMES. If number of retries exceed AUTO_RETRY_TIMES, fault is latched; Gate driver is tristated; nFAULT active
    #[strum(to_string = "Fault automatically cleared after LCK_RETRY time. Number of retries limited to AUTO_RETRY_TIMES. If number of retries exceed AUTO_RETRY_TIMES, fault is latched; Gate driver is tristated; nFAULT active")]
    TristateAutoRetry2 = 0x5,
    /// Fault automatically cleared after LCK_RETRY time. Number of retries limited to AUTO_RETRY_TIMES. If number of retries exceed AUTO_RETRY_TIMES, fault is latched; Gate driver is in high side brake mode (All high side FETs are turned ON)
    #[strum(to_string = "Fault automatically cleared after LCK_RETRY time. Number of retries limited to AUTO_RETRY_TIMES. If number of retries exceed AUTO_RETRY_TIMES, fault is latched; Gate driver is in high side brake mode (All high side FETs are turned ON)")]
    HighSideBrakeAutoRetry = 0x6,
    /// Fault automatically cleared after LCK_RETRY time. Number of retries limited to AUTO_RETRY_TIMES. If number of retries exceed AUTO_RETRY_TIMES, fault is latched; Gate driver is in low side brake mode (All low side FETs are turned ON)
    #[strum(to_string = "Fault automatically cleared after LCK_RETRY time. Number of retries limited to AUTO_RETRY_TIMES. If number of retries exceed AUTO_RETRY_TIMES, fault is latched; Gate driver is in low side brake mode (All low side FETs are turned ON)")]
    LowSideBrakeAutoRetry = 0x7,
    /// Lock detetection current limit is in report only but no action is taken; nFAULT active
    #[strum(to_string = "Lock detetection current limit is in report only but no action is taken; nFAULT active")]
    Report = 0x8,
    /// Lock is disabled
    #[strum(to_string = "Lock is disabled")]
    Disable = 0x9,
    /// Lock is disabled
    #[strum(to_string = "Lock is disabled")]
    Disable2 = 0xA,
    /// Lock is disabled
    #[strum(to_string = "Lock is disabled")]
    Disable3 = 0xB,
    /// Lock is disabled
    #[strum(to_string = "Lock is disabled")]
    Disable4 = 0xC,
    /// Lock is disabled
    #[strum(to_string = "Lock is disabled")]
    Disable5 = 0xD,
    /// Lock is disabled
    #[strum(to_string = "Lock is disabled")]
    Disable6 = 0xE,
    /// Lock is disabled
    #[strum(to_string = "Lock is disabled")]
    Disable7 = 0xF,
}

impl From<u8> for LockModeRaw {
    fn from(value: u8) -> Self {
        match value {
            0x0 => LockModeRaw::TristateNoRetry,
            0x1 => LockModeRaw::TristateNoRetry2,
            0x2 => LockModeRaw::HighSideBrakeNoRetry,
            0x3 => LockModeRaw::LowSideBrakeNoRetry,
            0x4 => LockModeRaw::TristateAutoRetry,
            0x5 => LockModeRaw::TristateAutoRetry2,
            0x6 => LockModeRaw::HighSideBrakeAutoRetry,
            0x7 => LockModeRaw::LowSideBrakeAutoRetry,
            0x8 => LockModeRaw::Report,
            0x9 => LockModeRaw::Disable,
            0xA => LockModeRaw::Disable2,
            0xB => LockModeRaw::Disable3,
            0xC => LockModeRaw::Disable4,
            0xD => LockModeRaw::Disable5,
            0xE => LockModeRaw::Disable6,
            0xF => LockModeRaw::Disable7,
            _ => panic!("Invalid value for LockMode: {}", value),
        }
    }
}

impl PartialEq for LockModeRaw {
    fn eq(&self, other: &Self) -> bool {
        let self_mode: LockMode = (*self).into();
        let other_mode: LockMode = (*other).into();
        self_mode == other_mode
    }
}

impl Eq for LockModeRaw {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LockMode {
    Enable {
        auto_retry: bool,
        driver_mode: LockIlimitDriverMode,
    },
    Report,
    Disable,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LockIlimitDriverMode {
    Tristate,
    HighSideBrake,
    LowSideBrake,
}

impl From<LockMode> for LockModeRaw {
    fn from(mode: LockMode) -> Self {
        match mode {
            LockMode::Enable { auto_retry: allow_retry, driver_mode } => {
                let mut value = match driver_mode {
                    LockIlimitDriverMode::Tristate => 0x0u8,
                    LockIlimitDriverMode::HighSideBrake => 0x2u8,
                    LockIlimitDriverMode::LowSideBrake => 0x3u8,
                };
                if allow_retry {
                    value |= 0x4; // Set the retry bit
                }
                value.into()
            }
            LockMode::Report => LockModeRaw::Report,
            LockMode::Disable => LockModeRaw::Disable,
        }
    }
}

impl From<LockModeRaw> for LockMode {
    fn from(mode: LockModeRaw) -> Self {
        match mode {
            LockModeRaw::TristateNoRetry | LockModeRaw::TristateNoRetry2 => LockMode::Enable {
                auto_retry: false,
                driver_mode: LockIlimitDriverMode::Tristate,
            },
            LockModeRaw::HighSideBrakeNoRetry => LockMode::Enable {
                auto_retry: false,
                driver_mode: LockIlimitDriverMode::HighSideBrake,
            },
            LockModeRaw::LowSideBrakeNoRetry => LockMode::Enable {
                auto_retry: false,
                driver_mode: LockIlimitDriverMode::LowSideBrake,
            },
            LockModeRaw::TristateAutoRetry | LockModeRaw::TristateAutoRetry2 => LockMode::Enable {
                auto_retry: true,
                driver_mode: LockIlimitDriverMode::Tristate,
            },
            LockModeRaw::HighSideBrakeAutoRetry => LockMode::Enable {
                auto_retry: true,
                driver_mode: LockIlimitDriverMode::HighSideBrake,
            },
            LockModeRaw::LowSideBrakeAutoRetry => LockMode::Enable {
                auto_retry: true,
                driver_mode: LockIlimitDriverMode::LowSideBrake,
            },
            LockModeRaw::Report => LockMode::Report,
            _ => LockMode::Disable,
        }
    }
}