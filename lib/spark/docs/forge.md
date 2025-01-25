# Forge Compiler Documentation
**Version:** 0.3.0 **Author:** isdood **Last 
Updated:** 2025-01-25 15:25:28
## Overview
Forge is the official compiler for the Spark 
programming language, designed to provide 
multi-stage compilation with safety-level 
awareness and advanced optimization capabilities.
## Core Features
### 1. Safety-Level Compilation
```bash
# Compile with different safety levels
forge compile src/main.spk --safety=calm forge 
compile src/main.spk --safety=balanced forge 
compile src/main.spk --safety=wild ```
### 2. Multi-Stage Pipeline
``` Source Code → Lexical Analysis → Parsing → 
AST Generation → Safety Analysis → Type Checking 
→ IR Generation → Optimization → Code Generation 
→ Binary Output ```
## Command Line Interface
### Basic Usage
```bash forge <command> [options] <input> ```
### Available Commands
#### Compilation
```bash
# Basic compilation
forge compile <file> [options] --safety <level> # 
  Safety level (calm/balanced/wild) --opt-level 
  <0-3> # Optimization level --target <triple> # 
  Target architecture --emit <type> # ir, asm, 
  obj, or binary
# Watch mode
forge watch <file> --safety <level> --exec # 
  Execute after compilation
# Multiple files
forge compile-all <dir> --pattern "*.spk" # File 
  pattern --recursive # Include subdirectories
```
#### Analysis Tools
```bash
# Static analysis
forge analyze <file> --safety-check # Verify 
  safety level compliance --unused # Find unused 
  code --memory-model # Analyze memory usage
# IR inspection
forge dump-ir <file> --stage <name> # Pipeline 
  stage --annotate # Add debug annotations
# Safety verification
forge verify <file> --strict # Strict safety 
  checks --report # Generate detailed report
```
## Compiler Stages
### 1. Front End
#### Lexical Analysis
```rust
// Input
let x: i32 = 42;
// Token Stream
[ Token::Let, Token::Identifier("x"), 
    Token::Colon, Token::Type("i32"), 
    Token::Equals, Token::Integer(42), 
    Token::Semicolon
] ```
#### Parsing
```rust
// AST Structure
Program └── VariableDeclaration ├── 
    Identifier("x") ├── Type("i32") └── 
    Value(Integer(42))
```
### 2. Middle End
#### Safety Analysis
```rust
// Safety Level: Calm
#[safety(calm)]
fn array_access(arr: &[i32], idx: usize) -> i32 {
    // Generates bounds checking
    arr[idx]
}
// Safety Level: Wild
#[safety(wild)]
fn unsafe_access(arr: &[i32], idx: usize) -> i32 
{
    // No bounds checking
    unsafe { *arr.as_ptr().add(idx) }
}
```
#### Type Checking
```rust
// Type inference and validation
let x = 42; // Inferred: i32 let y = 3.14; // 
Inferred: f64 let z = x + y; // Error: mismatched 
types ```
#### IR Generation
```llvm
; SPKIR (Spark Intermediate Representation)
define i32 @add(i32 %a, i32 %b) { entry:
  %result = add i32 %a, %b
  ret i32 %result
}
```
### 3. Back End
#### Optimization Passes
```bash
# View optimization passes
forge show-passes --safety <level> # Show 
  safety-level specific passes --target <triple> 
  # Show target-specific passes
``` Common Optimizations: - Dead code elimination 
- Constant folding - Loop unrolling - 
Vectorization - Inlining
#### Code Generation
```bash
# Generate assembly
forge compile example.spk --emit=asm
# Generate object file
forge compile example.spk --emit=obj
# Generate final binary
forge compile example.spk --emit=binary ```
## Advanced Features
### 1. Cross Compilation
```bash
# List available targets
forge targets list
# Cross compile
forge compile src/main.spk \ --target 
  x86_64-unknown-linux-gnu \ --safety balanced \ 
  --opt-level 2
```
### 2. Debug Information
```bash
# Compile with debug info
forge compile src/main.spk --debug
# Generate source maps
forge compile src/main.spk --source-map ```
### 3. Profile-Guided Optimization
```bash
# Generate profiling binary
forge compile --profile-generate
# Run instrumented binary
./my_program
# Use profile data
forge compile --profile-use ```
### 4. Safety Level Features
#### Calm
- Full bounds checking - Null pointer checks - 
Integer overflow protection - Thread safety 
validation - Memory safety guarantees
#### Balanced
- Optional bounds checking - Selective runtime 
checks - Smart pointer optimizations - Controlled 
unsafe blocks - Performance-safety tradeoffs
#### Wild
- No runtime checks - Direct memory access - 
Platform-specific optimizations - Maximum 
performance - Unsafe operations allowed
## Integration Example
### Project Setup
```bash
# Project structure
my_project/ ├── src/ │ └── main.spk ├── Seed.toml 
└── forge.config.toml ```
### Configuration (forge.config.toml)
```toml [compile] safety = "balanced" opt-level = 
2 debug = true [target] triple = 
"x86_64-unknown-linux-gnu" features = ["sse4.2", 
"avx2"] [analysis] safety-checks = true 
memory-model = "strict" [debug] source-map = true 
line-tables = true ```
### Build Script
```bash
#!/bin/bash
# build.sh Clean previous build
forge clean
# Analysis phase
forge analyze src/*.spk --safety-check
# Compilation phase
forge compile src/main.spk \ --safety balanced \ 
  --opt-level 2 \ --target 
  x86_64-unknown-linux-gnu
# Verification
forge verify target/main ```
## Performance Considerations
### Optimization Levels
- **O0**: No optimization (fastest compilation) - 
**O1**: Basic optimizations - **O2**: Aggressive 
optimizations (recommended) - **O3**: Maximum 
optimization (may increase binary size)
### Memory Model
```rust
// Strict memory model (Calm)
#[repr(C)]
struct Aligned { data: [u8; 16]
}
// Relaxed memory model (Wild)
#[repr(packed)]
struct Packed { data: [u8; 16]
}
```
## Development Tools
### Forge Inspector
```bash
# Inspect compilation stages
forge inspect src/main.spk \ --stage parsing \ 
  --output ast.json
# View optimization passes
forge inspect --show-passes
# Memory layout analysis
forge inspect --memory-layout ```
### Debugging
```bash
# Generate debug symbols
forge compile --debug
# Source-level debugging
forge debug ./my_program ``` For more detailed 
information and advanced usage, please refer to: 
- [Forge Compiler 
Guide](https://forge.spark-lang.org/guide) - 
[Optimization 
Manual](https://forge.spark-lang.org/optimization)
- [Safety Level Documentation](https://forge.spark-lang.org/safety)
