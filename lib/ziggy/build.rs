use std::process::Command;
use std::env;
use std::path::PathBuf;

fn main() {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

    // Build Zig code
    let status = Command::new("zig")
    .args(&["build"])
    .current_dir("lib/ziggy")
    .status()
    .expect("Failed to build Zig code");

    if !status.success() {
        panic!("Failed to build Zig library");
    }

    println!("cargo:rustc-link-search=native={}", out_dir.display());
    println!("cargo:rustc-link-lib=static=ziggy");

    // Ensure cargo rebuilds if our Zig files change
    println!("cargo:rerun-if-changed=lib/ziggy/src/vector3d.zig");
    println!("cargo:rerun-if-changed=lib/ziggy/build.zig");
}
