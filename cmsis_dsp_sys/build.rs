use std::env;
use std::path::PathBuf;
use std::process;

#[derive(Debug)]
enum Endian {
    Big,
    Little,
}

#[derive(Debug, PartialEq)]
enum Core {
    NotM7,
    M7,
}

fn main() {
    // Target information
    let target = env::var("TARGET").unwrap();
    let endian = env::var("CARGO_CFG_TARGET_ENDIAN").unwrap();
    let endian = match endian.as_str() {
        "big" => Endian::Big,
        "little" => Endian::Little,
        _ => panic!("Invalid endian {}", endian),
    };
    let core = if env::var_os("CARGO_FEATURE_CORTEX_M7").is_some() {
        Core::M7
    } else {
        Core::NotM7
    };

    let double_precision = env::var_os("CARGO_FEATURE_DOUBLE_PRECISION_FPU").is_some();
    if double_precision && core != Core::M7 {
        panic!("Double-precision FPU is only available on Cortex-M7 cores");
    }
    let dsp_instructions = env::var_os("CARGO_FEATURE_DSP_INSTRUCTIONS").is_some();

    // Library paths
    let manifest_dir = PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").unwrap());
    let lib_dir = manifest_dir.join("ARM.CMSIS.5.7.0/CMSIS/DSP/Lib/GCC");

    // Choose the right library file
    // Based on information from https://arm-software.github.io/CMSIS_5/DSP/html/index.html
    let lib_name: &str = match (target.as_str(), endian, core) {
        // Cortex-M0 or M0+
        ("thumbv6m-none-eabi", Endian::Little, _) => "arm_cortexM0l_math",
        ("thumbv6m-none-eabi", Endian::Big, _) => "arm_cortexM0b_math",
        // Cortex-M3
        ("thumbv7m-none-eabi", Endian::Little, _) => "arm_cortexM3l_math",
        ("thumbv7m-none-eabi", Endian::Big, _) => "arm_cortexM3b_math",
        // Cortex-M4 or M7 without FPU
        ("thumbv7em-none-eabi", Endian::Little, Core::NotM7) => "arm_cortexM4l_math",
        ("thumbv7em-none-eabi", Endian::Big, Core::NotM7) => "arm_cortexM4b_math",
        ("thumbv7em-none-eabi", Endian::Little, Core::M7) => "arm_cortexM7l_math",
        ("thumbv7em-none-eabi", Endian::Big, Core::M7) => "arm_cortexM7b_math",
        // Cortex M4 with FPU
        ("thumbv7em-none-eabihf", Endian::Little, Core::NotM7) => "arm_cortexM4lf_math",
        ("thumbv7em-none-eabihf", Endian::Big, Core::NotM7) => "arm_cortexM4bf_math",
        // Cortex-M7 with FPU
        ("thumbv7em-none-eabihf", Endian::Little, Core::M7) => {
            if double_precision {
                "arm_cortexM7lfdp_math"
            } else {
                "arm_cortexM7lfsp_math"
            }
        }
        ("thumbv7em-none-eabihf", Endian::Big, Core::M7) => {
            if double_precision {
                "arm_cortexM7bfdp_math"
            } else {
                "arm_cortexM7bfsp_math"
            }
        }
        // Cortex-M23 (ARMv8m baseline)
        ("thumbv8m.base-none-eabi", Endian::Little, _) => "arm_ARMv8MBLl_math",
        ("thumbv8m.base-none-eabi", Endian::Big, _) => panic!("ARMv8 big-endian is not supported"),
        // Cortex-M33, no FPU (ARMv8m mainline)
        ("thumbv8m.main-none-eabi", Endian::Little, _) => {
            if dsp_instructions {
                "arm_ARMv8MMLld_math"
            } else {
                "arm_ARMv8MMLl_math"
            }
        }
        ("thumbv8m.main-none-eabi", Endian::Big, _) => panic!("ARMv8 big-endian is not supported"),
        // Cortex-M33 with FPU (ARMv8m mainline)
        ("thumbv8m.main-none-eabihf", Endian::Little, _) => {
            if dsp_instructions {
                "arm_ARMv8MMLldfsp_math"
            } else {
                "arm_ARMv8MMLlfsp_math"
            }
        }
        ("thumbv8m.main-none-eabihf", Endian::Big, _) => panic!("ARMv8 big-endian is not supported"),
        // Something else
        _ => {
            // Allow the build to continue, but don't link any libraries. This allows documentation
            // to be generated on desktop computers without specifying a target.
            println!(
                "cargo:warning=CMSIS-DSP supports ARM Cortex-M processors only. \
                 cmsis_dsp_sys will not link the required library for this target."
            );
            process::exit(0);
        }
    };

    println!("cargo:rustc-link-search={}", lib_dir.display());
    println!("cargo:rustc-link-lib={}", lib_name);

    // Don't needlesly re-run the build script
    println!("cargo:rerun-if-changed=build.rs");
}
