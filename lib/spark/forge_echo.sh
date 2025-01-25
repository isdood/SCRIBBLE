#!/bin/bash

# Spark Echo Module Setup Script
# Author: isdood
# Created: 2025-01-25 19:26:13 UTC
# Repository: isdood/scribble
# Description: Sets up Spark's crystal-optimized formatted string system

PURPLE='\033[0;35m'
NC='\033[0m'

print_purple() {
    echo -e "${PURPLE}$1${NC}"
}

setup_echo_module() {
    cd forge/std || exit 1

    # 1. Create echo module structure
    mkdir -p src/echo
    mkdir -p tests/echo

    # 2. Update lib.rs
    if ! grep -q "pub mod echo;" src/lib.rs; then
        sed -i '/pub mod array;/a pub mod echo;' src/lib.rs
        sed -i '/pub use array::CrystalArray;/a pub use echo::{CrystalEcho, echo, EchoFmt};' src/lib.rs
    fi

    # 3. Create main module file
    cat > src/echo/mod.rs << 'EOL'
//! Crystal-optimized formatted string system.
//!
//! This module provides a high-performance string formatting system
//! optimized for crystal-space operations.

use std::fmt;
use std::borrow::Cow;
use std::sync::Arc;

/// A crystal-optimized formatted string
#[derive(Clone)]
pub struct CrystalEcho {
    content: Arc<EchoContent>,
}

/// The internal content of a formatted string
#[derive(Clone)]
enum EchoContent {
    Static(&'static str),
    Owned(String),
    Formatted(Box<dyn EchoFmt>),
}

/// A trait for types that can be formatted in crystal-space
pub trait EchoFmt: fmt::Display + Send + Sync + 'static {
    /// Returns the formatted string
    fn format(&self) -> Cow<'_, str>;

    /// Returns true if the formatted string is static
    fn is_static(&self) -> bool {
        false
    }

    /// Returns the approximate capacity needed
    fn capacity_hint(&self) -> usize {
        0
    }
}

impl CrystalEcho {
    /// Creates a new formatted string from a static str
    pub const fn new(s: &'static str) -> Self {
        Self {
            content: Arc::new(EchoContent::Static(s)),
        }
    }

    /// Creates a new formatted string from an owned String
    pub fn owned(s: String) -> Self {
        Self {
            content: Arc::new(EchoContent::Owned(s)),
        }
    }

    /// Creates a new formatted string from a formattable value
    pub fn fmt<T: EchoFmt>(value: T) -> Self {
        Self {
            content: Arc::new(EchoContent::Formatted(Box::new(value))),
        }
    }

    /// Returns the string contents
    pub fn as_str(&self) -> Cow<'_, str> {
        match &*self.content {
            EchoContent::Static(s) => Cow::Borrowed(*s),
            EchoContent::Owned(s) => Cow::Borrowed(s),
            EchoContent::Formatted(f) => f.format(),
        }
    }

    /// Returns true if the string is static
    pub fn is_static(&self) -> bool {
        matches!(&*self.content, EchoContent::Static(_))
    }

    /// Returns the capacity hint
    pub fn capacity_hint(&self) -> usize {
        match &*self.content {
            EchoContent::Static(s) => s.len(),
            EchoContent::Owned(s) => s.capacity(),
            EchoContent::Formatted(f) => f.capacity_hint(),
        }
    }
}

impl fmt::Display for CrystalEcho {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &*self.content {
            EchoContent::Static(s) => f.write_str(s),
            EchoContent::Owned(s) => f.write_str(s),
            EchoContent::Formatted(fmt) => fmt::Display::fmt(fmt, f),
        }
    }
}

impl fmt::Debug for CrystalEcho {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("CrystalEcho")
            .field("content", &self.as_str())
            .finish()
    }
}

/// Creates a new formatted string
#[macro_export]
macro_rules! echo {
    ($s:literal) => {
        $crate::echo::CrystalEcho::new($s)
    };
    ($($arg:tt)*) => {
        $crate::echo::CrystalEcho::owned(format!($($arg)*))
    };
}

// Implement EchoFmt for common types
impl EchoFmt for String {
    fn format(&self) -> Cow<'_, str> {
        Cow::Borrowed(self)
    }

    fn capacity_hint(&self) -> usize {
        self.capacity()
    }
}

impl EchoFmt for &'static str {
    fn format(&self) -> Cow<'_, str> {
        Cow::Borrowed(self)
    }

    fn is_static(&self) -> bool {
        true
    }

    fn capacity_hint(&self) -> usize {
        self.len()
    }
}

impl EchoFmt for char {
    fn format(&self) -> Cow<'_, str> {
        Cow::Owned(self.to_string())
    }

    fn capacity_hint(&self) -> usize {
        4 // Maximum bytes for a UTF-8 char
    }
}

// Implement for numeric types
macro_rules! impl_echo_fmt_for_num {
    ($($t:ty),*) => {
        $(
            impl EchoFmt for $t {
                fn format(&self) -> Cow<'_, str> {
                    Cow::Owned(self.to_string())
                }

                fn capacity_hint(&self) -> usize {
                    20 // Good default for most numeric types
                }
            }
        )*
    }
}

impl_echo_fmt_for_num!(i8, i16, i32, i64, i128, isize,
                       u8, u16, u32, u64, u128, usize,
                       f32, f64);

// Custom numeric formatters
pub struct Hex<T>(pub T);
pub struct Binary<T>(pub T);
pub struct Octal<T>(pub T);

macro_rules! impl_echo_fmt_for_numeric_fmt {
    ($wrapper:ident, $fmt:expr, $($t:ty),*) => {
        $(
            impl EchoFmt for $wrapper<$t> {
                fn format(&self) -> Cow<'_, str> {
                    Cow::Owned(format!($fmt, self.0))
                }

                fn capacity_hint(&self) -> usize {
                    std::mem::size_of::<$t>() * 2 + 2
                }
            }
        )*
    }
}

impl_echo_fmt_for_numeric_fmt!(Hex, "{:#x}", i8, i16, i32, i64, i128, isize,
                                           u8, u16, u32, u64, u128, usize);
impl_echo_fmt_for_numeric_fmt!(Binary, "{:#b}", i8, i16, i32, i64, i128, isize,
                                              u8, u16, u32, u64, u128, usize);
impl_echo_fmt_for_numeric_fmt!(Octal, "{:#o}", i8, i16, i32, i64, i128, isize,
                                             u8, u16, u32, u64, u128, usize);
EOL

    # 4. Create tests
    cat > tests/echo/mod.rs << 'EOL'
use spark_std::echo::{CrystalEcho, EchoFmt, Hex, Binary, Octal};
use std::borrow::Cow;

#[test]
fn test_static_echo() {
    let e = CrystalEcho::new("test");
    assert_eq!(e.as_str(), "test");
    assert!(e.is_static());
}

#[test]
fn test_owned_echo() {
    let e = CrystalEcho::owned(String::from("test"));
    assert_eq!(e.as_str(), "test");
    assert!(!e.is_static());
}

#[test]
fn test_formatted_echo() {
    let e = CrystalEcho::fmt(42);
    assert_eq!(e.as_str(), "42");
}

#[test]
fn test_echo_macro() {
    let x = 42;
    let e = echo!("value: {}", x);
    assert_eq!(e.as_str(), "value: 42");
}

#[test]
fn test_numeric_formats() {
    let n = 42u32;
    assert_eq!(CrystalEcho::fmt(Hex(n)).as_str(), "0x2a");
    assert_eq!(CrystalEcho::fmt(Binary(n)).as_str(), "0b101010");
    assert_eq!(CrystalEcho::fmt(Octal(n)).as_str(), "0o52");
}

#[test]
fn test_capacity_hints() {
    let s = "test";
    assert_eq!(CrystalEcho::new(s).capacity_hint(), 4);

    let n = 42i32;
    let e = CrystalEcho::fmt(n);
    assert!(e.capacity_hint() >= 2);
}

#[test]
fn test_custom_echo_fmt() {
    struct Custom(i32);

    impl EchoFmt for Custom {
        fn format(&self) -> Cow<'_, str> {
            Cow::Owned(format!("Custom({})", self.0))
        }
    }

    let c = Custom(42);
    let e = CrystalEcho::fmt(c);
    assert_eq!(e.as_str(), "Custom(42)");
}

#[test]
fn test_char_echo() {
    let c = 'x';
    let e = CrystalEcho::fmt(c);
    assert_eq!(e.as_str(), "x");
}
EOL

    print_purple "✓ Created echo module files"
}

main() {
    print_purple "🔮 Creating Spark Echo Module..."
    setup_echo_module
    print_purple "✨ Echo module created with crystal-space optimization!

Features:
- Crystal-optimized string formatting
- Static string optimization
- Custom numeric formatters
- Capacity hints
- Arc-based sharing
- Lazy formatting
- Comprehensive testing

Run 'cargo test' to verify the implementation!"
}

main
