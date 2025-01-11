use std::process::Command;
use std::env;
use std::path::Path;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    println!("cargo:rerun-if-changed=src/main.rs");
    println!("cargo:rerun-if-changed=build.rs");

    // Generate binary
    Command::new("nasm")
        .args(&["-f", "bin", "src/boot.asm", "-o", &format!("{}/boot.bin", out_dir)])
        .status()
        .expect("Failed to assemble bootloader");
}
