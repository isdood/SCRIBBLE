#!/bin/bash

# ZigZag directory structure setup
# Created: 2025-01-21 20:27:27 UTC
# Author: isdood

# Colors for output
GREEN='\033[0;32m'
BLUE='\033[0;34m'
NC='\033[0m'

echo -e "${BLUE}Creating ZigZag directory structure...${NC}"

# Create directory structure
mkdir -p src/{rust,zig,julia}/{core,bridge}
mkdir -p src/zig/vector
mkdir -p src/julia/quantum
mkdir -p {tests,benches,examples}/{rust,zig,julia}
mkdir -p benches/integration

# Create initial files
touch Cargo.toml build.zig Project.toml
touch src/rust/lib.rs
touch src/zig/main.zig
touch src/julia/ZigZag.jl

# Create README with simple content
cat > README.md << 'END_README'
# ZigZag
Quantum-Aware Vector Operations Library
Author: isdood
Created: 2025-01-21 20:27:27 UTC
END_README

# Create basic Cargo.toml
cat > Cargo.toml << 'END_CARGO'
[package]
name = "zigzag"
version = "0.1.0"
edition = "2021"
authors = ["isdood"]
description = "Quantum-aware vector operations library"

[dependencies]
END_CARGO

echo -e "${GREEN}Directory structure created successfully!${NC}"

# Show created structure
echo -e "\nCreated structure:"
tree -L 3
