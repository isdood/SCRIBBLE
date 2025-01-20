# error_derive

A procedural macro for deriving diagnostic capabilities on error types.

## Overview

`error_derive` provides a `#[derive(Diagnose)]` macro that automatically implements diagnostic functionality for error enums. This helps developers provide better error messages, suggestions, and quick fixes for their error types.

## Current Implementation

### Features

- **Error Path Tracking**: Track where errors originate using `#[error_path = "..."]`
- **Diagnostic Information**: Add detection conditions, suggestions, and quick fixes to enum variants
- **Zero-Cost Abstractions**: Compile-time generation of diagnostic implementations
- **Default Behaviors**: Graceful handling of variants without diagnostic attributes

### Usage

```rust
use error_derive::Diagnose;

#[derive(Debug, Diagnose)]
#[error_path = "my/module/path"]
enum MyError {
    #[diagnose(
        detect = "invalid input value",
        suggestion = "Input must be a positive number",
        quick_fix = "ensure_positive(input)"
    )]
    InvalidInput,
    
    // Variants without diagnose attributes work too
    SimpleError,
}
```

### Diagnostic Output

The derived implementation provides:
- Detailed error messages with detection conditions
- Actionable suggestions for fixing the error
- Quick fix code snippets where applicable
- Compile-time error path verification

## Future Plans

### Short Term
- [ ] Add support for conditional diagnostics
- [ ] Implement diagnostic severity levels
- [ ] Add support for error categories
- [ ] Enable custom diagnostic formatters

### Medium Term
- [ ] Integration with IDE tooling
- [ ] Diagnostic aggregation across error types
- [ ] Error pattern analysis
- [ ] Automated fix suggestions

### Long Term
- [ ] Machine learning-based error pattern recognition
- [ ] Interactive error resolution
- [ ] Error prevention recommendations
- [ ] Integration with static analysis tools
