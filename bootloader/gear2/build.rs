use std::env;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

    // Rerun if any source files change
    println!("cargo:rerun-if-changed=src/");
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=linker.ld");

    // Check for QEMU installation
    if Command::new("qemu-system-x86_64").arg("--version").output().is_err() {
        println!("cargo:warning=QEMU is not installed. You may not be able to test the bootloader.");
    }

    // Generate bootloader information
    let bootloader_info = format!(
        r#"
        pub mod bootloader_info {{
        pub const STAGE: u8 = 2;  // Second stage bootloader
        pub const LOAD_ADDRESS: u32 = 0x7E00;  // Traditional location after MBR
        pub const BUILDER: &str = "{}";
        pub const BUILD_TIME: &str = "{}";

        // Add version information
        pub const VERSION_MAJOR: u8 = 0;
        pub const VERSION_MINOR: u8 = 1;
        pub const VERSION_PATCH: u8 = 0;
}}
"#,
"isdood",
"2025-01-08 07:47:50"
    );

    // Write the bootloader info to a file
    std::fs::write(
        out_dir.join("bootloader_info.rs"),
                   bootloader_info
    ).unwrap();
}
