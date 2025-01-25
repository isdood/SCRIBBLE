# Seed Package Manager Documentation
**Version:** 0.5.0  
**Author:** isdood  
**Last Updated:** 2025-01-25 15:24:11

## Overview
Seed is the official package manager for the Spark ecosystem, designed to handle dependency management, project scaffolding, and build orchestration with a focus on high-performance computing workloads.

## Current Functionality

### Command Line Interface
```bash
seed <command> [options]
```

### Core Commands

#### Project Management
```bash
# Create new project
seed new <project-name>
  --template <template>   # quantum, compute, or basic
  --safety <level>       # calm, balanced, or wild
  --features <list>      # comma-separated feature flags

# Initialize existing directory
seed init
  --type <project-type>  # lib or bin

# Build project
seed build
  --release             # Release mode
  --safety <level>      # Override safety level
  --target <triple>     # Cross compilation target
```

#### Dependency Management
```bash
# Add dependency
seed add <package>[@version]
  --git <url>          # From git repository
  --path <local-path>  # From local path
  --features <list>    # Enable specific features

# Remove dependency
seed remove <package>

# Update dependencies
seed update
  --package <name>     # Update specific package
```

#### Development Tools
```bash
# Run tests
seed test
  --safety <level>     # Test with specific safety level
  --filter <pattern>   # Run specific tests

# Run benchmarks
seed bench
  --features <list>    # Enable features for benchmarking

# Format code
seed fmt
  --check             # Check formatting only
```

### Configuration

#### Seed.toml Structure
```toml
[package]
name = "quantum_compute"
version = "0.1.0"
authors = ["isdood"]
description = "Quantum computing simulation framework"
repository = "https://github.com/isdood/quantum_compute"

[features]
default = ["simd"]
simd = []
gpu = ["std/compute"]
distributed = ["std/network"]

[dependencies]
crystometer = "2.0"
resonance = { git = "https://github.com/isdood/resonance", branch = "main" }
waves = { path = "../waves", features = ["advanced"] }

[dev-dependencies]
sparktest = "1.0"
benchmark = "0.5"

[build]
safety = "calm"
target = "x86_64-unknown-linux-gnu"
optimize = true
```

### Package Resolution

#### Version Resolution
- Semantic versioning (SemVer) support
- Lock file generation (`Seed.lock`)
- Conflict resolution with dependency graph analysis

```toml
# Seed.lock
[[package]]
name = "crystometer"
version = "2.0.3"
checksum = "7d23ff90f238ad199d34c65"
source = "registry+https://seed.spark-lang.org"

[metadata]
generated = "2025-01-25T15:24:11Z"
```

### Build System

#### Build Pipeline
1. **Dependency Resolution**
   - Parse `Seed.toml`
   - Resolve version constraints
   - Generate/update `Seed.lock`

2. **Feature Resolution**
   - Calculate feature graph
   - Resolve feature conflicts
   - Enable required features

3. **Safety Level Validation**
   - Check safety level compatibility
   - Apply safety level constraints

4. **Compilation**
   - Generate build scripts
   - Invoke Forge compiler
   - Link dependencies

### Directory Structure
```
project/
├── Seed.toml           # Project configuration
├── Seed.lock           # Lock file
├── src/                # Source code
│   ├── main.spk        # Entry point
│   └── lib.spk         # Library code
├── tests/              # Test files
│   └── integration/    # Integration tests
├── benches/            # Benchmarks
├── examples/           # Example code
└── .seed/              # Build artifacts
    ├── cache/          # Package cache
    ├── build/          # Build files
    └── deps/           # Dependency objects
```

### Package Registry

#### Registry Interface
```bash
# Publish package
seed publish
  --dry-run           # Validate without publishing
  --no-verify         # Skip verification

# Search packages
seed search <query>
  --limit <n>         # Max results
  --sort <criterion>  # Sort order

# Show package info
seed info <package>
  --json              # Output as JSON
```

### Development Features

#### Workspaces
```toml
# Seed.toml in root
[workspace]
members = [
    "core",
    "cli",
    "plugins/*"
]

[workspace.dependencies]
crystometer = "2.0"
```

#### Plugin System
```rust
// plugins/gpu/src/lib.rs
#[seed_plugin]
pub fn gpu_compile(config: BuildConfig) -> Result<(), Error> {
    // GPU compilation logic
}
```

### Current Limitations
1. **Package Signing**
   - Basic checksum verification
   - No signature verification yet

2. **Cross Compilation**
   - Limited target support
   - No automatic toolchain management

3. **Cache Management**
   - Basic caching strategy
   - No advanced cache pruning

### Future Development
1. **Package Security**
   - Package signing
   - Supply chain verification
   - Vulnerability scanning

2. **Build Performance**
   - Incremental compilation
   - Distributed building
   - Better caching

3. **Plugin Ecosystem**
   - Standard plugin interface
   - Plugin discovery
   - Version compatibility checks

## Integration Example

```bash
# Create new quantum computing project
seed new quantum_sim --template quantum --safety balanced

# Add required dependencies
seed add crystometer@^2.0
seed add resonance --git https://github.com/isdood/resonance

# Enable GPU acceleration
seed add std/compute --features gpu

# Build with optimizations
seed build --release --features simd,gpu

# Run tests
seed test --safety wild
```

For more detailed information and advanced usage, please refer to:
- [Seed Package Manager Guide](https://seed.spark-lang.org/guide)
- [API Documentation](https://docs.spark-lang.org/seed)
- [Package Registry](https://seed.spark-lang.org/packages)
