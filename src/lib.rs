#![no_std]

//!
//! This library provides Rust-friendly interfaces to the CMSIS DSP library, which implements
//! many mathematical and signal processing operations for ARM Cortex-M processors.
//!
//! Complete documentation for the underlying library can be found
//! [here](https://arm-software.github.io/CMSIS_5/DSP/html/index.html).
//!

extern crate cmsis_dsp_sys;

pub mod basic;
pub mod transform;

/// DSP library errors
#[derive(Debug)]
pub enum Error {
    Argument,
    Length,
    SizeMismatch,
    NanInf,
    Singular,
    TestFailure,
    Unknown,
}

trait StatusCode {
    /// Converts this status code into Ok(()) if this represents success, or an error value
    /// if this represents an error
    fn check_status(self) -> Result<()>;
}

impl StatusCode for cmsis_dsp_sys::arm_status::Type {
    fn check_status(self) -> Result<()> {
        use cmsis_dsp_sys::arm_status::*;
        match self {
            ARM_MATH_SUCCESS => Ok(()),
            ARM_MATH_ARGUMENT_ERROR => Err(Error::Argument),
            ARM_MATH_LENGTH_ERROR => Err(Error::Length),
            ARM_MATH_NANINF => Err(Error::NanInf),
            ARM_MATH_SINGULAR => Err(Error::Singular),
            ARM_MATH_SIZE_MISMATCH => Err(Error::SizeMismatch),
            ARM_MATH_TEST_FAILURE => Err(Error::TestFailure),
            _ => Err(Error::Unknown),
        }
    }
}

/// Result type alias
pub type Result<T> = ::core::result::Result<T, Error>;
