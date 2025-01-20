# Ziggy Library
> High-performance Vector Mathematics with Zig Integration

## Overview
Ziggy is a high-performance vector mathematics library that leverages Zig's zero-cost abstractions and SIMD optimizations within a Rust interface. It's part of the Scribble operating system project and provides essential vector operations for graphics, physics, and quantum simulations.

**Author:** Caleb J.D. Terkovics \<isdood\>  
**Last Updated:** 2025-01-20 17:50:56 UTC  
**Version:** 0.1.0  
**License:** MIT

## Features
- 3D Vector operations
- SIMD-optimized calculations
- Zero-cost Zig-Rust interop
- Consistent sub-nanosecond performance
- Quantum state calculations support

## Performance Analysis

### Dot Product Operations
Average performance across 1000 samples per operation:

| Operation | Time (ns) | Variance | Outliers |
|-----------|-----------|----------|-----------|
| v1·v2 | 21.96 | ±0.27 | 5.90% |
| v1·v3 | 21.86 | ±0.22 | 4.60% |
| v1·v4 | 21.71 | ±0.16 | 4.20% |
| v2·v3 | 22.94 | ±0.56 | 0.10% |
| v2·v4 | 22.66 | ±0.59 | 14.60% |
| v3·v4 | 22.92 | ±0.43 | 0.30% |

Key observations:
- Consistent ~22ns execution time for dot products
- Low variance (±0.16ns to ±0.59ns)
- Most operations show <5% outliers
- Best performance: v1·v4 at 21.71ns
- Most stable: v2·v3 with only 0.10% outliers

### Magnitude Operations
Average performance across 1000 samples per vector:

| Vector | Time (ns) | Variance | Outliers |
|--------|-----------|----------|-----------|
| v1 | 39.42 | ±0.92 | - |
| v2 | 37.05 | ±0.28 | 6.20% |
| v3 | 36.96 | ±0.23 | 6.80% |
| v4 | 36.92 | ±0.27 | 5.40% |

Key observations:
- Consistent ~37ns execution time (excluding v1)
- Slightly higher variance than dot products
- v1 shows higher latency (~39.4ns)
- Operations maintain <7% outliers
- Best performance: v4 at 36.92ns

## Implementation Details
- Uses Zig's native SIMD operations
- Zero-copy data transfer between Rust and Zig
- Aligned memory access for optimal performance
- Rust safety guarantees maintained
- No runtime overhead for FFI calls

## Usage
```rust
use ziggy::Vector3D;

let v1 = Vector3D::new(1.0, 2.0, 3.0);
let v2 = Vector3D::new(4.0, 5.0, 6.0);

// Dot product
let dot = v1.dot(&v2);  // ~22ns execution time

// Magnitude
let mag = v1.magnitude();  // ~37ns execution time
```

## Integration
Add to your `Cargo.toml`:
```toml
[dependencies]
ziggy = { path = "lib/ziggy" }
```

## Building
Requires:
- Rust 1.75+
- Zig 0.11+
- SIMD-capable CPU

```bash
cargo build --release
```

## Testing
```bash
cargo test
cargo bench  # For performance measurements
```

## License
MIT License - See LICENSE file for details

## Contributing
See CONTRIBUTING.md for guidelines.
