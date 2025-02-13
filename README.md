# TMC2160 Driver

A **no_std**, embedded‑hal 1.0 driver for the TMC2160 stepper motor driver.

## Overview

This crate provides an implementation for controlling the TMC2160 stepper motor driver using the embedded‑hal 1.0 traits. It handles 40‑bit SPI transfers (8‑bit address + 32‑bit data) with bit‑field–based register manipulation (via the `bitfield` crate) and supports high‑level operations such as:

- Setting motor current (IHOLD_IRUN configuration)
- Configuring microstepping resolution
- Enabling/disabling the driver
- Controlling motor direction and stepping

A shadow register cache is maintained for write‑only registers to simplify read‑modify‑write operations.

## Features

- **No‑std Compatible:** Designed for embedded systems without the standard library.
- **Embedded‑hal 1.0 Support:** Uses the latest SPI (via `SpiBus`) and GPIO (`OutputPin`) traits.
- **Type‑Safe Register Access:** Uses the `bitfield` crate to manipulate registers according to the datasheet.
- **High‑Level API:** Simple functions for initialization, current control, microstepping, stepping, and status reading.
- **Shadow Register Cache:** Ensures correct read‑modify‑write behavior for write‑only registers.

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
tmc2160-driver = "0.1.0"
```

## Usage

Below is an example of initializing and using the TMC2160 driver. Replace the placeholder SPI and GPIO types with your specific hardware implementations.

```rust
use embedded_hal::spi::SpiBus;
use embedded_hal::digital::OutputPin;
use tmc2160_driver::{Tmc2160, Direction, MicrostepResolution};

// Pseudo-code: Replace these with your actual SPI and GPIO implementations.
struct MySpi; // Your SPI bus implementation.
struct MyPin; // Your GPIO implementation.

impl SpiBus<u8> for MySpi {
    type Error = ();
    // Implement required methods...
}

impl OutputPin for MyPin {
    type Error = ();
    // Implement required methods...
}

fn main() {
    // Create instances of your SPI and GPIO pins.
    let spi = MySpi::new();
    let cs = MyPin::new();
    let en = MyPin::new();
    let dir = MyPin::new();
    let step = MyPin::new();

    // Create the TMC2160 driver instance.
    let mut driver = Tmc2160::new(spi, cs, en, dir, step).unwrap();

    // Initialize the driver (sets default current, microstepping, etc.).
    driver.init().unwrap();

    // Enable the driver.
    driver.enable_driver().unwrap();

    // Set motor parameters.
    driver.set_current(16, 8, 4).unwrap(); // run_current=16, hold_current=8, hold_delay=4
    driver.set_microsteps(MicrostepResolution::Half).unwrap();
    driver.set_direction(Direction::CW).unwrap();

    // Generate a step pulse.
    driver.step().unwrap();

    // Retrieve and process driver status.
    let status = driver.get_driver_status().unwrap();
    // For example, check if the driver has reported a fault:
    if status.drv_err {
        // Handle error condition...
    }
}
```

## Public API
- `Tmc2160::new(spi, cs, en, dir, step) -> Result<Self, Error>`  Creates a new driver instance. It consumes the SPI bus and GPIO pins, and sets initial safe states (e.g., CS high, driver disabled).

- `init() -> Result<(), Error>`
  Configures default parameters (such as current limits and chopper settings) and prepares the driver for operation.

- `enable_driver() / disable_driver() -> Result<(), Error>`
  Activates or deactivates the motor driver by toggling the enable (EN) pin (active-low).

- `set_direction(direction: Direction) -> Result<(), Error>`
  Sets the motor rotation direction (using the Direction enum).

- `step() -> Result<(), Error>` 
  Generates a single step pulse by toggling the STEP pin. You may insert a delay if required by your hardware.

- `set_current(run_current, hold_current, hold_delay) -> Result<(), Error>`
  Configures the IHOLD_IRUN register to set the motor current.
    - run_current (0–31): motor run current (best microstepping performance for values ≥ 16)
    - hold_current (0–31): motor hold current
    - hold_delay (0–7): delay (in multiples of 2^18 clocks) before powering down the motor at standstill

- `set_microsteps(microsteps: MicrostepResolution) -> Result<(), Error>`
  Sets the microstepping resolution by updating the CHOPCONF register.

- `get_driver_status() -> Result<DriverStatus, Error>`
  Reads and decodes status registers (GSTAT and DRV_STATUS) into a DriverStatus structure.

- `reset() -> Result<(), Error>`
  Resets the driver to a safe state by re-configuring key registers.