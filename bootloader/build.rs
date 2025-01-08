use std::env;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    // Get environment variables
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

    // Rerun if any source files change
    println!("cargo:rerun-if-changed=src/");
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=linker.ld");

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
"isdood",
"2025-01-08 06:35:50"
    );

    // Write the check to a file
    std::fs::write(
        out_dir.join("boot_check.rs"),
                   check_size
    ).unwrap();
}
