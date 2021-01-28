//!
//! # CMSIS-DSP low-level bindings
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

#![no_std]
#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals)]

mod bindings;
pub use self::bindings::*;

/// C data types are defined here, because they're missing from libc and core::os::raw does not
/// exist.
mod ctypes {
    /// The C int type
    ///
    /// This is used only for the arm_status enum.
    pub type c_int = i32;
    /// The C unsigned int type
    ///
    /// This is used for some enums.
    pub type c_uint = u32;
}
