//! Definitions of C math functions implemented in micromath
//!
//! Micromath implements fewer functions than libm, and has no double-precision functions.


macro_rules! forward {
    // One argument, argument and result are both f32
    { $( $c_name:ident -> $micromath_name:ident ,)+ } => {
        $(
            #[no_mangle]
            pub extern "C" fn $c_name(value: f32) -> f32 {
                micromath::F32Ext::$micromath_name(value)
            }
        )+
    };
    // Two arguments, result is f32
    { $( $c_name:ident($arg1_type:ty, $arg2_type:ty) -> $micromath_name:ident ,)+ } => {
        $(
            #[no_mangle]
            pub extern "C" fn $c_name(arg1: $arg1_type, arg2: $arg2_type) -> f32 {
                micromath::F32Ext::$micromath_name(arg1, arg2)
            }
        )+
    };
}

forward! {
    absf -> abs,
    asinf -> asin,
    acosf -> acos,
    atanf -> atan,
    ceilf -> ceil,
    cosf -> cos,
    floorf -> floor,
    sinf -> sin,
    sqrtf -> sqrt,
    tanf -> tan,
    truncf -> trunc,
    roundf -> round,
    expf -> exp,
    log2f -> log2,
    log10f -> log10,
}

forward! {
    // atan2 order is (y, x) like in the C standard library
    atan2f(f32, f32) -> atan2,
    hypotf(f32, f32) -> hypot,
    powf(f32, f32) -> powf,
}
