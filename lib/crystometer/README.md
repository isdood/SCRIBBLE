# Crystometer 🔮

A high-performance, multi-language crystallographic analysis toolkit designed for precise crystal structure measurements and transformations.

## Overview

Crystometer combines the performance of Zig, the safety of Rust, and the scientific computing power of Julia to provide a comprehensive suite of crystallographic analysis tools. Built for researchers, material scientists, and computational crystallographers.

## Key Features

- **High-Performance Core** ⚡
  - Written in Zig for maximum performance
  - Zero-cost abstractions
  - Minimal runtime overhead
  - SIMD-optimized operations

- **Safe Abstractions** 🛡️
  - Type-safe Rust bindings
  - Memory-safe interfaces
  - Error handling with Result types
  - Thread-safe operations

- **Scientific Computing** 📊
  - Julia integration for complex calculations
  - Native matrix operations
  - Advanced statistical analysis
  - Visualization capabilities

## Installation

### Prerequisites
- Zig 0.11.0 or later
- Rust 1.70.0 or later
- Julia 1.9.0 or later

### From Source
```bash
git clone https://github.com/isdood/crystometer.git
cd crystometer
zig build
```

### Package Managers
```bash
# Rust
cargo add crystometer

# Julia
] add Crystometer
```

## Usage

### Rust
```rust
use crystometer::CrystalWrapper;

fn main() -> Result<(), &'static str> {
    // Initialize crystal structure
    let mut crystal = CrystalWrapper::new();
    
    // Perform rotations
    crystal.rotate(45.0)?;
    
    // Apply transformations
    let transform = [
        1.0, 0.0, 0.0,
        0.0, 1.0, 0.0,
        0.0, 0.0, 1.0
    ];
    crystal.transform(&transform)?;
    
    Ok(())
}
```

### Zig
```zig
const std = @import("std");
const Crystal = @import("crystal_core.zig").Crystal;

pub fn main() !void {
    var crystal = try Crystal.init();
    defer crystal.deinit();
    
    try crystal.rotate(45.0);
    try crystal.transform(&[9]f64{
        1.0, 0.0, 0.0,
        0.0, 1.0, 0.0,
        0.0, 0.0, 1.0,
    });
}
```

### Julia
```julia
using Crystometer

# Create crystal structure
crystal = Crystal()

# Perform operations
rotate!(crystal, 45.0)
transform!(crystal, [
    1.0 0.0 0.0;
    0.0 1.0 0.0;
    0.0 0.0 1.0
])
```

## Benchmarking

Crystometer includes a comprehensive benchmarking suite that measures performance across all supported languages:

```bash
zig build bench-all
```

Sample output:
```
+====================================+
|     CRYSTOMETER BENCH SUITE        |
+------------------------------------+
| Time: 2025-01-24 00:44:49          |
| User: isdood                       |
| CPU:  4 cores                      |
| Mem:  15Gi                         |
+====================================+

# ... benchmark results ...
```

## Features in Detail

### Crystal Operations
- Structure initialization
- Rotation transformations
- Matrix transformations
- Symmetry operations
- Unit cell calculations
- Space group analysis

### Analysis Tools
- Structure validation
- Symmetry detection
- Bond length calculation
- Angle measurements
- Torsion analysis
- Packing efficiency

### Visualization
- Structure rendering
- Symmetry visualization
- Bond highlighting
- Crystal packing views
- Export to common formats

## Performance

Crystometer is designed for high performance:
- Zero-cost abstractions in the core
- SIMD-optimized matrix operations
- Cache-friendly data structures
- Parallel computation support
- Minimal memory allocations

## Contributing

We welcome contributions! Please see our [Contributing Guide](CONTRIBUTING.md) for details.

### Development Setup
1. Clone the repository
2. Install dependencies
3. Run `zig build test`
4. Submit a PR with your changes

## License

MIT License - See [LICENSE](LICENSE) for details.

## Citation

If you use Crystometer in your research, please cite:

```bibtex
@software{crystometer2025,
  author = {isdood},
  title = {Crystometer: Multi-language Crystallographic Analysis Toolkit},
  year = {2025},
  url = {https://github.com/isdood/crystometer}
}
```

## Support

- [Documentation](https://crystometer.readthedocs.io/)
- [Issue Tracker](https://github.com/isdood/crystometer/issues)
- [Discussion Forum](https://github.com/isdood/crystometer/discussions)

## Acknowledgments

Special thanks to:
- The crystallography community
- Zig, Rust, and Julia language teams
- Our contributors and users

---
Last updated: 2025-01-24
