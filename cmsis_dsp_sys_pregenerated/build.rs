extern crate ureq;
extern crate zip;

use std::env;
use std::error::Error;
use std::fs::{self, File, OpenOptions};
use std::io::{self, ErrorKind, Seek, SeekFrom};
use std::path::{Path, PathBuf};
use zip::read::ZipFile;
use zip::ZipArchive;

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
    match choose_library() {
        Some(library_name) => {
            download_and_link_library(library_name).unwrap();
        }
        None => {
            println!(
                "cargo:warning=CMSIS-DSP supports ARM Cortex-M processors only. \
cmsis_dsp_sys_pregenerated will not link the required library for this target."
            );
        }
    }

    // Don't needlesly re-run the build script
    println!("cargo:rerun-if-changed=build.rs");
}

/// Downloads the CMSIS pack (if necessary), extracts the library (if necessary), and tells Cargo
/// to link the library
fn download_and_link_library(library_name: &str) -> Result<(), Box<dyn Error>> {
    let out_dir = PathBuf::from(env::var_os("OUT_DIR").expect("No OUT_DIR"));

    if done_file_exists(&out_dir) {
        // Library should already be here
        let library_file_path = out_dir.join(format!("lib{}.a", library_name));
        if file_exists(&library_file_path) {
            println!("Library file already extracted");
        } else {
            // Surprisingly, a done file is there but the library is not.
            remove_done_file(&out_dir)?;
            download_library(&out_dir, library_name)?;
            add_done_file(&out_dir)?;
        }
    } else {
        download_library(&out_dir, library_name)?;
        add_done_file(&out_dir)?;
    }
    // Cleanup: Remove CMSIS pack file
    let _ = fs::remove_file(cmsis_pack_path(&out_dir));

    println!("cargo:rustc-link-search={}", out_dir.display());
    println!("cargo:rustc-link-lib={}", library_name);
    Ok(())
}

fn download_library(out_dir: &Path, library_name: &str) -> Result<(), Box<dyn Error>> {
    let pack_file = download_cmsis_pack(&out_dir)?;
    let mut zip = ZipArchive::new(pack_file)?;
    // Extract one library file
    let library_archive_path = format!("CMSIS/DSP/Lib/GCC/lib{}.a", library_name);
    let library_extracted_path = out_dir.join(format!("lib{}.a", library_name));
    let mut archive_file = zip.by_name(&library_archive_path)?;
    if fully_extracted(&archive_file, &library_extracted_path) {
        println!("lib{}.a is already fully extracted", library_name);
        Ok(())
    } else {
        println!("Extracting library {}", library_archive_path);
        io::copy(
            &mut archive_file,
            &mut File::create(library_extracted_path)?,
        )?;
        Ok(())
    }
}

fn cmsis_pack_path(out_dir: &Path) -> PathBuf {
    out_dir.join("CMSIS.pack")
}

fn download_cmsis_pack(out_dir: &Path) -> Result<File, Box<dyn Error>> {
    let pack_path = cmsis_pack_path(out_dir);
    // Determine if the file needs to be downloaded by comparing the local and remote file sizes
    if fully_downloaded(CMSIS_PACK_URL, &pack_path) {
        println!("CMSIS pack already downloaded");
        File::open(pack_path).map_err(Into::into)
    } else {
        println!("Downloading CMSIS pack...");
        download_and_open_file(CMSIS_PACK_URL, &pack_path)
    }
}

/// Returns true if path points to a file that exists, url points to a remote file that exists
/// on a server that handles HEAD requests, and the local and remote files have the same size,
/// or false otherwise
fn fully_downloaded(url: &str, path: &Path) -> bool {
    file_size(&path)
        .and_then(|local_size| {
            get_remote_file_size(url)
                .ok()
                .map(|remote_size| (local_size, remote_size))
        })
        .map(|(local_size, remote_size)| local_size == remote_size)
        .unwrap_or(false)
}

/// Returns true if path points to a file that exists and has the same size as the provided
/// archive file
fn fully_extracted(archive_file: &ZipFile, path: &Path) -> bool {
    file_size(path)
        .map(|extracted_size| archive_file.size() == extracted_size)
        .unwrap_or(false)
}

/// Chooses a library to link based on the target and enabled features, and returns its name.
/// The returned name does not include the `lib` prefix or the `.a` suffix.
/// This function returns None if the current target does not have a corresponding library.
fn choose_library() -> Option<&'static str> {
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

    // Choose the right library file
    // Based on information from https://arm-software.github.io/CMSIS_5/DSP/html/index.html
    let library_name = match (target.as_str(), endian, core) {
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
        ("thumbv8m.main-none-eabihf", Endian::Big, _) => {
            panic!("ARMv8 big-endian is not supported")
        }
        // Something else
        _ => return None,
    };
    Some(library_name)
}

/// URL where the CMSIS pack can be downloaded
const CMSIS_PACK_URL: &str =
    "https://github.com/ARM-software/CMSIS_5/releases/download/5.7.0/ARM.CMSIS.5.7.0.pack";

/// Returns the size of a file, if available
fn file_size(path: &Path) -> Option<u64> {
    fs::metadata(path).ok().map(|metadata| metadata.len())
}

fn file_exists(path: &Path) -> bool {
    fs::metadata(path).is_ok()
}

/// Downloads content from a URL to a file at the provided path, and returns an open File object
/// associated with the downloaded file. The returned File object will be configured to read starting
/// at the beginning of the file.
fn download_and_open_file(url: &str, destination: &Path) -> Result<File, Box<dyn Error>> {
    let response = ureq::get(url).call()?;
    if response.status() != 200 {
        panic!("Unexpected response status: {:#?}", response);
    }
    // Write to file
    let mut file = OpenOptions::new()
        .truncate(true)
        .read(true)
        .write(true)
        .create(true)
        .open(destination)?;
    io::copy(&mut response.into_reader(), &mut file)?;

    // Seek back to the beginning
    file.seek(SeekFrom::Start(0))?;

    Ok(file)
}

/// Sends a HEAD request and returns the value of the Content-Length header in the response
fn get_remote_file_size(url: &str) -> Result<u64, ()> {
    let response = ureq::head(url).call().map_err(drop)?;
    if response.status() != 200 {
        return Err(());
    }
    if let Some(content_length_str) = response.header("Content-Length") {
        let content_length: u64 = content_length_str.parse().map_err(drop)?;
        Ok(content_length)
    } else {
        Err(())
    }
}

fn done_file_exists(out_dir: &Path) -> bool {
    let done_path = out_dir.join(".done");
    file_exists(&done_path)
}

fn remove_done_file(out_dir: &Path) -> Result<(), io::Error> {
    let done_path = out_dir.join(".done");
    match fs::remove_file(done_path) {
        Ok(()) => Ok(()),
        Err(e) => match e.kind() {
            ErrorKind::NotFound => Ok(()), // File does not exist is not an error
            _ => Err(e),
        },
    }
}

fn add_done_file(out_dir: &Path) -> Result<(), io::Error> {
    let done_path = out_dir.join(".done");
    let _file = File::create(done_path)?;
    Ok(())
}
