// boot/spinUP/build.rs
// Last Updated: 2025-01-13 05:36:01 UTC
// Author: Caleb J.D. Terkovics (isdood)
// Current User: isdood

use std::env;
use std::path::PathBuf;

fn main() {
    // Output directory for the build
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

    // Get the current directory
    let current_dir = env::current_dir().unwrap();
    let linker_path = current_dir.join("src/arch/x86_64/linker.ld");

    // Tell cargo to rerun if our linker script changes
    println!("cargo:rerun-if-changed=src/arch/x86_64/linker.ld");
    println!("cargo:rerun-if-changed=build.rs");

    // Copy linker script to the output directory
    std::fs::copy(&linker_path, out_dir.join("linker.ld"))
    .unwrap_or_else(|_| panic!("Failed to copy linker script from {:?}", linker_path));

    // Tell rustc to use our linker script and other necessary flags
    println!("cargo:rustc-link-arg=-T{}", out_dir.join("linker.ld").display());
    println!("cargo:rustc-link-arg=-nostartfiles");
    println!("cargo:rustc-link-arg=-static");
    println!("cargo:rustc-link-arg=-z");
    println!("cargo:rustc-link-arg=noexecstack");
}
