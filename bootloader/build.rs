use std::process::Command;

fn main() {
    // Build the boot image
    Command::new("cargo")
    .args(&["bootimage", "--target", "x86_64-scribble.json"])
    .status()
    .unwrap();
}
