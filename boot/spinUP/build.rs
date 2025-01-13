// build.rs
use std::env;
use std::fs;
use std::path::PathBuf;

fn main() {
    let out_dir = PathBuf::from(env::var_os("OUT_DIR").unwrap());

    // Copy linker script to the output directory
    fs::copy("src/linker.ld", out_dir.join("linker.ld")).unwrap();

    println!("cargo:rustc-link-arg=-T{}", out_dir.join("linker.ld").display());
    println!("cargo:rustc-link-arg=-nostartfiles");
    println!("cargo:rerun-if-changed=src/linker.ld");
}
