#!/bin/bash
# update_workspace.sh
# Created: 2025-01-21 18:50:15
# Author: isdood

echo "[$(date -u '+%Y-%m-%d %H:%M:%S UTC')] Updating workspace configuration..."

# First, add lazuline to the workspace root
cd /home/guavabot1/scribble/scribble

# Backup the original workspace Cargo.toml
cp Cargo.toml Cargo.toml.bak

# Add lazuline to workspace members
awk '
/members = \[/ {
    print $0
    print "    \"lib/lazuline\","
    next
}
{ print }
' Cargo.toml.bak > Cargo.toml

echo "[$(date -u '+%Y-%m-%d %H:%M:%S UTC')] Updated workspace root Cargo.toml"

# Now update lazuline's Cargo.toml
cd lib/lazuline

cat > Cargo.toml << 'END'
[package]
name = "lazuline"
version = "0.1.0"
edition = "2021"
authors = ["Caleb J.D. Terkovics <isdood>"]
description = "Lazuline component for the Scribble project"
repository = "https://github.com/isdood/scribble"
license = "MIT"

[dependencies]
tokio = { version = "1.0", features = ["full"] }
rayon = "1.7"
crossbeam = "0.8"

[dev-dependencies]
criterion = { version = "0.5", features = ["html_reports"] }

[[bench]]
name = "lib"
harness = false

[package.metadata]
created = "2025-01-21"
updated = "2025-01-21 18:50:15"
current_user = "isdood"
END

# Ensure lazuline is properly added to Git (if using Git)
git add Cargo.toml
cd ../..
git add lib/lazuline
git add Cargo.toml

echo "[$(date -u '+%Y-%m-%d %H:%M:%S UTC')] ✨ Workspace configuration updated!"
echo "Run 'cargo build' in the workspace root to verify."
