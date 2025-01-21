#!/bin/bash

# Colors for output
GREEN='\033[0;32m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

echo -e "${BLUE}🌟 Initializing Prismancer Directory Structure${NC}"

# Create main source directory structure
mkdir -p src/{core,render,physics,systems,low_level,parallel}

# Create core Rust module files
echo -e "${GREEN}Creating Rust core modules...${NC}"
for module in core render physics systems; do
    touch src/${module}/mod.rs
done

# Create specific Rust files
core_files=(engine crystal reality quantum)
render_files=(crystal_renderer shader pipeline cache)
physics_files=(quantum_physics collision dynamics)
systems_files=(world entity components)

for file in "${core_files[@]}"; do
    touch src/core/${file}.rs
done

for file in "${render_files[@]}"; do
    touch src/render/${file}.rs
done

for file in "${physics_files[@]}"; do
    touch src/physics/${file}.rs
done

for file in "${systems_files[@]}"; do
    touch src/systems/${file}.rs
done

# Create Zig files
echo -e "${GREEN}Creating Zig low-level components...${NC}"
zig_files=(memory geometry cache vulkan)
for file in "${zig_files[@]}"; do
    touch src/low_level/${file}.zig
done

# Create Chapel files
echo -e "${GREEN}Creating Chapel parallel computing files...${NC}"
chapel_files=(distribution sync coherence)
for file in "${chapel_files[@]}"; do
    touch src/parallel/${file}.chpl
done

# Create additional directories
echo -e "${GREEN}Creating supporting directories...${NC}"
mkdir -p include/ffi \
         examples/{basic_game,physics_demo,render_test} \
         tests/{core,render,physics,integration} \
         benches/{render,physics,memory} \
         build/{rust,zig,julia,chapel} \
         docs/{api,guides,examples} \
         scripts

# Create FFI headers
touch include/prismancer.h
touch include/ffi/{julia_bridge,zig_bridge,chapel_bridge}.h

# Create build and utility scripts
echo -e "${GREEN}Creating utility scripts...${NC}"
for script in build test benchmark; do
    touch scripts/${script}.sh
    chmod +x scripts/${script}.sh
done

# Create basic README files
echo -e "${GREEN}Creating documentation files...${NC}"
cat > README.md << 'EOF'
# Prismancer Game Engine

A crystal-based high-performance game engine built on the Scribble framework.

## Structure
- `src/` - Source code
  - `core/` - Rust-based core engine systems
  - `render/` - Crystal-based rendering system
  - `physics/` - Julia integration for physics
  - `systems/` - Game systems and ECS
  - `low_level/` - Zig-based performance critical code
  - `parallel/` - Chapel-based distributed computing
- `include/` - Public headers and FFI interfaces
- `examples/` - Example implementations
- `tests/` - Test suites
- `benches/` - Performance benchmarks
- `build/` - Build configuration
- `docs/` - Documentation
- `scripts/` - Build and utility scripts

## Getting Started
1. Build the engine: `./scripts/build.sh`
2. Run tests: `./scripts/test.sh`
3. Check performance: `./scripts/benchmark.sh`

## License
TBD
EOF

# Create basic Cargo.toml
cat > Cargo.toml << 'EOF'
[package]
name = "prismancer"
version = "0.1.0"
edition = "2021"
description = "Crystal-based high-performance game engine"
authors = ["isdood"]

[dependencies]
# Core dependencies will go here

[build-dependencies]
# Build dependencies will go here

[dev-dependencies]
# Development dependencies will go here
EOF

# Create basic build.zig
cat > build.zig << 'EOF'
const std = @import("std");

pub fn build(b: *std.build.Builder) void {
    // Standard release options
    const mode = b.standardReleaseOptions();
    const target = b.standardTargetOptions(.{});

    // Library
    const lib = b.addStaticLibrary("prismancer-zig", "src/low_level/main.zig");
    lib.setBuildMode(mode);
    lib.setTarget(target);
    lib.install();
}
EOF

echo -e "${BLUE}✨ Prismancer directory structure initialized!${NC}"
echo -e "${BLUE}Next steps:${NC}"
echo -e "1. Review the created structure"
echo -e "2. Configure build scripts in scripts/"
echo -e "3. Start implementing core components"
