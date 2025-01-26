#!/bin/bash

# Spark Echo Fix Script (Part 4)
# Author: isdood
# Created: 2025-01-25 19:31:23 UTC
# Repository: isdood/scribble
# Description: Fixes echo module macro naming conflicts

PURPLE='\033[0;35m'
NC='\033[0m'

print_purple() {
    echo -e "${PURPLE}$1${NC}"
}

fix_echo_module() {
    cd forge/std || exit 1

    # 1. Update lib.rs with correct exports
    cat > src/lib.rs << 'EOL'
pub mod align;
pub mod any;
pub mod array;
pub mod conv;
pub mod def;
pub mod echo;
pub mod shard;

pub use align::Alignment;
pub use array::CrystalArray;
pub use conv::{CrystalFrom, CrystalInto, CrystalTryFrom, CrystalTryInto};
pub use def::{CrystalDefault, CrystalInit};
pub use echo::{CrystalEcho, EchoFmt};
EOL

    # 2. Update echo module
    cat > src/echo/mod.rs << 'EOL'
//! Crystal-optimized formatted string system.
//!
//! This module provides a high-performance string formatting system
//! optimized for crystal-space operations.

use std::fmt;
use std::borrow::Cow;
use std::sync::Arc;

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
    Formatted(String),
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
    pub fn new(s: &'static str) -> Self {
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
    pub fn fmt<T: fmt::Display>(value: T) -> Self {
        Self {
            content: Arc::new(EchoContent::Formatted(value.to_string())),
        }
    }

    /// Returns the string contents
    pub fn as_str(&self) -> Cow<'_, str> {
        match &*self.content {
            EchoContent::Static(s) => Cow::Borrowed(*s),
            EchoContent::Owned(s) => Cow::Borrowed(s),
            EchoContent::Formatted(s) => Cow::Borrowed(s),
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
            EchoContent::Formatted(s) => s.capacity(),
        }
    }
}

impl fmt::Display for CrystalEcho {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &*self.content {
            EchoContent::Static(s) => f.write_str(s),
            EchoContent::Owned(s) => f.write_str(s),
            EchoContent::Formatted(s) => f.write_str(s),
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

// Implement numeric formatters
#[derive(Debug, Clone, Copy)]
pub struct Hex<T>(pub T);

#[derive(Debug, Clone, Copy)]
pub struct Binary<T>(pub T);

#[derive(Debug, Clone, Copy)]
pub struct Octal<T>(pub T);

macro_rules! impl_display_for_wrapper {
    ($wrapper:ident, $fmt:expr, $($t:ty),*) => {
        $(
            impl fmt::Display for $wrapper<$t> {
                fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                    write!(f, $fmt, self.0)
                }
            }

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

// Implement for unsigned integers
impl_display_for_wrapper!(Hex, "{:#x}", u8, u16, u32, u64, u128, usize);
impl_display_for_wrapper!(Binary, "{:#b}", u8, u16, u32, u64, u128, usize);
impl_display_for_wrapper!(Octal, "{:#o}", u8, u16, u32, u64, u128, usize);

// Implement for signed integers
impl_display_for_wrapper!(Hex, "{:#x}", i8, i16, i32, i64, i128, isize);
impl_display_for_wrapper!(Binary, "{:#b}", i8, i16, i32, i64, i128, isize);
impl_display_for_wrapper!(Octal, "{:#o}", i8, i16, i32, i64, i128, isize);

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
EOL

    print_purple "✓ Fixed echo module implementation"
}

main() {
    print_purple "🔮 Fixing Spark Echo Module..."
    fix_echo_module
    print_purple "✨ Echo module fixes applied!

Fixed Issues:
- Fixed macro naming conflicts
- Improved module organization
- Fixed macro exports
- Enhanced error handling
- Fixed documentation
- Improved type safety
- Fixed public interface

Run 'cargo test' to verify the fixes!"
}

main
