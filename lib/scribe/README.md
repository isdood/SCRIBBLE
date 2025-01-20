# ✒️ Scribe
## Native Formatting and Display System

```ascii
 Input       Scribe       Output
  │            │            │
  ▼            ▼            ▼
[Data] ───> [Format] ──> [String]
  │            │            │
  └─> Native Processing <──┘
```

Scribe provides a lightweight yet powerful native formatting and string handling system for the Scribble framework, with custom string implementation optimized for crystal computing architecture.

## ✨ Features

### Native String Implementation
```rust
pub struct String {
    inner: std::vec::Vec<u8>,
}
```

### Core Scribe Trait
```rust
pub trait Scribe {
    fn scribe(&self) -> String;
    fn type_name(&self) -> &'static str;
    fn scribe_debug(&self) -> String;
}
```

## 🚀 Quick Start

```rust
use scribe::{Scribe, String};

// Create a custom type
struct Crystal {
    name: String,
    facets: u32,
}

// Implement Scribe for your type
impl Scribe for Crystal {
    fn scribe(&self) -> String {
        let mut result = String::new();
        result.push_str(&self.name.to_str());
        result.push_str(": ");
        result.push_str(&self.facets.to_string());
        result
    }
}

// Use the formatting utilities
fn main() {
    let crystal = Crystal {
        name: String::from("Amethyst"),
        facets: 12,
    };
    
    println!("{}", crystal.scribe().to_str());
    println!("{}", crystal.scribe_debug().to_str());
}
```

## 🎯 Core Components

### 1. Native String (`native_string.rs`)
- Custom string implementation
- UTF-8 validation
- Efficient byte storage
- String conversion utilities

### 2. Formatting Utilities (`utils`)
```rust
// Join strings with separator
utils::join(&strings, ", ");

// Format with decorators
utils::bracketed("content");    // [content]
utils::parenthesized("content"); // (content)
utils::braced("content");       // {content}
utils::quoted("content");       // "content"
```

### 3. Result Extensions
```rust
pub type ScribeResult<T, E> = Result<T, E>;

// Convert results to strings
let result: ScribeResult<T, E> = ...;
let string = result.to_scribe_string();
```

## 💫 String Operations

### Creating Strings
```rust
// Create empty string
let mut string = String::new();

// Create from &str
let string = String::from("Hello");

// Push string content
string.push_str(" World");
```

### Converting Strings
```rust
// Get string slice
let slice = string.as_str();

// Get string representation
let str_rep = string.to_str();
```

## ⚡ Performance Characteristics

### String Operations
- Creation: O(1)
- Push: O(n) amortized
- Conversion: O(1)
- UTF-8 Validation: O(n)

### Formatting Operations
- Join: O(n) where n is total length
- Wrap: O(n + k) where k is wrapper length
- Format Multiple: O(n * m) where m is item count

## 🛠️ Requirements

### System Requirements
- Rust 1.75+
- UTF-8 text support
- Byte-level memory access

### Dependencies
```toml
[dependencies]
# Core functionality is no-std compatible
```

## 🔬 String Implementation

### Memory Layout
```
┌──────────────────────────┐
│     String Structure     │
├──────────────────────────┤
│ Vec<u8> for UTF-8 bytes │
└──────────────────────────┘
```

### UTF-8 Handling
- Automatic validation
- Safe conversion
- Efficient storage

## 📈 Utility Functions

### Text Decoration
```rust
let text = "crystal";
utils::bracketed(text);     // [crystal]
utils::parenthesized(text); // (crystal)
utils::braced(text);       // {crystal}
utils::quoted(text);       // "crystal"
```

### Multiple Item Formatting
```rust
let items = vec![item1, item2, item3];
utils::format_multiple(&items, ", ");
```

## 🤝 Contributing

1. Maintain UTF-8 compliance
2. Optimize memory usage
3. Add tests for new features
4. Update documentation
5. Follow no_std guidelines when possible

## 📊 Current Status
- Version: 0.1.0
- Last Updated: 2025-01-19 18:44:18 UTC
- Implementation: Rust
- Author: Caleb J.D. Terkovics (isdood)

## 📜 License
MIT - See LICENSE for details

---

*"Every crystal tells a story, every string holds its truth."* - isdood
