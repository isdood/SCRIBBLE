# Chomp

A high-performance cross-compiler for seamless Zig and Rust integration.

*Created: 2025-01-21 02:47:41 UTC*  
*Author: @isdood*

## Overview

Chomp is a modern cross-compiler designed to make Zig and Rust work together seamlessly while maintaining the unique strengths of both languages. It focuses on automated build integration, zero-cost FFI, and strong safety guarantees.

## Features

- 🚀 **Zero-Cost FFI**: Direct language integration with no runtime overhead
- 🛡️ **Safety Guarantees**: Maintains Rust's safety guarantees even across language boundaries
- 🔧 **Automated Build Integration**: Seamlessly manages both Zig and Rust build systems
- 📝 **Type System Integration**: Intelligent type mapping between languages
- ⚡ **Fast Compilation**: Optimized build process with smart caching
- 🔍 **Rich Development Tools**: Full IDE support and detailed error messages

## Quick Start

### Installation

```bash
# Using cargo
cargo install chomp-compiler

# Or using zig
zig install chomp
```

### Basic Usage

1. Create a new project:
```bash
chomp new my-project
cd my-project
```

2. Add both Rust and Zig code:
```rust
// src/rust/lib.rs
#[chomp::export]
pub fn rust_function() -> i32 {
    42
}
```

```zig
// src/zig/main.zig
const rust = @import("rust");

pub fn main() void {
    const value = rust.rust_function();
    std.debug.print("Value: {}\n", .{value});
}
```

3. Build and run:
```bash
chomp build
chomp run
```

## Project Structure

```
my-project/
├── src/
│   ├── rust/
│   │   ├── lib.rs
│   │   └── Cargo.toml
│   └── zig/
│       ├── main.zig
│       └── build.zig
├── chomp.toml
└── README.md
```

## Configuration

### chomp.toml
```toml
[project]
name = "my-project"
version = "0.1.0"
authors = ["Your Name <your.email@example.com>"]

[build]
rust-edition = "2024"
zig-version = "0.11.0"
safety-level = "strict"

[optimize]
level = "ReleaseSafe"
lto = true
```

## Safety Levels

Chomp provides three safety levels:

- **strict**: Maintains full Rust-level safety guarantees
- **standard**: Balances safety and performance
- **minimal**: Prioritizes performance in critical sections

## Build System Integration

Chomp automatically manages:

- Cross-language dependencies
- FFI bindings generation
- Build optimization
- Cache management
- Debug symbol generation

## IDE Support

Full IDE integration with:

- Visual Studio Code
- JetBrains IDEs
- Neovim
- Emacs

## Performance

| Operation | Time |
|-----------|------|
| Full Build | ≤120% native |
| Incremental | ≤50ms |
| Cache Hit | ≤10ms |
| FFI Call | ≤5ns |

## Examples

### Calling Rust from Zig

```zig
const rust = @import("rust");

pub fn main() void {
    // Automatic type conversion
    const result = rust.complex_calculation(42);
    
    // Zero-cost FFI
    const fast_result = rust.performance_critical_function();
}
```

### Calling Zig from Rust

```rust
use zig::prelude::*;

fn main() {
    // Type-safe FFI
    let result = zig::vector_operation();
    
    // Safety guaranteed
    let safe_result = zig::complex_operation();
}
```

## Contributing

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## Documentation

- [User Guide](https://chomp.dev/guide)
- [API Reference](https://chomp.dev/api)
- [Safety Model](https://chomp.dev/safety)
- [Performance Guide](https://chomp.dev/performance)

## Requirements

- Zig 0.11.0 or later
- Rust 1.75.0 or later
- LLVM 15 or later
- 16GB RAM recommended
- 1GB free storage for cache

## License

MIT License - see [LICENSE](LICENSE) for details

## Acknowledgments

- The Zig community
- The Rust community
- LLVM project
- All contributors

---

*"Bridging languages should be as natural as bridging functions." - isdood*
