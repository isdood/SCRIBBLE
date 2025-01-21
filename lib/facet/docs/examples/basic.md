# Basic Facet Examples
> Getting Started with Crystal-Based Calculations
> Last Updated: 2025-01-21 12:46:35 UTC

## Table of Contents
- [Quick Start](#quick-start)
- [Basic Operations](#basic-operations)
- [Simple Crystal Patterns](#simple-crystal-patterns)
- [Basic Error Handling](#basic-error-handling)
- [Common Usage Patterns](#common-usage-patterns)

## Quick Start

### Initialize Calculator
```zig
const std = @import("std");
const Calculator = @import("facet").Calculator;

// Create a new calculator instance
var calc = Calculator.init();
```

### Your First Calculation
```zig
// Simple addition
const result = try calc.compute("2 + 2");
std.debug.print("Result: {d}\n", .{result.value});
// Output: Result: 4.0000
```

## Basic Operations

### Arithmetic Operations
```zig
// Addition
const sum = try calc.add(5, 3);
// Result: 8.0000 (Coherence: 1.0000)

// Subtraction
const difference = try calc.subtract(10, 4);
// Result: 6.0000 (Coherence: 1.0000)

// Multiplication
const product = try calc.multiply(6, 7);
// Result: 42.0000 (Coherence: 1.0000)

// Division
const quotient = try calc.divide(15, 3);
// Result: 5.0000 (Coherence: 1.0000)
```

### String Expressions
```zig
// Calculate string expressions
const expressions = [_][]const u8{
    "1 + 2",
    "10 - 5",
    "4 * 3",
    "8 / 2",
};

for (expressions) |expr| {
    const result = try calc.compute(expr);
    std.debug.print("{s} = {d}\n", .{expr, result.value});
}
```

## Simple Crystal Patterns

### Basic Crystal Structure
```zig
// Initialize with default crystal pattern
var calc = Calculator.initWithCrystal();

// Perform calculation with crystal alignment
const result = try calc.computeAligned("5 + 5");
std.debug.print("Result: {d} (Crystal Clarity: {d})\n", 
    .{result.value, result.clarity});
```

### Simple Reality Anchor
```zig
// Initialize with basic reality anchor
var calc = Calculator.initWithAnchor();

// Compute with reality checking
const result = try calc.computeAnchored("3 * 4");
std.debug.print("Result: {d} (Anchor Strength: {d})\n", 
    .{result.value, result.anchor});
```

## Basic Error Handling

### Simple Error Checks
```zig
// Division by zero protection
const result = calc.compute("10 / 0") catch |err| {
    switch (err) {
        error.DivisionByZero => {
            std.debug.print("Cannot divide by zero!\n", .{});
            return;
        },
        else => return err,
    }
};
```

### Coherence Checking
```zig
// Check calculation coherence
const result = try calc.compute("5 + 5");
if (result.coherence < 0.87) {
    std.debug.print("Warning: Low coherence detected!\n", .{});
}
```

## Common Usage Patterns

### Chain Operations
```zig
// Multiple operations in sequence
var value = try calc.compute("2 + 2");
value = try calc.multiply(value.value, 2);
value = try calc.subtract(value.value, 1);

std.debug.print("Final result: {d}\n", .{value.value});
// Output: Final result: 7.0000
```

### Basic Formatting
```zig
const result = try calc.compute("10 / 3");

// Print with different formats
std.debug.print("Default: {d}\n", .{result.value});
std.debug.print("Fixed: {d:.2}\n", .{result.value});
std.debug.print("Scientific: {e}\n", .{result.value});
```

### Memory Operations
```zig
// Store and recall values
try calc.store("x", 5);
try calc.store("y", 3);

const result = try calc.compute("x + y");
std.debug.print("x + y = {d}\n", .{result.value});
```

## Simple Integration Examples

### Rust Integration
```rust
use facet::Calculator;

// Create calculator instance
let mut calc = Calculator::new();

// Perform simple calculation
let result = calc.compute("5 + 5")?;
println!("Result: {}", result.value);
```

### Command Line Usage
```bash
# Basic calculation
facet "2 + 2"

# Multiple calculations
facet "5 + 5" "10 * 2" "15 / 3"

# Save result to file
facet "100 * 1.5" > result.txt
```

## Tips for Beginners

1. **Always Check Results**
```zig
const result = try calc.compute("5 / 2");
if (!result.isValid()) {
    std.debug.print("Calculation error!\n", .{});
    return;
}
```

2. **Use Clear Expressions**
```zig
// Good
const result = try calc.compute("2 + (3 * 4)");

// Avoid
const result = try calc.compute("2+3*4");  // Less readable
```

3. **Handle Errors Properly**
```zig
const result = calc.compute(user_input) catch |err| {
    std.debug.print("Error: {}\n", .{err});
    return;
};
```

## Common Mistakes to Avoid

### 1. Forgetting Error Handling
```zig
// Wrong
const result = calc.compute("5 / 0");  // Will crash!

// Right
const result = try calc.compute("5 / 0");
```

### 2. Ignoring Coherence
```zig
// Wrong
_ = calc.compute("expression");  // Coherence not checked

// Right
const result = try calc.compute("expression");
if (result.coherence < 0.87) {
    // Handle low coherence
}
```

### 3. Not Initializing Properly
```zig
// Wrong
var calc: Calculator = undefined;  // Uninitialized!

// Right
var calc = Calculator.init();
```

## Next Steps

Once you're comfortable with these basics, check out:
- [Advanced Examples](advanced.md) for more complex operations
- [API Documentation](../API.md) for detailed reference
- [Architecture Guide](../ARCHITECTURE.md) for system understanding

---

*Generated for Facet v0.1.0*  
*Author: @isdood*  
*Last Updated: 2025-01-21 12:46:35 UTC*
