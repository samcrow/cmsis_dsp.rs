# CMSIS-DSP bindings

The CMSIS-DSP library provides "a suite of common signal processing functions for use on Cortex-M and Cortex-A processor
based devices."

This package provides Rust bindings to CMSIS-DSP for Cortex-M0, M0+, M3, M4, and M7 devices.

## Supported features

High-level bindings are currently provided for basic functions and fast Fourier transforms. The CMSIS-DSP library also
has other functionality, but nobody has written Rust bindings for it yet.

### Limitations

Some CMSIS-DSP functions, like `arm_sqrt_f32`, are defined inline in the header files and are missing from the compiled
libraries. This package currently does not provide those functions.

## Configuring and using the bindings

Different versions of the library will be linked depending on the type of processor the code will run on.
You may need to enable some Cargo features depending on the target:

* Cortex-M0, M0+, M3: No features required
* Cortex-M4: Feature `cortex-m4`
* Cortex-M7: Feature `cortex-m7`, and `double-precision-fpu` if the target has a double-precision floating-point unit

## Licensing

ARM provides the CMSIS-DSP library under the Apache license 2.0. This package of bindings (cmsis_dsp_sys) is released
under the 0-clause BSD license, which is extremely permissive and does not require attribution. This means that using
CMSIS-DSP in Rust does not require any more license compliance work than using CMSIS-DSP in C.

## Links

* [CMSIS information page](https://www.arm.com/why-arm/technologies/cmsis)
* [CMSIS GitHub repository](https://github.com/ARM-software/CMSIS_5/)
* [CMSIS-DSP documentation](https://arm-software.github.io/CMSIS_5/DSP/html/index.html)

# Upgrading CMSIS

The CMSIS-DSP libraries are in the folder `ARM.CMSIS.5.6.0/CMSIS/DSP/Lib/ARM`. The file names as provided are
`[library name].lib`. The linker, however, only looks for files with names of the form `lib[library name].a`.
To make linking work, all library files must be renamed to the `lib[library name].a` format.
