# error_core

Core traits and types for advanced error handling and diagnostics.

## Overview

`error_core` provides the fundamental building blocks for creating rich, diagnostic-enabled error types. It works in conjunction with `error_derive` to create a comprehensive error handling system.

## Current Implementation (as of 2025-01-20)

### Core Types

```rust
pub struct DiagnosticReport {
    pub message: String,
    pub suggestions: Vec<String>,
    pub quick_fixes: Vec<QuickFix>,
}

pub struct QuickFix {
    pub description: String,
    pub code: String,
}

pub struct CompileTimeError {
    pub message: String,
    pub location: String,
}
```

### Traits

```rust
pub trait Diagnose {
    fn diagnose(&self) -> DiagnosticReport;
    fn get_quick_fixes(&self) -> Vec<QuickFix>;
    fn check_at_compile_time() -> Option<CompileTimeError>;
}
```

### Features

- **Diagnostic Reports**: Structured error information with messages and suggestions
- **Quick Fixes**: Code-level suggestions for error resolution
- **Compile-Time Checks**: Static verification of error configurations
- **Zero-Overhead Abstractions**: Efficient error handling without runtime cost

## Usage

```rust
use error_core::{Diagnose, DiagnosticReport, QuickFix};

// Typically used through error_derive
#[derive(Debug, error_derive::Diagnose)]
enum MyError {
    #[diagnose(
        detect = "invalid state",
        suggestion = "Initialize before use",
        quick_fix = "init()"
    )]
    UninitializedState,
}

// Can also be implemented manually
impl Diagnose for MyCustomError {
    fn diagnose(&self) -> DiagnosticReport {
        DiagnosticReport {
            message: "Custom error occurred".to_string(),
            suggestions: vec!["Try this instead".to_string()],
            quick_fixes: vec![
                QuickFix {
                    description: "Initialize system".to_string(),
                    code: "system.init()".to_string(),
                }
            ],
        }
    }
    
    // ... implement other trait methods
}
```

## Future Plans

### Short Term (Q1-Q2 2025)
- [ ] Add diagnostic severity levels (Error, Warning, Info)
- [ ] Support error categorization
- [ ] Add error context tracking
- [ ] Implement diagnostic metadata

### Medium Term (Q3-Q4 2025)
- [ ] Error chain analysis
- [ ] Performance metrics for error handling
- [ ] Async diagnostic support
- [ ] Integration with logging systems

### Long Term (2026+)
- [ ] Error pattern analysis framework
- [ ] Diagnostic telemetry
- [ ] Error handling optimization suggestions
- [ ] Multi-language diagnostic support

## Integration

Works seamlessly with:
- `error_derive` for automatic trait implementation
- Standard Rust error handling
- Custom error types
- Testing frameworks

## Contributing

Contributions welcome! Please check our contribution guidelines.

## Development Status

- Maintainer: @isdood
- Status: Active Development
- Version: 0.1.0
- Rust Version: 2021 edition
