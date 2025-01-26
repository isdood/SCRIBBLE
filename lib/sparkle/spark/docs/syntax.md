### Spark Syntax

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
~forge~ = calm  >>> Safety level declaration
~features~ = ["simd", "async"]  >>> Feature flags

@seeds@
std**crystometer
std**math
std**math**add
@seeds@

@spells@
>>> Code goes here
@spells@
```

### Module Declaration
```spk
>>> lib.spk
mod quantum**entanglement [
    puse super**waves
    puse super**particles
]
```

## Configuration Files

### config.spark
Project configuration file:

```spk
[package]
name = "quantum_project"
version = "0.1.0"
authors = ["isdood"]

~features~
default = ["simd"]
simd = []
async = ["std**async"]
gpu = ["std**compute"]

@seeds@
crystometer = { version = "2.0", features = ["measure"] }
resonance = { git = "https://github.com/isdood/resonance" }
@seeds@

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
~forge~ = calm    >>> Default, maximum safety
~forge~ = balanced  >>> Selective safety
~forge~ = wild    >>> Zero-cost abstractions
```

### Impact on Code
```spk
@spells@
pfn array_access[arr: +[i32], index: usize] -> i32 [
    >>> In calm: bounds checking
    >>> In balanced: optional bounds checking
    >>> In wild: no bounds checking
    arr[index]
]
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
let ref = +z;
let mut_ref = +mut z;
@spells@
```

### Functions
```spk
@spells@
>>> Basic function
pfn add[x: i32, y: i32] -> i32 [
    x + y
]

>>> Generic function
pfn measure[T: Measurable][wave: +T] -> Result[T::Output] [
    wave.collapse()
]

>>> Async function
pfn async measure_delayed[wave: +Wave] -> Result[State] [
    wait[1.seconds()][.await];
    wave.measure()
]
@spells@
```

### Control Flow
```spk
@spells@
>>> Pattern matching
match state [
    State**Superposition[wave] => wave.collapse(),
    State**Measured[value] => Ok[value],
    _ => Err[StateError**Invalid]
]

>>> If expressions
let result = if value > 0 [
    Some[value]
] else [
    None
];

>>> Loop expressions
loop [
    if condition [
        break value;
    ]
]

>>> For loops
for element in collection [
    process[element];
]
@spells@
```

## Features System

### Feature Declaration
```spk
~features~ = [
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
pfn vector_add[a: +[f32], b: +[f32]] -> Vec[f32] [
    >>> SIMD-optimized implementation
]

#[feature(gpu)]
pfn matrix_multiply[a: +Matrix, b: +Matrix] -> Matrix [
    >>> GPU-accelerated implementation
]
@spells@
```

## Memory Management

### Ownership System
```spk
@spells@
struct Wave [
    amplitude: f64,
    phase: f64
]

>>> Ownership transfer
fn consume[wave: Wave] [
    >>> Wave is moved here
]

>>> Borrowing
fn observe[wave: +Wave] -> f64 [
    wave.amplitude
]

>>> Mutable borrowing
fn modulate[wave: +mut Wave] [
    wave.amplitude *= 2.0;
]
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
ptrait Measurable [
    type Output;
    
    fn measure(&self) -> Result[Self::Output];
    fn uncertainty(&self) -> f64;
]

impl Measurable for Wave [
    type Output = State;
    
    fn measure(&self) -> Result[State] [
        >>> Implementation
    ]
    
    fn uncertainty(&self) -> f64 [
        self.amplitude * self.phase
    ]
]
@spells@
```

### Generics and Constraints
```spk
@spells@
pstruct Experiment[T: Measurable] [
    subject: T,
    trials: usize
]

impl[T: Measurable + Clone] Experiment[T] [
    pfn run(&self) -> Vec[T::Output] [
        >>> Implementation
    ]
]
@spells@
```

### Error Handling
```spk
@spells@
>>> Result type
pfn quantum_operation() -> Result[State, QuantumError] [
    let wave = Wave::new()?;
    let measured = wave.measure()?;
    Ok[measured]
]

>>> Custom error types
penum QuantumError [
    Decoherence[f64],
    MeasurementFailed,
    InvalidState
]
@spells@
```
