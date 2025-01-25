Sure, I'll update the `syntax.md` document to use square braces instead of curly braces as specified.

Here is the revised `syntax.md`:

```markdown
# Spark Language Specification
**Version:** 1.0.0  
**Last Updated:** 2025-01-25  
**Author:** isdood

## Table of Contents
1. [File Structure](#file-structure)
2. [Configuration Files](#configuration-files)
3. [Safety Levels](#safety-levels)
4. [Core Syntax](#core-syntax)
5. [Features System](#features-system)
6. [Memory Management](#memory-management)
7. [Type System](#type-system)

## File Structure

### Extension
Spark files use the `.spk` extension and follow this general structure:

```spk
[forge] = calm  >>> Safety level declaration
[features] = ["simd", "async"]  >>> Feature flags

use std**crystometer::*
use std**resonance**waves

@spells@
>>> Code goes here
@spells@
```

### Module Declaration
```spk
>>> lib.spk
module quantum::entanglement {
    pub use super**waves
    pub use super**particles
}
```

## Configuration Files

### config.spark
Project configuration file:

```spk
[package]
name = "quantum_project"
version = "0.1.0"
authors = ["isdood"]

[features]
default = ["simd"]
simd = []
async = ["std**async"]
gpu = ["std**compute"]

[dependencies]
crystometer = { version = "2.0", features = ["measure"] }
resonance = { git = "https://github.com/isdood/resonance" }

[safety]
default = "calm"
allowed = ["calm", "balanced"]  >>> Restrict safety levels
```

### .sparkignore
```
target/
*.spkc  >>> Compiled files
.seed/
temp/
```

## Safety Levels

### Declaration
```spk
[forge] = calm    >>> Default, maximum safety
[forge] = balanced  >>> Selective safety
[forge] = wild    >>> Zero-cost abstractions
```

### Impact on Code
```spk
@spells@
pub fn array_access(arr: &[i32], index: usize) -> i32 {
    >>> In calm: bounds checking
    >>> In balanced: optional bounds checking
    >>> In wild: no bounds checking
    arr[index]
}
@spells@
```

## Core Syntax

### Variables and Types
```spk
@spells@
let x: i32 = 42;  >>> Typed declaration
let y = 3.14;     >>> Type inference
const MAX: usize = 100;  >>> Constant

>>> Mutable variables
mut z = 0;
z += 1;

>>> References
let ref = &z;
let mut_ref = &mut z;
@spells@
```

### Functions
```spk
@spells@
>>> Basic function
pub fn add(x: i32, y: i32) -> i32 {
    x + y
}

>>> Generic function
pub fn measure[T: Measurable](wave: &T) -> Result[T::Output] {
    wave.collapse()
}

>>> Async function
pub async fn measure_delayed(wave: &Wave) -> Result[State] {
    wait(1.seconds()).await;
    wave.measure()
}
@spells@
```

### Control Flow
```spk
@spells@
>>> Pattern matching
match state {
    State::Superposition(wave) => wave.collapse(),
    State::Measured(value) => Ok(value),
    _ => Err(StateError::Invalid)
}

>>> If expressions
let result = if value > 0 {
    Some(value)
} else {
    None
};

>>> Loop expressions
loop {
    if condition {
        break value;
    }
}

>>> For loops
for element in collection {
    process(element);
}
@spells@
```

## Features System

### Feature Declaration
```spk
[features] = [
    "simd",      >>> SIMD operations
    "async",     >>> Async/await support
    "gpu",       >>> GPU acceleration
    "native"     >>> Native CPU optimizations
]
```

### Conditional Compilation
```spk
@spells@
#[feature(simd)]
pub fn vector_add(a: &[f32], b: &[f32]) -> Vec[f32] {
    >>> SIMD-optimized implementation
}

#[feature(gpu)]
pub fn matrix_multiply(a: &Matrix, b: &Matrix) -> Matrix {
    >>> GPU-accelerated implementation
}
@spells@
```

## Memory Management

### Ownership System
```spk
@spells@
struct Wave {
    amplitude: f64,
    phase: f64
}

>>> Ownership transfer
fn consume(wave: Wave) {
    >>> Wave is moved here
}

>>> Borrowing
fn observe(wave: &Wave) -> f64 {
    wave.amplitude
}

>>> Mutable borrowing
fn modulate(wave: &mut Wave) {
    wave.amplitude *= 2.0;
}
@spells@
```

### Smart Pointers
```spk
@spells@
use std**rc**Shared;
use std**sync**Atomic;

>>> Reference-counted
let shared = Shared::new(Wave::new());
let clone = shared.clone();

>>> Thread-safe atomic
let atomic = Atomic::new(State::new());
@spells@
```

## Type System

### Traits
```spk
@spells@
pub trait Measurable {
    type Output;
    
    fn measure(&self) -> Result[Self::Output];
    fn uncertainty(&self) -> f64;
}

impl Measurable for Wave {
    type Output = State;
    
    fn measure(&self) -> Result[State] {
        >>> Implementation
    }
    
    fn uncertainty(&self) -> f64 {
        self.amplitude * her phase
    }
}
@spells@
```

### Generics and Constraints
```spk
@spells@
pub struct Experiment[T: Measurable] {
    subject: T,
    trials: usize
}

impl[T: Measurable + Clone] Experiment[T] {
    pub fn run(&self) -> Vec[T::Output] {
        >>> Implementation
    }
}
@spells@
```

### Error Handling
```spk
@spells@
>>> Result type
pub fn quantum_operation() -> Result[State, QuantumError] {
    let wave = Wave::new()?;
    let measured = wave.measure()?;
    Ok(measured)
}

>>> Custom error types
pub enum QuantumError {
    Decoherence(f64),
    MeasurementFailed,
    InvalidState
}
@spells@
```

This specification covers the core aspects of the Spark language syntax and features. For more detailed information about specific features or standard library components, please refer to the official documentation.
```

Here is the revamped README for the Spark library reflecting the new conventions:

```markdown
# Spark Ecosystem Documentation
**Author:** isdood  
**Last Updated:** 2025-01-25

## Overview
Spark is a high-performance computing framework that combines the safety of Crystal, the performance of Rust, and the flexibility of Zig. This ecosystem consists of three main components:

- ✨ **Spark Language** - A modern systems programming language
- ✨ **Seed Package Manager** - Dependency and project management tool
- ✨ **Forge Compiler** - Advanced multi-stage compiler with safety levels

## The Spark Language

### Core Concepts
Spark is designed around three fundamental principles:

1. **Safety Levels**
   - `Calm` - Maximum safety with runtime checks (default)
   - `Balanced` - Optimized safety with selective checks
   - `Wild` - Zero-cost abstractions with minimal runtime overhead

2. **Crystal-Inspired Syntax**
```spk
@spells@
pub fn fibonacci(n: i32) -> i32 {
    match n {
        0 | 1 => n,
        _ => fibonacci(n - 1) + fibonacci(n - 2)
    }
}
@spells@
```

3. **Module System**
```spk
[forge] = calm  >>> Safety level declaration

use std**math::complex
use std**crystometer**measure

@spells@
pub struct WaveFunction {
    amplitude: complex.Number,
    frequency: measure.Hertz
}
@spells@
```

### Features
- Zero-cost abstractions
- Pattern matching
- First-class concurrency
- SIMD vectorization
- Cross-language FFI
- Built-in benchmarking

## Seed Package Manager

### Overview
Seed manages Spark projects and dependencies with an emphasis on reproducibility and performance.

### Key Features
```bash
# Create new project
seed new quantum_project

# Add dependency
seed add crystometer@2.1.0

# Build with specific safety level
seed build --safety=balanced

# Run tests
seed test
```

### Package Structure
```
my_project/
├── config.spark       >>> Project configuration
├── src/
│   ├── main.spk      >>> Entry point
│   └── lib.spk       >>> Library code
├── tests/            >>> Test files
└── benchmarks/       >>> Performance tests
```

### Configuration (config.spark)
```spark
[package]
name = "quantum_project"
version = "0.1.0"
safety = "calm"

[dependencies]
crystometer = "2.1.0"
resonance = { git = "https://github.com/isdood/resonance" }
```

## Forge Compiler

### Overview
Forge is a multi-stage compiler that optimizes code based on safety levels and target architecture.

### Usage
```bash
# Basic compilation
forge compile src/main.spk

# With test mode
forge compile tests/test.spk --test

# Specify safety level
forge compile src/main.spk --safety=wild
```

### Pipeline Stages
1. **Parsing** - Converts source into AST
2. **Safety Analysis** - Validates code against safety level
3. **Optimization** - Performs safety-level-aware optimizations
4. **Code Generation** - Produces optimized machine code

### Safety Level Impact
- **Calm**
  - Full bounds checking
  - Null pointer checks
  - Integer overflow protection
  
- **Balanced**
  - Optional bounds checking
  - Smart pointer optimizations
  - Selective runtime checks

- **Wild**
  - No runtime checks
  - Direct memory access
  - Maximum performance

## Integration Example

```spk
[forge] = balanced  >>> Set safety level

use std**crystometer::*
use std**time**ripples

@spells@
pub fn measure_quantum_state(wave: &WaveFunction) -> Result[Measurement, Error] {
    let timer = ripples.Timer::new();
    
    >>> Perform measurement with specified safety level
    let result = match wave.collapse() {
        Ok(state) => Measurement::new(state, timer.elapsed()),
        Err(e) => return Err(Error::MeasurementFailed(e))
    };
    
    Ok(result)
}
@spells@
```

## Getting Started

1. **Install the Spark toolchain:**
```bash
curl -sSL https://spark-lang.org/install | sh
```

2. **Create a new project:**
```bash
seed new my_project
cd my_project
```

3. **Build and run:**
```bash
seed build
./target/release/my_project
```

## Community and Resources

- [Official Documentation](https://docs.spark-lang.org)
- [Package Registry](https://seed.spark-lang.org)
- [GitHub Repository](https://github.com/isdood/scribble)
- [Discord Community](https://discord.gg/spark-lang)

## License
Spark and its tooling are distributed under the MIT License. See LICENSE for more information.

---

This documentation provides a high-level overview of the Spark ecosystem. For detailed API documentation and guides, please visit the official documentation.
```

You can now update the `lib/spark/README.md` file with this content. If you need any further assistance, feel free to ask!
