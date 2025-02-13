//! TMC2160 driver implementation.
//!
//! This module implements the main driver logic for the TMC2160 stepper motor driver.
//!
//! It uses embedded-hal v1.0 traits for SPI and digital output, and the `bitfield` crate for safe
//! manipulation of register bitfields (see `registers.rs`).
//!
//! SPI transfers are 40 bits (8‑bit address + 32‑bit data). Write operations require the address MSB
//! set (i.e. address | 0x80), while reads use the raw address. A register cache is maintained to track
//! write‑only registers.

use crate::registers::{ChopConf, IHoldIrun, Register};
use crate::types::{Direction, DriverStatus, Error, MicrostepResolution, RegisterCache};
use embedded_hal::delay::DelayNs;
use embedded_hal::digital::OutputPin;
use embedded_hal::spi::SpiBus;

/// Main driver structure for the TMC2160.
pub struct Tmc2160<SPI, CS, EN, DIR, STEP, D> {
    spi: SPI,
    cs: CS,
    en: EN,
    dir: DIR,
    step: STEP,
    delay: D,
    /// Cache for write‑only registers.
    pub register_cache: RegisterCache,
}

impl<SPI, CS, EN, DIR, STEP, D, SpiE, PinE> Tmc2160<SPI, CS, EN, DIR, STEP, D>
where
    SPI: SpiBus<u8, Error = SpiE>,
    CS: OutputPin<Error = PinE>,
    EN: OutputPin<Error = PinE>,
    DIR: OutputPin<Error = PinE>,
    STEP: OutputPin<Error = PinE>,
    D: DelayNs,
{
    /// Creates a new TMC2160 driver instance.
    ///
    /// This consumes the SPI interface and GPIO pins, sets them to safe initial states (e.g. CS high,
    /// driver disabled), and returns a new `Tmc2160` instance.
    pub fn new(
        spi: SPI,
        mut cs: CS,
        mut en: EN,
        mut dir: DIR,
        mut step: STEP,
        delay: D,
    ) -> Result<Self, Error<SpiE, PinE>> {
        cs.set_high().map_err(Error::Pin)?;
        en.set_high().map_err(Error::Pin)?;
        dir.set_low().map_err(Error::Pin)?;
        step.set_low().map_err(Error::Pin)?;
        Ok(Self {
            spi,
            cs,
            en,
            dir,
            step,
            delay,
            register_cache: RegisterCache::default(),
        })
    }

    /// Initializes the TMC2160 with default safe configuration settings.
    ///
    /// This should be called after construction and before enabling the driver.
    pub fn init(&mut self) -> Result<(), Error<SpiE, PinE>> {
        // Set default current: run current 16, hold current 8, hold delay 4.
        self.set_current(16, 8, 4)?;
        // Set default microstepping (Full step).
        self.set_microsteps(MicrostepResolution::Full)?;
        // Configure CHOPCONF with a safe default (e.g. TOFF = 5).
        let mut chopconf = self.read_chopconf()?;
        chopconf.set_toff(5);
        self.write_chopconf(chopconf)?;
        Ok(())
    }

    /// Reads a 32-bit register value via SPI.
    ///
    /// This performs a 40-bit transfer: the first byte is the register address (read, MSB = 0)
    /// followed by four dummy bytes. The returned data is parsed as a big-endian u32.
    pub fn read_register(&mut self, reg: Register) -> Result<u32, Error<SpiE, PinE>> {
        let addr = reg as u8; // For read, MSB remains 0.
        let mut write_buf = [addr, 0, 0, 0, 0];
        let mut read_buf = [0u8; 5];
        self.cs.set_low().map_err(Error::Pin)?;
        self.spi
            .transfer(&mut write_buf, &mut read_buf)
            .map_err(Error::Spi)?;
        self.cs.set_high().map_err(Error::Pin)?;
        let value = ((read_buf[1] as u32) << 24)
            | ((read_buf[2] as u32) << 16)
            | ((read_buf[3] as u32) << 8)
            | (read_buf[4] as u32);
        Ok(value)
    }

    /// Writes a 32-bit value to a register via SPI.
    ///
    /// The address is OR'd with 0x80 to indicate a write operation. The 32-bit data is sent MSB first.
    pub fn write_register(&mut self, reg: Register, value: u32) -> Result<(), Error<SpiE, PinE>> {
        let addr = (reg as u8) | 0x80;
        let buf = [
            addr,
            (value >> 24) as u8,
            (value >> 16) as u8,
            (value >> 8) as u8,
            value as u8,
        ];
        self.cs.set_low().map_err(Error::Pin)?;
        self.spi.write(&buf).map_err(Error::Spi)?;
        self.cs.set_high().map_err(Error::Pin)?;
        self.update_register_cache(reg, value);
        Ok(())
    }

    /// Performs a read-modify-write operation on a register.
    pub fn modify_register<F>(&mut self, reg: Register, f: F) -> Result<(), Error<SpiE, PinE>>
    where
        F: FnOnce(u32) -> u32,
    {
        let val = self.read_register(reg)?;
        let new_val = f(val);
        self.write_register(reg, new_val)
    }

    /// Updates the register cache for write-only registers.
    fn update_register_cache(&mut self, reg: Register, value: u32) {
        match reg {
            Register::IHoldIrun => self.register_cache.ihold_irun = value,
            Register::TPwmThrs => self.register_cache.tpwmthrs = value,
            Register::CoolConf => self.register_cache.coolconf = value,
            Register::PwmConf => self.register_cache.pwmconf = value,
            _ => {} // Other registers are either readable or not cached.
        }
    }

    /// Enables the motor driver by setting the EN pin low (active-low).
    pub fn enable_driver(&mut self) -> Result<(), Error<SpiE, PinE>> {
        self.en.set_low().map_err(Error::Pin)
    }

    /// Disables the motor driver by setting the EN pin high.
    pub fn disable_driver(&mut self) -> Result<(), Error<SpiE, PinE>> {
        self.en.set_high().map_err(Error::Pin)
    }

    /// Sets the motor rotation direction.
    ///
    /// Maps `Direction::CW` to one logic level and `Direction::CCW` to the other.
    pub fn set_direction(&mut self, direction: Direction) -> Result<(), Error<SpiE, PinE>> {
        match direction {
            Direction::CW => self.dir.set_low().map_err(Error::Pin),
            Direction::CCW => self.dir.set_high().map_err(Error::Pin),
        }
    }

    /// Generates a single step pulse by toggling the STEP pin.
    ///
    /// If a minimum pulse width is required, insert a delay between setting the pin high and low.
    pub fn step(&mut self) -> Result<(), Error<SpiE, PinE>> {
        self.step.set_high().map_err(Error::Pin)?;

        // delay
        DelayNs::delay_ns(&mut self.delay, 1000);

        self.step.set_low().map_err(Error::Pin)
    }

    /// Sets the motor current by configuring the IHOLD_IRUN register.
    ///
    /// - `run_current` and `hold_current` must be between 0 and 31.
    /// - `hold_delay` must be between 0 and 7.
    pub fn set_current(
        &mut self,
        run_current: u8,
        hold_current: u8,
        hold_delay: u8,
    ) -> Result<(), Error<SpiE, PinE>> {
        if run_current > 31 || hold_current > 31 || hold_delay > 7 {
            return Err(Error::InvalidArgument);
        }
        let mut reg_val = IHoldIrun(0);
        reg_val.set_ihold(hold_current as u32);
        reg_val.set_irun(run_current as u32);
        reg_val.set_iholddelay(hold_delay as u32);
        self.write_register(Register::IHoldIrun, reg_val.0)
    }

    /// Sets the microstepping resolution by updating the CHOPCONF register's MRES field.
    pub fn set_microsteps(
        &mut self,
        microsteps: MicrostepResolution,
    ) -> Result<(), Error<SpiE, PinE>> {
        let mut chopconf = self.read_chopconf()?;
        chopconf.set_mres(microsteps.to_bits() as u32);
        self.write_chopconf(chopconf)
    }

    /// Reads the CHOPCONF register and returns a `ChopConf` bitfield.
    fn read_chopconf(&mut self) -> Result<ChopConf, Error<SpiE, PinE>> {
        let val = self.read_register(Register::ChopConf)?;
        Ok(ChopConf(val))
    }

    /// Writes a `ChopConf` bitfield to the CHOPCONF register.
    fn write_chopconf(&mut self, chopconf: ChopConf) -> Result<(), Error<SpiE, PinE>> {
        self.write_register(Register::ChopConf, chopconf.0)
    }

    /// Retrieves driver status by reading GSTAT and DRV_STATUS registers.
    ///
    /// Returns a `DriverStatus` struct with decoded flags.
    pub fn get_driver_status(&mut self) -> Result<DriverStatus, Error<SpiE, PinE>> {
        let gstat_val = self.read_register(Register::GStat)? as u8;
        let drv_status_val = self.read_register(Register::DrvStatus)?;

        let reset_flag = (gstat_val & 0x01) != 0;
        let drv_err = (gstat_val & 0x02) != 0;
        let uv_cp = (gstat_val & 0x04) != 0;

        // Simplified decoding of DRV_STATUS.
        let cs_actual = ((drv_status_val >> 16) & 0xFF) as u8;
        let stealth_mode = (drv_status_val & (1 << 15)) != 0;
        let stallguard_status = (drv_status_val & (1 << 14)) != 0;

        // Additional status decoding can be added here.
        Ok(DriverStatus {
            reset_flag,
            drv_err,
            uv_cp,
            short_to_gnd_a: false,
            short_to_gnd_b: false,
            open_load_a: false,
            open_load_b: false,
            stallguard_status,
            stealth_mode,
            cs_actual,
        })
    }

    /// Resets the driver to a safe state by reconfiguring key registers.
    pub fn reset(&mut self) -> Result<(), Error<SpiE, PinE>> {
        self.set_current(16, 8, 4)?;
        let mut chopconf = self.read_chopconf()?;
        chopconf.set_toff(5);
        self.write_chopconf(chopconf)?;
        Ok(())
    }
}
