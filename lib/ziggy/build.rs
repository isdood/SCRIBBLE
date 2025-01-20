//! Ziggy Build Script
//! =================
//!
//! Author: Caleb J.D. Terkovics <isdood>
//! Current User: isdood
//! Created: 2025-01-19
//! Last Updated: 2025-01-20 16:49:21 UTC
//! Version: 0.1.0
//! License: MIT

use std::process::Command;
use std::{env, path::Path};

fn check_zig_installation() -> Result<(), Box<dyn std::error::Error>> {
    let zig_version = Command::new("zig")
    .arg("version")
    .output()
    .map_err(|_| "Zig compiler not found. Please install Zig: https://ziglang.org/download/")?;

    if !zig_version.status.success() {
        return Err("Failed to get Zig version".into());
    }
    Ok(())
}

fn check_build_files(project_root: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let build_zig = project_root.join("build.zig");
    let vector3d_zig = project_root.join("src").join("vector3d.zig");

    if !build_zig.exists() {
        return Err(format!("build.zig not found at {:?}", build_zig).into());
    }
    if !vector3d_zig.exists() {
        return Err(format!("vector3d.zig not found at {:?}", vector3d_zig).into());
    }
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Check Zig installation first
    check_zig_installation()?;

    let out_dir = env::var("OUT_DIR").expect("OUT_DIR not set");
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR not set");
    let project_root = Path::new(&manifest_dir);

    // Check required files exist
    check_build_files(project_root)?;

    // Build Zig code
    let build_status = Command::new("zig")
    .current_dir(project_root)
    .args(&["build"])
    .status()
    .map_err(|e| format!("Failed to execute zig build: {}", e))?;

    if !build_status.success() {
        return Err("Zig build failed".into());
    }

    // Copy the built library to the output directory
    let lib_name = if cfg!(windows) {
        "ziggy.lib"
    } else {
        "libziggy.a"
    };

    let src_lib = project_root.join("zig-out").join("lib").join(lib_name);
    let dst_lib = Path::new(&out_dir).join(lib_name);

    std::fs::copy(&src_lib, &dst_lib)
    .map_err(|e| format!("Failed to copy library from {:?} to {:?}: {}", src_lib, dst_lib, e))?;

    // Output cargo configuration
    println!("cargo:rustc-link-search={}", out_dir);
    println!("cargo:rustc-link-lib=static=ziggy");
    println!("cargo:rerun-if-changed=src/vector3d.zig");
    println!("cargo:rerun-if-changed=build.zig");

    Ok(())
}
