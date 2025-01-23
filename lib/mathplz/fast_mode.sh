#!/bin/bash

echo "📦 Creating fast_mode optimizations for MathPLZ (2025-01-23 04:59:33 UTC)"
echo "Author: isdood"

# First update the workspace Cargo.toml with resolver = "2"
cat > "/home/guavabot1/scribble/scribble/Cargo.toml" << 'END_WORKSPACE'
[workspace]
resolver = "2"
members = [
    "lib/mathplz/lib/rust"
]

[profile.release]
lto = "fat"
codegen-units = 1
panic = "abort"
opt-level = 3
debug = false

[profile.bench]
lto = "fat"
codegen-units = 1
opt-level = 3
debug = false
END_WORKSPACE

# Update package Cargo.toml (unchanged from before)
cat > "./lib/rust/Cargo.toml" << 'END_CARGO'
[package]
name = "mathplz"
version = "0.1.0"
edition = "2021"

[lib]
path = "src/lib.rs"
name = "mathplz"
bench = true

[dependencies]
rand = "0.8"
num-complex = "0.4"
rayon = "1.7"
crossbeam = "0.8"
parking_lot = "0.12"
hashbrown = "0.14"

[dev-dependencies]
criterion = { version = "0.5", features = ["html_reports"] }

[[bench]]
name = "crystal_bench"
path = "benches/crystal_bench.rs"
harness = false

[[bench]]
name = "quantum_bench"
path = "benches/quantum_bench.rs"
harness = false

[[bench]]
name = "dna_bench"
path = "benches/dna_bench.rs"
harness = false
END_CARGO

echo "✨ Fast mode optimizations updated successfully!

Key improvements:
1. Added workspace resolver = \"2\"
2. Fixed edition compatibility
3. Maintained optimization settings
4. Clean benchmark configuration
5. Updated timestamps

Run benchmarks:
cd lib/rust && RUSTFLAGS='-C target-cpu=native' cargo bench

Created: 2025-01-23 04:59:33 UTC
Author: isdood"

chmod +x "./fast_mode.sh"
