//! Common types for the TMC2160 driver crate.

/// Generic error type returned by TMC2160 driver functions.
/// `SpiE` is the error type for SPI operations and `PinE` is the error type for GPIO operations.
#[derive(Debug)]
pub enum Error<SpiE, PinE> {
    /// An error occurred during an SPI transaction.
    Spi(SpiE),
    /// An error occurred while toggling or reading a GPIO pin.
    Pin(PinE),
    /// An argument was provided that is outside the accepted range.
    InvalidArgument,
    /// The driver has not been properly initialized.
    NotInitialized,
}

/// Direction for motor rotation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    /// Clockwise rotation.
    CW,
    /// Counterclockwise rotation.
    CCW,
}

/// Microstepping resolution for the driver.
/// These variants represent common microstepping modes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MicrostepResolution {
    /// Full step (1 microstep per full step).
    Full,
    /// Half step (2 microsteps per full step).
    Half,
    /// Quarter step (4 microsteps per full step).
    Quarter,
    /// Eighth step (8 microsteps per full step).
    Eighth,
    /// Sixteenth step (16 microsteps per full step).
    Sixteenth,
    /// Thirty-second step (32 microsteps per full step).
    ThirtySecond,
    /// Sixty-fourth step (64 microsteps per full step).
    SixtyFourth,
    /// 1/128 microstep (128 microsteps per full step).
    OneTwentyEighth,
    /// 1/256 microstep (256 microsteps per full step).
    TwoFiftySixth,
}

impl MicrostepResolution {
    /// Returns the bit code corresponding to the microstepping resolution
    /// as required by the CHOPCONF register's MRES field.
    ///
    /// The mapping is based on the TMC2160 datasheet:
    /// - 0: Full step (256 microsteps mode)
    /// - 1: Half step (128 microsteps mode)
    /// - 2: Quarter step (64 microsteps mode)
    /// - 3: Eighth step (32 microsteps mode)
    /// - 4: Sixteenth step (16 microsteps mode)
    /// - 5: Thirty-second step (8 microsteps mode)
    /// - 6: Sixty-fourth step (4 microsteps mode)
    /// - 7: 1/128 microstep (2 microsteps mode)
    /// - 8: 1/256 microstep (1 microstep mode)
    ///
    /// Adjust the mapping if necessary to match the datasheet.
    pub fn to_bits(self) -> u8 {
        match self {
            MicrostepResolution::Full => 0,
            MicrostepResolution::Half => 1,
            MicrostepResolution::Quarter => 2,
            MicrostepResolution::Eighth => 3,
            MicrostepResolution::Sixteenth => 4,
            MicrostepResolution::ThirtySecond => 5,
            MicrostepResolution::SixtyFourth => 6,
            MicrostepResolution::OneTwentyEighth => 7,
            MicrostepResolution::TwoFiftySixth => 8,
        }
    }
}

/// Driver status as decoded from GSTAT and DRV_STATUS registers.
/// The fields correspond to various diagnostic and fault indicators.
#[derive(Debug, Clone, Copy)]
pub struct DriverStatus {
    /// True if a reset flag is set.
    pub reset_flag: bool,
    /// True if a driver error is indicated.
    pub drv_err: bool,
    /// True if undervoltage (UV_CP) is detected.
    pub uv_cp: bool,
    /// True if short to ground is detected on motor phase A.
    pub short_to_gnd_a: bool,
    /// True if short to ground is detected on motor phase B.
    pub short_to_gnd_b: bool,
    /// True if an open load condition is detected on motor phase A.
    pub open_load_a: bool,
    /// True if an open load condition is detected on motor phase B.
    pub open_load_b: bool,
    /// StallGuard or stall detection status.
    pub stallguard_status: bool,
    /// True if stealth mode (e.g., StealthChop PWM) is active.
    pub stealth_mode: bool,
    /// Actual current scaling value (as read from DRV_STATUS, for example).
    pub cs_actual: u8,
}

/// Cache for storing write‑only register values.
/// This cache is required to ensure that read‑modify‑write operations
/// use the last known values for registers that cannot be read back.
#[derive(Debug, Default, Clone, Copy)]
pub struct RegisterCache {
    /// Cached value for the IHOLD_IRUN register.
    pub ihold_irun: u32,
    /// Cached value for the TPWMTHRS register.
    pub tpwmthrs: u32,
    /// Cached value for the COOLCONF register.
    pub coolconf: u32,
    /// Cached value for the PWMCONF register.
    pub pwmconf: u32,
    // Add additional registers here as needed.
}
