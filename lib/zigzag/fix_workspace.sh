#!/bin/bash
# fix_workspace.sh
# Created by: isdood
# Date: 2025-01-22 00:17:52 UTC

echo "Fixing workspace configuration..."

# Update Cargo.toml with workspace configuration
cat > Cargo.toml << 'EOF'
[package]
name = "zigzag"
version = "0.1.0"
edition = "2021"

[workspace]

[dependencies]
parking_lot = "0.12"

[features]
default = []
superpurple = []

[dev-dependencies]
criterion = { version = "0.5", features = ["html_reports"] }
rand = "0.8"

[[bench]]
name = "superpurple_bench"
harness = false
path = "benches/superpurple/main.rs"
EOF

echo "Added workspace configuration to Cargo.toml"
echo "You can now run: cargo bench --features superpurple"
