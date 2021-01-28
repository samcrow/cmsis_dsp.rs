#![no_std]

//!
//! This library provides Rust-friendly interfaces to the CMSIS DSP library, which implements
//! many mathematical and signal processing operations for ARM Cortex-M processors.
//!
//! Complete documentation for the underlying library can be found
//! [here](https://arm-software.github.io/CMSIS_5/DSP/html/index.html).
//!
//! ## Configuring and using the bindings
//!
//! Different versions of the library will be linked depending on the type of processor the code will run on.
//! You may need to enable some Cargo features depending on the target:
//!
//!  * Cortex-M7: Enable the feature `cortex-m7`. Also, if the processor has a double-precision floating point unit, enable
//!    the `double-precision-fpu` feature.
//!  * Cortex-M33 (target `thumbv8m.main-none-eabi` or `thumbv8m.main-none-eabihf`): If the processor supports DSP
//!    instructions, enable the `dsp-instructions` feature.
//!
//! All other targets will be configured automatically based on the target passed to cargo. If you forget to enable a
//! feature, everything should still work but it may be slower.
//!
//!

extern crate cmsis_dsp_sys_pregenerated as cmsis_dsp_sys;

pub mod basic;
pub mod transform;
pub mod complex;
#[cfg(feature = "libm")]
mod libm_c;
#[cfg(all(feature = "micromath", not(feature = "libm")))]
mod micromath_c;

use core::convert::TryInto;
use core::fmt::Debug;

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



/// Checks that all elements of the provided lengths value/tuple are equal, and that the length
/// value fits into the returned integer type. This function panics if any condition does not hold.
fn check_length<L, N>(lengths: L) -> N
    where
        L: Lengths,
        usize: TryInto<N>,
        <usize as TryInto<N>>::Error: Debug,
{
    lengths.assert_lengths_equal();
    lengths
        .length()
        .try_into()
        .expect("Length too large for size type")
}

trait Lengths {
    fn assert_lengths_equal(&self);
    fn length(&self) -> usize;
}

impl Lengths for usize {
    fn assert_lengths_equal(&self) {
        // Only one element, nothing to do
    }

    fn length(&self) -> usize {
        *self
    }
}

impl Lengths for (usize, usize) {
    fn assert_lengths_equal(&self) {
        assert_eq!(self.0, self.1);
    }

    fn length(&self) -> usize {
        self.0
    }
}

impl Lengths for (usize, usize, usize) {
    fn assert_lengths_equal(&self) {
        assert!(self.0 == self.1 && self.1 == self.2);
    }

    fn length(&self) -> usize {
        self.0
    }
}
