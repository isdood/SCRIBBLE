use std::process::Command;
use std::{env, path::Path};

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();

    // Build Zig code
    Command::new("zig")
    .args(&["build", "-p", &out_dir])
    .status()
    .expect("Failed to build Zig code");

    println!("cargo:rustc-link-search=native={}", out_dir);
    println!("cargo:rustc-link-lib=static=ziggy");
    println!("cargo:rerun-if-changed=src/vector3d.zig");
    println!("cargo:rerun-if-changed=build.zig");
}
