use std::env;
use std::path::PathBuf;

fn main() {
    // Output directory for the build
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

    // Tell cargo to rerun if our linker script changes
    println!("cargo:rerun-if-changed=linker.ld");
    println!("cargo:rerun-if-changed=build.rs");

    // Copy linker script to the output directory
    std::fs::copy("linker.ld", out_dir.join("linker.ld"))
    .expect("Failed to copy linker script");

    // Tell rustc to link with our linker script
    println!("cargo:rustc-link-arg=-T{}", out_dir.join("linker.ld").display());
}
