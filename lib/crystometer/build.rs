use std::process::Command;

fn main() {
    // Build Zig core library
    let status = Command::new("zig")
        .args(&["build", "-Doptimize=ReleaseFast"])
        .status()
        .expect("Failed to build Zig core");

    if !status.success() {
        panic!("Failed to build Zig core library");
    }

    // Link against Zig core library
    println!("cargo:rustc-link-search=zig-out/lib");
    println!("cargo:rustc-link-lib=static=crystometer");

    // Rebuild if Zig source changes
    println!("cargo:rerun-if-changed=zig/core/crystal_core.zig");
}
