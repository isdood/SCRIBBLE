#!/bin/bash
# fix_dependencies_and_optimize.sh
# Created by: isdood
# Date: 2025-01-22 02:00:17 UTC

echo "Fixing dependencies and optimizing..."

# Update Cargo.toml with num-traits dependency
cat > Cargo.toml << 'EOF_CARGO'
[package]
name = "zigzag"
version = "0.1.0"
edition = "2021"

[workspace]

[dependencies]
parking_lot = "0.12"
num-traits = "0.2"

[dev-dependencies]
criterion = "0.5"
proptest = "1.2"

[[bench]]
name = "quantum_ops"
harness = false

[[bench]]
name = "lattice_ops"
harness = false

[profile.release]
opt-level = 3
lto = true
codegen-units = 1
panic = "abort"
debug = false

[profile.bench]
opt-level = 3
lto = true
codegen-units = 1
debug = false
EOF_CARGO

# Update root workspace Cargo.toml
cat > ../../../Cargo.toml << 'EOF_WORKSPACE'
[workspace]
members = [
    "lib/zigzag",
]

[profile.release]
opt-level = 3
lto = true
codegen-units = 1
panic = "abort"
debug = false

[profile.bench]
opt-level = 3
lto = true
codegen-units = 1
debug = false
EOF_WORKSPACE

echo "Running benchmarks with optimized LTO settings..."
RUSTFLAGS='-C target-cpu=native' cargo clean
RUSTFLAGS='-C target-cpu=native' cargo bench

echo "Optimization complete!"
echo "Changes made:"
echo "1. Added num-traits dependency"
echo "2. Maintained workspace configuration"
echo "3. Kept full LTO optimization"
echo "4. Maintained optimal profile settings"
echo "5. Cleaned build before benchmarking"
echo ""
echo "The benchmarks should now run with maximum Link Time Optimization."
