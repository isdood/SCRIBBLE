#!/bin/bash

# Spark Standard Library Setup Script
# Author: isdood
# Created: 2025-01-25 17:40:00 UTC
# Repository: isdood/scribble

PURPLE='\033[0;35m'
NC='\033[0m'

print_purple() {
    echo -e "${PURPLE}$1${NC}"
}

create_directory_structure() {
    print_purple "📚 Creating Spark Standard Library structure..."
    mkdir -p forge/std/src/math
    mkdir -p forge/std/tests
}

setup_std_library() {
    # Add std to workspace
    if ! grep -q "std" forge/Cargo.toml; then
        sed -i '/members = \[/a \ \ \ \ "std",' forge/Cargo.toml
    fi

    cat > forge/std/Cargo.toml << 'EOL'
[package]
name = "spark-std"
version.workspace = true
edition.workspace = true
authors.workspace = true

[dependencies]
thiserror.workspace = true
serde.workspace = true
serde_json.workspace = true

[lib]
name = "spark_std"
path = "src/lib.rs"
EOL

    cat > forge/std/src/lib.rs << 'EOL'
//! Spark Standard Library - Where Magic Begins ✨

pub mod math;
pub mod types;

pub use types::*;
pub use math::operations;
EOL

    cat > forge/std/src/types.rs << 'EOL'
//! Spark's Magical Type System

/// Whisper - A tiny number (8-bit integer)
pub type Whisper = i8;

/// Murmur - A small number (16-bit integer)
pub type Murmur = i16;

/// Voice - A regular number (32-bit integer)
pub type Voice = i32;

/// Shout - A big number (64-bit integer)
pub type Shout = i64;

/// Echo - A floating number (32-bit float)
pub type Echo = f32;

/// Thunder - A precise floating number (64-bit float)
pub type Thunder = f64;

/// Scroll - A string of characters
pub type Scroll = String;

/// Rune - A single character
pub type Rune = char;

/// Essence - A basic true/false value
pub type Essence = bool;

/// Void - Represents nothing (unit type)
pub type Void = ();

/// Size of magical containers
pub type Magnitude = usize;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MagicKind {
    Whisper,
    Murmur,
    Voice,
    Shout,
    Echo,
    Thunder,
    Scroll,
    Rune,
    Essence,
    Void,
}

impl std::fmt::Display for MagicKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MagicKind::Whisper => write!(f, "Whisper"),
            MagicKind::Murmur => write!(f, "Murmur"),
            MagicKind::Voice => write!(f, "Voice"),
            MagicKind::Shout => write!(f, "Shout"),
            MagicKind::Echo => write!(f, "Echo"),
            MagicKind::Thunder => write!(f, "Thunder"),
            MagicKind::Scroll => write!(f, "Scroll"),
            MagicKind::Rune => write!(f, "Rune"),
            MagicKind::Essence => write!(f, "Essence"),
            MagicKind::Void => write!(f, "Void"),
        }
    }
}
EOL

    cat > forge/std/src/math/mod.rs << 'EOL'
//! Magical Mathematics

mod add;
mod sub;

pub mod operations {
    pub use super::add::enchant as add;
    pub use super::sub::enchant as sub;
}
EOL

    cat > forge/std/src/math/add.rs << 'EOL'
//! Addition Enchantments

use crate::types::*;

/// Specialized addition for each magical type
pub mod enchant {
    use super::*;

    #[inline]
    pub fn whispers(a: Whisper, b: Whisper) -> Whisper {
        a + b
    }

    #[inline]
    pub fn murmurs(a: Murmur, b: Murmur) -> Murmur {
        a + b
    }

    #[inline]
    pub fn voices(a: Voice, b: Voice) -> Voice {
        a + b
    }

    #[inline]
    pub fn shouts(a: Shout, b: Shout) -> Shout {
        a + b
    }

    #[inline]
    pub fn echoes(a: Echo, b: Echo) -> Echo {
        a + b
    }

    #[inline]
    pub fn thunders(a: Thunder, b: Thunder) -> Thunder {
        a + b
    }
}
EOL

    cat > forge/std/src/math/sub.rs << 'EOL'
//! Subtraction Enchantments

use crate::types::*;

/// Specialized subtraction for each magical type
pub mod enchant {
    use super::*;

    #[inline]
    pub fn whispers(a: Whisper, b: Whisper) -> Whisper {
        a - b
    }

    #[inline]
    pub fn murmurs(a: Murmur, b: Murmur) -> Murmur {
        a - b
    }

    #[inline]
    pub fn voices(a: Voice, b: Voice) -> Voice {
        a - b
    }

    #[inline]
    pub fn shouts(a: Shout, b: Shout) -> Shout {
        a - b
    }

    #[inline]
    pub fn echoes(a: Echo, b: Echo) -> Echo {
        a - b
    }

    #[inline]
    pub fn thunders(a: Thunder, b: Thunder) -> Thunder {
        a - b
    }
}
EOL

    cat > forge/std/tests/primitive_tests.rs << 'EOL'
use spark_std::{
    types::*,
    math::operations::{add, sub},
};

#[test]
fn test_primitive_types() {
    let whisper: Whisper = 42;
    let murmur: Murmur = 966;  // 966 / 23 = 42
    let voice: Voice = 42;
    let _shout: Shout = 1_000_000;
    let _echo: Echo = 3.14159;
    let _thunder: Thunder = 3.14159265359;
    let _scroll: Scroll = "Magic!".to_string();
    let rune: Rune = '✨';
    let essence: Essence = true;
    let void: Void = ();

    // Test integer operations
    assert_eq!(whisper as i16, murmur / 23);
    assert_eq!(voice, 42); // The answer to everything

    // Other type tests
    assert_eq!(rune.to_string(), "✨");
    assert_eq!(essence, !false);
    assert_eq!(void, ());
}

#[test]
fn test_math_operations() {
    // Test addition
    assert_eq!(add::whispers(40, 2), 42);
    assert_eq!(add::echoes(3.0, 0.14), 3.14);

    // Test subtraction
    assert_eq!(sub::voices(100, 58), 42);
    assert_eq!(sub::thunders(10.5, 7.5), 3.0);

    // Test floating point operations with exact values
    let result = add::echoes(3.14159, 0.0);
    assert!((result - 3.14159).abs() < 0.000001);
}
EOL

    print_purple "✓ Created standard library files"
}

main() {
    print_purple "📚 Creating Spark Standard Library..."
    create_directory_structure
    setup_std_library
    print_purple "✨ Standard library created with magical primitive types!

Primitive Types:
- Whisper (i8)  - For tiny numbers
- Murmur (i16)  - For small numbers
- Voice (i32)   - For regular numbers
- Shout (i64)   - For big numbers
- Echo (f32)    - For floating numbers
- Thunder (f64) - For precise floating numbers
- Scroll        - For text
- Rune         - For single characters
- Essence      - For true/false values
- Void         - For nothing

Math Operations:
- std**math**add
- std**math**sub

Run 'cd forge/std && cargo test' to verify the enchantments!"
}

main
