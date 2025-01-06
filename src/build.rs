fn main() {
    // Rebuild the bootloader if any of these files change
    println!("cargo:rerun-if-changed=build.rs");
}
