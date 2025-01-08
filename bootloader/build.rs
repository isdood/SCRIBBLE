use std::env;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    // Get environment variables
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

    // Configure linker script
    let linker_script = manifest_dir.join("linker.ld");
    println!("cargo:rerun-if-changed=linker.ld");
    println!("cargo:rustc-link-arg=-T{}", linker_script.display());

    // Rerun if any source files change
    println!("cargo:rerun-if-changed=src/");
    println!("cargo:rerun-if-changed=build.rs");

    // Set optimization flags for release builds
    if env::var("PROFILE").unwrap() == "release" {
        println!("cargo:rustc-link-arg=-s"); // Strip symbols
        println!("cargo:rustc-link-arg=-nmagic"); // No magic sections
    }

    // Architecture-specific configurations
    println!("cargo:rustc-link-arg=-z");
    println!("cargo:rustc-link-arg=max-page-size=0x1000");
    println!("cargo:rustc-link-arg=-mno-red-zone");
    println!("cargo:rustc-link-arg=-z");
    println!("cargo:rustc-link-arg=stack-size=0x4000");

    // Check for QEMU installation
    if Command::new("qemu-system-x86_64").arg("--version").output().is_err() {
        println!("cargo:warning=QEMU is not installed. You may not be able to test the bootloader.");
    }

    // Generate boot sector size check
    let check_size = format!(
        r#"
        use std::fs;
        use std::path::Path;

        pub fn verify_bootloader_size(path: &Path) {{
        let size = fs::metadata(path).unwrap().len();
    if size > 512 {{
        panic!("Bootloader is too large: {{}} bytes (max 512)", size);
}}
}}

// Build information
pub const BUILDER: &str = "{}";
pub const BUILD_TIME: &str = "{}";
"#,
"isdood",  // Using the provided login
"2025-01-08 06:35:50"  // Using the provided UTC time
    );

    // Write the check to a file
    std::fs::write(
        out_dir.join("boot_check.rs"),
                   check_size
    ).unwrap();
}
