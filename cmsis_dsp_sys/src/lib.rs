#![no_std]
#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals)]

mod bindings;
pub use self::bindings::*;

/// C data types are defined here, because they're missing from libc and core::os::raw does not
/// exist.
mod ctypes {
    /// The C integer type
    ///
    /// This is used only for the arm_status enum.
    pub type c_int = i32;
}
