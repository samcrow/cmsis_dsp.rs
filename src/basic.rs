//! Basic math functions

use fixed::types::{I16F48, I18F14, I1F15, I1F31, I1F7, I34F30};

use crate::check_length;

/// Calculates the absolute value of multiple values
///
/// This is functionally equivalent to performing `dst[i] = abs(src[i])` for all values of i in
/// range.
///
/// # Panics
///
/// This function panics if src and dst do not have the same length.
pub fn abs_f32(src: &[f32], dst: &mut [f32]) {
    let length = check_length((src.len(), dst.len()));
    unsafe {
        cmsis_dsp_sys::arm_abs_f32(src.as_ptr(), dst.as_mut_ptr(), length);
    }
}

/// Calculates the absolute value of multiple values
///
/// This is functionally equivalent to performing `dst[i] = abs(src[i])` for all values of i in
/// range.
///
/// # Panics
///
/// This function panics if src and dst do not have the same length.
pub fn abs_q31(src: &[I1F31], dst: &mut [I1F31]) {
    let length = check_length((src.len(), dst.len()));
    unsafe {
        cmsis_dsp_sys::arm_abs_q31(src.as_ptr() as *const _, dst.as_mut_ptr() as *mut _, length);
    }
}

/// Calculates the absolute value of multiple values
///
/// This is functionally equivalent to performing `dst[i] = abs(src[i])` for all values of i in
/// range.
///
/// # Panics
///
/// This function panics if src and dst do not have the same length.
pub fn abs_q15(src: &[I1F15], dst: &mut [I1F15]) {
    let length = check_length((src.len(), dst.len()));
    unsafe {
        cmsis_dsp_sys::arm_abs_q15(src.as_ptr() as *const _, dst.as_mut_ptr() as *mut _, length);
    }
}

/// Calculates the absolute value of multiple values
///
/// This is functionally equivalent to performing `dst[i] = abs(src[i])` for all values of i in
/// range.
///
/// # Panics
///
/// This function panics if src and dst do not have the same length.
pub fn abs_q7(src: &[I1F7], dst: &mut [I1F7]) {
    let length = check_length((src.len(), dst.len()));
    unsafe {
        cmsis_dsp_sys::arm_abs_q7(src.as_ptr() as *const _, dst.as_mut_ptr() as *mut _, length);
    }
}

/// Calculates the absolute value of multiple values in place
///
/// This is functionally equivalent to performing `values[i] = abs(values[i])` for all values of i
/// in range.
pub fn abs_in_place_f32(values: &mut [f32]) {
    let length = check_length(values.len());
    // The CMSIS DSP function specifically does support argument aliasing. Is this really safe
    // from the Rust perspective?
    unsafe {
        let ptr = values.as_mut_ptr();
        cmsis_dsp_sys::arm_abs_f32(ptr, ptr, length);
    }
}

/// Calculates the absolute value of multiple values in place
///
/// This is functionally equivalent to performing `values[i] = abs(values[i])` for all values of i
/// in range.
pub fn abs_in_place_q31(values: &mut [I1F31]) {
    let length = check_length(values.len());
    unsafe {
        let ptr = values.as_mut_ptr();
        cmsis_dsp_sys::arm_abs_q31(ptr as *const _, ptr as *mut _, length);
    }
}

/// Calculates the absolute value of multiple values in place
///
/// This is functionally equivalent to performing `values[i] = abs(values[i])` for all values of i
/// in range.
pub fn abs_in_place_q15(values: &mut [I1F15]) {
    let length = check_length(values.len());
    unsafe {
        let ptr = values.as_mut_ptr();
        cmsis_dsp_sys::arm_abs_q15(ptr as *const _, ptr as *mut _, length);
    }
}

/// Calculates the absolute value of multiple values in place
///
/// This is functionally equivalent to performing `values[i] = abs(values[i])` for all values of i
/// in range.
pub fn abs_in_place_q7(values: &mut [I1F7]) {
    let length = check_length(values.len());
    unsafe {
        let ptr = values.as_mut_ptr();
        cmsis_dsp_sys::arm_abs_q7(ptr as *const _, ptr as *mut _, length);
    }
}

/// Adds multiple values
///
/// This is functionally equivalent to performing `dst[i] = src1[i] + src2[i]` for all values of i
/// in range.
///
/// # Panics
///
/// This function panics if src1, src2, and dst do not have the same length.
pub fn add_f32(src1: &[f32], src2: &[f32], dst: &mut [f32]) {
    let length = check_length((src1.len(), src2.len(), dst.len()));
    unsafe {
        cmsis_dsp_sys::arm_add_f32(src1.as_ptr(), src2.as_ptr(), dst.as_mut_ptr(), length);
    }
}

/// Adds multiple values
///
/// This is functionally equivalent to performing `dst[i] = src1[i] + src2[i]` for all values of i
/// in range.
///
/// # Panics
///
/// This function panics if src1, src2, and dst do not have the same length.
pub fn add_q31(src1: &[I1F31], src2: &[I1F31], dst: &mut [I1F31]) {
    let length = check_length((src1.len(), src2.len(), dst.len()));
    unsafe {
        cmsis_dsp_sys::arm_add_q31(
            src1.as_ptr() as *const _,
            src2.as_ptr() as *const _,
            dst.as_mut_ptr() as *mut _,
            length,
        );
    }
}

/// Adds multiple values
///
/// This is functionally equivalent to performing `dst[i] = src1[i] + src2[i]` for all values of i
/// in range.
///
/// # Panics
///
/// This function panics if src1, src2, and dst do not have the same length.
pub fn add_q15(src1: &[I1F15], src2: &[I1F15], dst: &mut [I1F15]) {
    let length = check_length((src1.len(), src2.len(), dst.len()));
    unsafe {
        cmsis_dsp_sys::arm_add_q15(
            src1.as_ptr() as *const _,
            src2.as_ptr() as *const _,
            dst.as_mut_ptr() as *mut _,
            length,
        );
    }
}

/// Adds multiple values
///
/// This is functionally equivalent to performing `dst[i] = src1[i] + src2[i]` for all values of i
/// in range.
///
/// # Panics
///
/// This function panics if src1, src2, and dst do not have the same length.
pub fn add_q7(src1: &[I1F7], src2: &[I1F7], dst: &mut [I1F7]) {
    let length = check_length((src1.len(), src2.len(), dst.len()));
    unsafe {
        cmsis_dsp_sys::arm_add_q7(
            src1.as_ptr() as *const _,
            src2.as_ptr() as *const _,
            dst.as_mut_ptr() as *mut _,
            length,
        );
    }
}

/// Calculates the dot product of two vectors
///
/// The returned value is the sum of `src1[i] * src2[i]` over all values of i
/// in range.
///
/// # Panics
///
/// This function panics if src1 and src2 do not have the same length.
pub fn dot_product_f32(src1: &[f32], src2: &[f32]) -> f32 {
    let length = check_length((src1.len(), src2.len()));
    let mut result = 0.0;
    unsafe {
        cmsis_dsp_sys::arm_dot_prod_f32(src1.as_ptr(), src2.as_ptr(), length, &mut result);
    }
    result
}

/// Calculates the dot product of two vectors
///
/// The returned value is the sum of `src1[i] * src2[i]` over all values of i
/// in range.
///
/// # Panics
///
/// This function panics if src1 and src2 do not have the same length.
pub fn dot_product_q31(src1: &[I1F31], src2: &[I1F31]) -> I16F48 {
    let length = check_length((src1.len(), src2.len()));
    let mut result = I16F48::from_bits(0);
    unsafe {
        cmsis_dsp_sys::arm_dot_prod_q31(
            src1.as_ptr() as *const _,
            src2.as_ptr() as *const _,
            length,
            &mut result as *mut _ as *mut _,
        );
    }
    result
}

/// Calculates the dot product of two vectors
///
/// The returned value is the sum of `src1[i] * src2[i]` over all values of i
/// in range.
///
/// # Panics
///
/// This function panics if src1 and src2 do not have the same length.
pub fn dot_product_q15(src1: &[I1F15], src2: &[I1F15]) -> I34F30 {
    let length = check_length((src1.len(), src2.len()));
    let mut result = I34F30::from_bits(0);
    unsafe {
        cmsis_dsp_sys::arm_dot_prod_q15(
            src1.as_ptr() as *const _,
            src2.as_ptr() as *const _,
            length,
            &mut result as *mut _ as *mut _,
        );
    }
    result
}

/// Calculates the dot product of two vectors
///
/// The returned value is the sum of `src1[i] * src2[i]` over all values of i
/// in range.
///
/// # Panics
///
/// This function panics if src1 and src2 do not have the same length.
pub fn dot_product_q7(src1: &[I1F7], src2: &[I1F7]) -> I18F14 {
    let length = check_length((src1.len(), src2.len()));
    let mut result = I18F14::from_bits(0);
    unsafe {
        cmsis_dsp_sys::arm_dot_prod_q7(
            src1.as_ptr() as *const _,
            src2.as_ptr() as *const _,
            length,
            &mut result as *mut _ as *mut _,
        );
    }
    result
}

/// Multiplies multiple values
///
/// This is functionally equivalent to performing `dst[i] = src1[i] * src2[i]` for all values of i
/// in range.
///
/// # Panics
///
/// This function panics if src1, src2, and dst do not have the same length.
pub fn multiply_f32(src1: &[f32], src2: &[f32], dst: &mut [f32]) {
    let length = check_length((src1.len(), src2.len(), dst.len()));
    unsafe {
        cmsis_dsp_sys::arm_mult_f32(src1.as_ptr(), src2.as_ptr(), dst.as_mut_ptr(), length);
    }
}

/// Multiplies multiple values
///
/// This is similar to performing `dst[i] = src1[i] * src2[i]` for all values of i
/// in range. This function saturates on overflow.
///
/// # Panics
///
/// This function panics if src1, src2, and dst do not have the same length.
pub fn multiply_q31(src1: &[I1F31], src2: &[I1F31], dst: &mut [I1F31]) {
    let length = check_length((src1.len(), src2.len(), dst.len()));
    unsafe {
        cmsis_dsp_sys::arm_mult_q31(
            src1.as_ptr() as *const _,
            src2.as_ptr() as *const _,
            dst.as_mut_ptr() as *mut _,
            length,
        );
    }
}

/// Multiplies multiple values
///
/// This is similar to performing `dst[i] = src1[i] * src2[i]` for all values of i
/// in range. This function saturates on overflow.
///
/// # Panics
///
/// This function panics if src1, src2, and dst do not have the same length.
pub fn multiply_q15(src1: &[I1F15], src2: &[I1F15], dst: &mut [I1F15]) {
    let length = check_length((src1.len(), src2.len(), dst.len()));
    unsafe {
        cmsis_dsp_sys::arm_mult_q15(
            src1.as_ptr() as *const _,
            src2.as_ptr() as *const _,
            dst.as_mut_ptr() as *mut _,
            length,
        );
    }
}

/// Multiplies multiple values
///
/// This is similar to performing `dst[i] = src1[i] * src2[i]` for all values of i
/// in range. This function saturates on overflow.
///
/// # Panics
///
/// This function panics if src1, src2, and dst do not have the same length.
pub fn multiply_q7(src1: &[I1F7], src2: &[I1F7], dst: &mut [I1F7]) {
    let length = check_length((src1.len(), src2.len(), dst.len()));
    unsafe {
        cmsis_dsp_sys::arm_mult_q7(
            src1.as_ptr() as *const _,
            src2.as_ptr() as *const _,
            dst.as_mut_ptr() as *mut _,
            length,
        );
    }
}
