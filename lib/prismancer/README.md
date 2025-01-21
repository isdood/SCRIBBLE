# Prismancer Game Engine

A crystal-based high-performance game engine built on the Scribble framework.

## Structure
- `src/` - Source code
  - `core/` - Rust-based core engine systems
  - `render/` - Crystal-based rendering system
  - `physics/` - Julia integration for physics
  - `systems/` - Game systems and ECS
  - `low_level/` - Zig-based performance critical code
  - `parallel/` - Rust-based distributed computing
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
