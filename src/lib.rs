#![no_std]

//! # TMC2160 Driver Crate
//!
//! This crate provides a no_std, embedded-hal v1.0 driver for the TMC2160 stepper motor driver.
//! It offers:
//!
//! - SPI communication with 40-bit transfers (8-bit address + 32-bit data)
//! - Bitfield manipulation using the `bitfield` crate for register definitions
//! - A high-level API for motor control (current settings, microstepping, stepping, etc.)
//!
//! ## Example Usage
//!
//! ```no_run
//! use embedded_hal::spi::SpiDevice;
//! use embedded_hal::digital::v2::OutputPin;
//! use tmc2160_driver::Tmc2160;
//!
//! // Your hardware-specific SPI and GPIO types would be used here.
//! // For example, assuming `spi`, `cs`, `en`, `dir`, and `step` have been instantiated:
//! // let mut driver = Tmc2160::new(spi, cs, en, dir, step).unwrap();
//! // driver.init().unwrap();
//! // driver.enable_driver().unwrap();
//! // driver.step().unwrap();
//! ```
//!
//! For detailed documentation, see the module docs.

pub mod registers;
pub mod tmc2160;
pub mod types;

// Re-export key public types for ease of use.
pub use tmc2160::Tmc2160;
pub use types::{Direction, DriverStatus, Error, MicrostepResolution};
