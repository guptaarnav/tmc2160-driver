//! This module contains the register map definitions and bitfield structures
//! for the TMC2160 driver. It follows the datasheet specification:
//
/// # Register Map
///
/// ## 2.1 General Configuration Registers (0x00 - 0x0F)
/// | Address | Name           | Bits | Description                     |
/// |---------|----------------|------|---------------------------------|
/// | 0x00    | GCONF          | 18   | Global Configuration            |
/// | 0x01    | GSTAT          | 3    | Global Status Flags             |
/// | 0x04    | IOIN           | 8    | Read Input Pin States           |
/// | 0x06    | OTP_PROG       | -    | OTP Memory Programming          |
/// | 0x07    | OTP_READ       | -    | OTP Read                        |
/// | 0x08    | FACTORY_CONF   | 5    | Factory Configuration           |
/// | 0x09    | SHORT_CONF     | 19   | Short Circuit Detection         |
/// | 0x0A    | DRV_CONF       | 22   | Driver Strength and Protection  |
/// | 0x0B    | GLOBAL_SCALER  | 8    | Current Scaling Factor          |
/// | 0x0C    | OFFSET_READ    | 16   | Offset Calibration              |
///
/// ## 2.2 Velocity‑Based Driver Feature Control (0x10 - 0x1F)
/// | Address | Name         | Bits | Description                           |
/// |---------|--------------|------|---------------------------------------|
/// | 0x10    | IHOLD_IRUN   | 5+5+4| Current Control (Hold, Run, Delay)    |
/// | 0x11    | TPOWERDOWN   | 8    | Time to Power Down                    |
/// | 0x12    | TSTEP        | 20   | Actual Step Time                      |
/// | 0x13    | TPWMTHRS     | 20   | Velocity Threshold for PWM Mode       |
/// | 0x14    | TCOOLTHRS    | 20   | CoolStep & StallGuard Threshold       |
/// | 0x15    | THIGH        | 20   | High Velocity Threshold               |
///
/// ## 2.3 DcStep (0x33)
/// | Address | Name   | Bits | Description                          |
/// |---------|--------|------|--------------------------------------|
/// | 0x33    | VDCMIN | 23   | Minimum Velocity for DcStep          |
///
/// ## 2.4 Motor Driver Registers (0x60 - 0x7F)
/// | Address       | Name                  | Bits      | Description                              |
/// |---------------|-----------------------|-----------|------------------------------------------|
/// | 0x60–0x67   | MSLUT[0..7]          | 32 x 8    | Microstep Look‑Up Tables                |
/// | 0x68        | MSLUTSEL              | 32        | LUT Segmentation Definition             |
/// | 0x69        | MSLUTSTART            | 16        | Start Values for Microstepping           |
/// | 0x6A        | MSCNT                 | 10        | Microstep Counter                        |
/// | 0x6B        | MSCURACT              | 9+9       | Actual Motor Phase Currents              |
/// | 0x6C        | CHOPCONF              | 32        | Chopper and PWM Configuration            |
/// | 0x6D        | COOLCONF              | 25        | CoolStep and StallGuard2                 |
/// | 0x6E        | DCCTRL                | 24        | DcStep Control                           |
/// | 0x6F        | DRV_STATUS            | 32        | Diagnostics and StallGuard2 Feedback     |
/// | 0x70        | PWMCONF               | 32        | PWM Configuration                        |
/// | 0x71        | PWM_SCALE             | 9+8       | StealthChop PWM Scaling                  |
/// | 0x72        | PWM_AUTO              | 8+8       | Automatic PWM Control                    |
/// | 0x73        | LOST_STEPS            | 20        | Step Loss Counter                        |
use bitfield::bitfield;

//
/// Enumeration of TMC2160 registers.
#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub enum Register {
    // General Configuration Registers
    GConf = 0x00,
    GStat = 0x01,
    IOIN = 0x04,
    OtpProg = 0x06,
    OtpRead = 0x07,
    FactoryConf = 0x08,
    ShortConf = 0x09,
    DrvConf = 0x0A,
    GlobalScaler = 0x0B,
    OffsetRead = 0x0C,

    // Velocity‑Based Driver Feature Control
    IHoldIrun = 0x10,
    TPowerdown = 0x11,
    TStep = 0x12,
    TPwmThrs = 0x13,
    TCoolThrs = 0x14,
    THigh = 0x15,

    // DcStep
    VdcMin = 0x33,

    // Motor Driver Registers
    // MSLUT[0..7] occupy 0x60–0x67.
    MSLutSel = 0x68,
    MSLutStart = 0x69,
    MsCnt = 0x6A,
    MsCurAct = 0x6B,
    ChopConf = 0x6C,
    CoolConf = 0x6D,
    DcCtrl = 0x6E,
    DrvStatus = 0x6F,
    PwmConf = 0x70,
    PwmScale = 0x71,
    PwmAuto = 0x72,
    LostSteps = 0x73,
}

bitfield! {
    #[doc = "GConf represents the Global Configuration register (0x00).\n\nThis register contains various global configuration flags:\n\n- Bit 0: recalibrate (Zero‑crossing recalibration)\n- Bit 1: faststandstill (Shortened standstill timeout)\n- Bit 2: en_pwm_mode (Enables StealthChop PWM)\n- Bit 3: multistep_filt (Enables Step Filtering)\n- Bit 4: shaft (Inverts Motor Direction)\n- Bit 5: diag0_error (DIAG0 Active on Errors)\n- Bit 6: diag0_otpw (DIAG0 Active on Overtemperature Warning)\n- Bit 7: diag0_stall (DIAG0 Active on Stall Detection)\n- Bit 8: diag1_stall (DIAG1 Active on Stall Detection)\n- Bit 9: diag1_index (DIAG1 Active on Index Position)\n- Bit 10: diag1_onstate (DIAG1 Active when Chopper is ON)\n- Bit 11: diag1_steps_skipped (DIAG1 Toggles on Missed Steps)\n- Bit 12: diag0_int_pushpull (DIAG0 Push‑Pull Output)\n- Bit 13: diag1_pushpull (DIAG1 Push‑Pull Output)\n- Bit 14: small_hysteresis (Reduces Step Hysteresis)\n- Bit 15: stop_enable (Emergency Stop via DCEN)\n- Bit 16: direct_mode (SPI Direct Coil Current Control)"]
    #[derive(Clone, Copy)]
    pub struct GConf(u32);
    impl Debug;
    pub recalibrate, set_recalibrate: 0;
    pub faststandstill, set_faststandstill: 1;
    pub en_pwm_mode, set_en_pwm_mode: 2;
    pub multistep_filt, set_multistep_filt: 3;
    pub shaft, set_shaft: 4;
    pub diag0_error, set_diag0_error: 5;
    pub diag0_otpw, set_diag0_otpw: 6;
    pub diag0_stall, set_diag0_stall: 7;
    pub diag1_stall, set_diag1_stall: 8;
    pub diag1_index, set_diag1_index: 9;
    pub diag1_onstate, set_diag1_onstate: 10;
    pub diag1_steps_skipped, set_diag1_steps_skipped: 11;
    pub diag0_int_pushpull, set_diag0_int_pushpull: 12;
    pub diag1_pushpull, set_diag1_pushpull: 13;
    pub small_hysteresis, set_small_hysteresis: 14;
    pub stop_enable, set_stop_enable: 15;
    pub direct_mode, set_direct_mode: 16;
}

bitfield! {
    #[doc = "IHoldIrun represents the IHOLD_IRUN register (0x10).\n\n- Bits 0..=4: IHOLD (hold current, 0=1/32 … 31=32/32)\n- Bits 8..=12: IRUN (run current, 0=1/32 … 31=32/32; best microstep performance for IRUN>=16)\n- Bits 16..=19: IHOLDDELAY (hold delay; 0=instant power down, 1..15: delay per step in multiples of 2^18 clocks)"]
    #[derive(Clone, Copy)]
    pub struct IHoldIrun(u32);
    impl Debug;
    pub ihold, set_ihold: 4, 0;
    pub irun, set_irun: 12, 8;
    pub iholddelay, set_iholddelay: 19, 16;
}

bitfield! {
    #[doc = "ChopConf represents the CHOPCONF register (0x6C).\n\nA simplified view of CHOPCONF:\n- TOFF: bits 0–3\n- HSTRT: bits 4–6\n- HEND: bits 7–10\n- TBL: bits 11–12\n- CHM: bit 15\n- MRES: bits 24–27 (microstep resolution)"]
    #[derive(Clone, Copy)]
    pub struct ChopConf(u32);
    impl Debug;
    pub toff, set_toff: 3, 0;
    pub hstrt, set_hstrt: 6, 4;
    pub hend, set_hend: 10, 7;
    pub tbl, set_tbl: 12, 11;
    pub chm, set_chm: 15, 15;
    pub mres, set_mres: 27, 24;
}

bitfield! {
    #[doc = "CoolConf represents the COOLCONF register (0x6D).\n\nThis register is used for CoolStep and StallGuard2 configuration.\nExample fields:\n- StallGuard threshold: bits 0–7\n- CoolStep threshold: bits 8–15\n(Extend this definition with additional fields as needed.)"]
    #[derive(Clone, Copy)]
    pub struct CoolConf(u32);
    impl Debug;
    pub sg_thrs, set_sg_thrs: 7, 0;
    pub cool_thrs, set_cool_thrs: 15, 8;
}

bitfield! {
    #[doc = "PwmConf represents the PWMCONF register (0x70).\n\nThis register configures the PWM parameters used in StealthChop and related modes.\nExample field:\n- PWM frequency setting: bits 0–3\n(Extend this definition as required by the datasheet.)"]
    #[derive(Clone, Copy)]
    pub struct PwmConf(u32);
    impl Debug;
    pub pwm_freq, set_pwm_freq: 3, 0;
}

//
/// GSTAT (Global Status Flags) - Register 0x01 (3 bits)
#[doc = "GStat wraps the 3‑bit Global Status Flags from register 0x01."]
#[derive(Debug, Clone, Copy)]
pub struct GStat(pub u8);

//
/// IOIN (Input Pin States) - Register 0x04 (8 bits)
#[doc = "IOIn wraps the 8‑bit register used to read input pin states (register 0x04)."]
#[derive(Debug, Clone, Copy)]
pub struct IOIn(pub u8);

//
/// OTP_PROG (OTP Memory Programming) - Register 0x06
#[doc = "OtpProg is a write‑only register for OTP memory programming (register 0x06)."]
#[derive(Debug, Clone, Copy)]
pub struct OtpProg(pub u32);

//
/// OTP_READ (OTP Read) - Register 0x07
#[doc = "OtpRead is a register used for reading OTP memory (register 0x07)."]
#[derive(Debug, Clone, Copy)]
pub struct OtpRead(pub u32);

//
/// FACTORY_CONF (Factory Configuration) - Register 0x08 (5 bits)
#[doc = "FactoryConf wraps the 5‑bit factory configuration register (register 0x08)."]
#[derive(Debug, Clone, Copy)]
pub struct FactoryConf(pub u8);

//
/// SHORT_CONF (Short Circuit Detection) - Register 0x09 (19 bits)
#[doc = "ShortConf wraps the 19‑bit register for short circuit detection (register 0x09)."]
#[derive(Debug, Clone, Copy)]
pub struct ShortConf(pub u32);

//
/// DRV_CONF (Driver Strength and Protection) - Register 0x0A (22 bits)
#[doc = "DrvConf wraps the 22‑bit register for driver strength and protection (register 0x0A)."]
#[derive(Debug, Clone, Copy)]
pub struct DrvConf(pub u32);

//
/// GLOBAL_SCALER (Current Scaling Factor) - Register 0x0B (8 bits)
#[doc = "GlobalScaler wraps the 8‑bit current scaling factor register (register 0x0B)."]
#[derive(Debug, Clone, Copy)]
pub struct GlobalScaler(pub u8);

//
/// OFFSET_READ (Offset Calibration) - Register 0x0C (16 bits)
#[doc = "OffsetRead wraps the 16‑bit offset calibration register (register 0x0C)."]
#[derive(Debug, Clone, Copy)]
pub struct OffsetRead(pub u16);

//
/// TPOWERDOWN (Time to Power Down) - Register 0x11 (8 bits)
#[doc = "TPowerdown wraps the 8‑bit time to power down register (register 0x11)."]
#[derive(Debug, Clone, Copy)]
pub struct TPowerdown(pub u8);

//
/// TSTEP (Actual Step Time) - Register 0x12 (20 bits)
#[doc = "TStep wraps the 20‑bit register that provides the actual step time (register 0x12)."]
#[derive(Debug, Clone, Copy)]
pub struct TStep(pub u32);

//
/// TPWMTHRS (Velocity Threshold for PWM Mode) - Register 0x13 (20 bits)
#[doc = "TPwmThrs wraps the 20‑bit velocity threshold for PWM mode register (register 0x13)."]
#[derive(Debug, Clone, Copy)]
pub struct TPwmThrs(pub u32);

//
/// TCOOLTHRS (CoolStep & StallGuard Threshold) - Register 0x14 (20 bits)
#[doc = "TCoolThrs wraps the 20‑bit CoolStep & StallGuard threshold register (register 0x14)."]
#[derive(Debug, Clone, Copy)]
pub struct TCoolThrs(pub u32);

//
/// THIGH (High Velocity Threshold) - Register 0x15 (20 bits)
#[doc = "THigh wraps the 20‑bit high velocity threshold register (register 0x15)."]
#[derive(Debug, Clone, Copy)]
pub struct THigh(pub u32);

//
/// VDCMIN (Minimum Velocity for DcStep) - Register 0x33 (23 bits)
#[doc = "VdcMin wraps the 23‑bit register for minimum velocity in DcStep mode (register 0x33)."]
#[derive(Debug, Clone, Copy)]
pub struct VdcMin(pub u32);

//
/// MSLUT - Microstep Look‑Up Table Entries (Registers 0x60 – 0x67)
#[doc = "MSLut represents one entry (32 bits) in the microstep look‑up table (registers 0x60 to 0x67)."]
#[derive(Debug, Clone, Copy)]
pub struct MSLut(pub u32);

//
/// MSLUTSEL (LUT Segmentation Definition) - Register 0x68 (32 bits)
#[doc = "MSLutSel wraps the 32‑bit LUT segmentation definition register (register 0x68)."]
#[derive(Debug, Clone, Copy)]
pub struct MSLutSel(pub u32);

//
/// MSLUTSTART (Start Values for Microstepping) - Register 0x69 (16 bits)
#[doc = "MSLutStart wraps the 16‑bit start values for microstepping register (register 0x69)."]
#[derive(Debug, Clone, Copy)]
pub struct MSLutStart(pub u16);

//
/// MSCNT (Microstep Counter) - Register 0x6A (10 bits)
#[doc = "MsCnt wraps the 10‑bit microstep counter (register 0x6A)."]
#[derive(Debug, Clone, Copy)]
pub struct MsCnt(pub u16);

//
/// MSCURACT (Actual Motor Phase Currents) - Register 0x6B (9+9 bits)
#[doc = "MsCurAct represents the actual motor phase currents from register 0x6B.\nIt contains two 9‑bit fields for phase A and phase B currents."]
#[derive(Debug, Clone, Copy)]
pub struct MsCurAct {
    /// Current for phase A (9 bits)
    pub phase_a: u16,
    /// Current for phase B (9 bits)
    pub phase_b: u16,
}

//
/// DCCTRL (DcStep Control) - Register 0x6E (24 bits)
#[doc = "DcCtrl wraps the 24‑bit register for DcStep control (register 0x6E)."]
#[derive(Debug, Clone, Copy)]
pub struct DcCtrl(pub u32);

//
/// DRV_STATUS (Diagnostics and StallGuard2 Feedback) - Register 0x6F (32 bits)
#[doc = "DrvStatus wraps the 32‑bit diagnostics and StallGuard2 feedback register (register 0x6F)."]
#[derive(Debug, Clone, Copy)]
pub struct DrvStatus(pub u32);

//
/// PWM_SCALE (StealthChop PWM Scaling) - Register 0x71 (9+8 bits)
#[doc = "PwmScale wraps the 17‑bit StealthChop PWM scaling register (register 0x71)."]
#[derive(Debug, Clone, Copy)]
pub struct PwmScale(pub u16);

//
/// PWM_AUTO (Automatic PWM Control) - Register 0x72 (8+8 bits)
#[doc = "PwmAuto wraps the 16‑bit automatic PWM control register (register 0x72)."]
#[derive(Debug, Clone, Copy)]
pub struct PwmAuto(pub u16);

//
/// LOST_STEPS (Step Loss Counter) - Register 0x73 (20 bits)
#[doc = "LostSteps wraps the 20‑bit step loss counter register (register 0x73)."]
#[derive(Debug, Clone, Copy)]
pub struct LostSteps(pub u32);
