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
    M4,
    M7,
    Other,
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
    let cortex_m4 = env::var_os("CARGO_FEATURE_CORTEX_M4").is_some();
    let cortex_m7 = env::var_os("CARGO_FEATURE_CORTEX_M7").is_some();
    let core_type = match (cortex_m4, cortex_m7) {
        (false, false) => Core::Other,
        (true, false) => Core::M4,
        (false, true) => Core::M7,
        (true, true) => {
            panic!("Invalid target: Features cortex-m4 and cortex-m7 must not both be enabled.")
        }
    };
    let double_precision = env::var_os("CARGO_FEATURE_DOUBLE_PRECISION_FPU").is_some();
    if core_type != Core::M7 && double_precision {
        panic!("Double-precision FPU is only available on Cortex-M7 cores");
    }

    // Library paths
    let manifest_dir = PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").unwrap());
    let lib_dir = manifest_dir.join("ARM.CMSIS.5.6.0/CMSIS/DSP/Lib/ARM");

    // Choose the right library file
    // Based on information from https://arm-software.github.io/CMSIS_5/DSP/html/index.html
    let lib_name: &str = match (target.as_str(), endian, core_type) {
        // Cortex M0 or M0+
        ("thumbv6m-none-eabi", Endian::Little, _) => "arm_cortexM0l_math",
        ("thumbv6m-none-eabi", Endian::Big, _) => "arm_cortexM0b_math",
        // Cortex M3
        ("thumbv7m-none-eabi", Endian::Little, _) => "arm_cortexM3l_math",
        ("thumbv7m-none-eabi", Endian::Big, _) => "arm_cortexM3b_math",
        // Cortex M4 or M7 (without FPU)
        ("thumbv7em-none-eabi", Endian::Little, Core::M4) => "arm_cortexM4l_math",
        ("thumbv7em-none-eabi", Endian::Big, Core::M4) => "arm_cortexM4b_math",
        ("thumbv7em-none-eabi", Endian::Little, Core::M7) => "arm_cortexM7l_math",
        ("thumbv7em-none-eabi", Endian::Big, Core::M7) => "arm_cortexM7b_math",
        ("thumbv7em-none-eabi", _, _) => panic!(
            "A feature cortex-m4 or cortex-m7 must be enabled to specify the target processor type"
        ),
        ("thumbv7em-none-eabihf", Endian::Little, Core::M4) => "arm_cortexM4lf_math",
        ("thumbv7em-none-eabihf", Endian::Big, Core::M4) => "arm_cortexM4bf_math",
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
        ("thumbv7em-none-eabihf", _, _) => panic!(
            "A feature cortex-m4 or cortex-m7 must be enabled to specify the target processor type"
        ),
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
}
