# MagicMath 🧮✨

High-Performance Crystal Lattice Mathematical Operations Library

## Overview

MagicMath is a specialized mathematical library designed for quantum-aware calculations in crystal lattice systems. Part of the Crystal Computing Framework, it provides high-precision operations while maintaining quantum state coherence and stability.

## Latest Updates (2025-01-19 14:49:13 UTC)

- Moved error handling to external `errors` crate
- Made struct fields public for better accessibility
- Fixed quantum stability checks in fractal calculations
- Updated module organization for better maintainability
- Improved documentation across all modules
- Added integration with harmony_core v0.2.1
- Enhanced shard compatibility layer
- Implemented crystal_thread support

## Crystal Computing Framework Integration

MagicMath is part of a larger ecosystem of quantum-aware computing libraries:

### harmony_core (v0.2.1)
- Quantum state harmonization
- Wave function collapse management
- Coherence optimization
- Thread safety guarantees for quantum states

### shard (v0.1.5)
- Data partitioning for quantum calculations
- Memory coherence management
- State distribution across compute nodes
- Quantum-aware load balancing

### crystal_thread (v0.1.2)
- Quantum-safe thread pooling
- State-aware task scheduling
- Coherence-preserving parallel execution
- Resonance-based thread synchronization

## Features

- 🔮 Quantum-aware mathematical operations
- 🌀 Fractal generation (Julia, Mandelbrot, Custom)
- 🎯 High-precision complex number calculations
- 🔋 Quantum state tracking and stability monitoring
- 🔄 Resonance calculations
- 🛡️ Safe dereferencing operations
- 🧬 Crystal lattice optimizations
- 🔗 Harmony integration
- 💎 Shard compatibility
- 🧵 Crystal threading support

## Core Components

- `core`: Fundamental quantum-aware mathematical operations
- `fractal`: Fractal generation and iteration
- `julia`: Julia set calculations
- `brot`: Mandelbrot set calculations
- `traits`: Core traits for crystal lattice operations
- `deref`: Safe quantum dereferencing
- `constants`: System-wide mathematical constants
- `harmony`: Harmony Core integration layer
- `shard`: Shard compatibility interfaces
- `crystal`: Crystal Thread management

## Usage

```rust
use magicmath::{QuantumMath, Operation};
use harmony_core::Harmonizer;
use shard::ShardManager;
use crystal_thread::ThreadPool;

// Initialize the quantum computing stack
let mut qmath = QuantumMath::new();
let harmonizer = Harmonizer::new();
let shard_mgr = ShardManager::new();
let thread_pool = ThreadPool::new(4);

// Perform quantum-aware calculations
let result = qmath
    .with_harmony(&harmonizer)
    .with_sharding(&shard_mgr)
    .parallel(&thread_pool)
    .operate(Operation::Add, 42.0);
```

## Quantum Operations

- Basic: Add, Subtract, Multiply, Divide
- Advanced: SquareRoot, Logarithm
- Constants: Pi, Golden Ratio
- Special: Pythagorean, Fibonacci
- Fractal: Julia, Mandelbrot, Custom
- Harmony: Wave function operations
- Shard: Distributed calculations
- Crystal: Parallel quantum operations

## Stability Monitoring

All operations are monitored for quantum stability using:
- Coherence tracking
- Phase alignment
- Energy conservation
- Resonance factors
- Harmony state verification
- Shard consistency checks
- Thread quantum safety

## Requirements

- Rust 1.70.0 or higher
- External `errors` crate for error handling
- harmony_core v0.2.1 or higher
- shard v0.1.5 or higher
- crystal_thread v0.1.2 or higher

## Installation

Add to your `Cargo.toml`:
```toml
[dependencies]
magicmath = "0.1.0"
errors = "0.1.0"  # Required dependency
harmony_core = "0.2.1"
shard = "0.1.5"
crystal_thread = "0.1.2"
```

## Documentation

Comprehensive documentation is available at:
- [API Documentation](https://docs.rs/magicmath)
- [User Guide](https://github.com/isdood/magicmath/wiki)
- [Harmony Core Integration Guide](https://docs.rs/harmony_core)
- [Shard Documentation](https://docs.rs/shard)
- [Crystal Thread Guide](https://docs.rs/crystal_thread)

## License

MIT License

## Author

Caleb J.D. Terkovics <isdood>

## Contributing

1. Fork the repository
2. Create a feature branch
3. Commit your changes
4. Push to the branch
5. Create a Pull Request

## Current Status

Version: 0.1.0
Last Updated: 2025-01-19 14:49:13 UTC
Current User: isdood

## Related Projects

- [harmony_core](https://github.com/isdood/harmony_core)
- [shard](https://github.com/isdood/shard)
- [crystal_thread](https://github.com/isdood/crystal_thread)
- [errors](https://github.com/isdood/errors)
