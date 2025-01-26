#!/bin/bash

# Spark Glitch Module Setup Script
# Author: isdood
# Created: 2025-01-25 19:21:33 UTC
# Repository: isdood/scribble
# Description: Sets up Spark's crystal-optimized error handling system

PURPLE='\033[0;35m'
NC='\033[0m'

print_purple() {
    echo -e "${PURPLE}$1${NC}"
}

setup_glitch_module() {
    cd forge/std || exit 1

    # 1. Create glitch module structure
    mkdir -p src/glitch
    mkdir -p tests/glitch

    # 2. Update lib.rs
    if ! grep -q "pub mod glitch;" src/lib.rs; then
        sed -i '/pub mod array;/a pub mod glitch;' src/lib.rs
        sed -i '/pub use array::CrystalArray;/a pub use glitch::{CrystalError, CrystalResult, Glitch};' src/lib.rs
    fi

    # 3. Create main module file
    cat > src/glitch/mod.rs << 'EOL'
//! Crystal-optimized error handling system.
//!
//! This module provides a high-performance error handling system
//! optimized for crystal-space operations.

use std::error::Error;
use std::fmt;
use std::sync::Arc;
use std::backtrace::Backtrace;

/// A crystal-optimized error type
#[derive(Debug)]
pub struct Glitch {
    kind: GlitchKind,
    message: String,
    source: Option<Arc<dyn Error + Send + Sync>>,
    backtrace: Option<Backtrace>,
}

/// The specific kind of error that occurred
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GlitchKind {
    /// An I/O error occurred
    Io,
    /// A parsing error occurred
    Parse,
    /// A validation error occurred
    Validation,
    /// A configuration error occurred
    Config,
    /// An alignment error occurred
    Alignment,
    /// A memory error occurred
    Memory,
    /// A system error occurred
    System,
    /// An unknown error occurred
    Unknown,
}

impl Glitch {
    /// Creates a new error with the given kind and message
    pub fn new<M: Into<String>>(kind: GlitchKind, message: M) -> Self {
        Self {
            kind,
            message: message.into(),
            source: None,
            backtrace: Some(Backtrace::capture()),
        }
    }

    /// Creates a new I/O error
    pub fn io<M: Into<String>>(message: M) -> Self {
        Self::new(GlitchKind::Io, message)
    }

    /// Creates a new parsing error
    pub fn parse<M: Into<String>>(message: M) -> Self {
        Self::new(GlitchKind::Parse, message)
    }

    /// Creates a new validation error
    pub fn validation<M: Into<String>>(message: M) -> Self {
        Self::new(GlitchKind::Validation, message)
    }

    /// Creates a new configuration error
    pub fn config<M: Into<String>>(message: M) -> Self {
        Self::new(GlitchKind::Config, message)
    }

    /// Creates a new alignment error
    pub fn alignment<M: Into<String>>(message: M) -> Self {
        Self::new(GlitchKind::Alignment, message)
    }

    /// Creates a new memory error
    pub fn memory<M: Into<String>>(message: M) -> Self {
        Self::new(GlitchKind::Memory, message)
    }

    /// Creates a new system error
    pub fn system<M: Into<String>>(message: M) -> Self {
        Self::new(GlitchKind::System, message)
    }

    /// Gets the kind of error
    pub fn kind(&self) -> GlitchKind {
        self.kind
    }

    /// Gets the error message
    pub fn message(&self) -> &str {
        &self.message
    }

    /// Gets the error source
    pub fn source(&self) -> Option<&(dyn Error + Send + Sync)> {
        self.source.as_deref()
    }

    /// Sets the error source
    pub fn with_source<E>(mut self, source: E) -> Self
    where
        E: Error + Send + Sync + 'static,
    {
        self.source = Some(Arc::new(source));
        self
    }

    /// Gets the backtrace
    pub fn backtrace(&self) -> Option<&Backtrace> {
        self.backtrace.as_ref()
    }
}

impl fmt::Display for Glitch {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)?;
        if let Some(source) = self.source() {
            write!(f, ": {}", source)?;
        }
        Ok(())
    }
}

impl Error for Glitch {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.source.as_deref().map(|e| e as &(dyn Error + 'static))
    }
}

/// A type alias for Result with Glitch as the error type
pub type CrystalResult<T> = Result<T, Glitch>;

/// A trait for types that can be converted into a Glitch
pub trait CrystalError: Sized {
    /// Converts self into a Glitch
    fn into_glitch(self) -> Glitch;

    /// Converts self into a CrystalResult
    fn into_crystal_result<T>(self) -> CrystalResult<T> {
        Err(self.into_glitch())
    }
}

impl CrystalError for std::io::Error {
    fn into_glitch(self) -> Glitch {
        Glitch::io(self.to_string()).with_source(self)
    }
}

impl CrystalError for std::fmt::Error {
    fn into_glitch(self) -> Glitch {
        Glitch::parse(self.to_string()).with_source(self)
    }
}

impl CrystalError for std::num::ParseIntError {
    fn into_glitch(self) -> Glitch {
        Glitch::parse(self.to_string()).with_source(self)
    }
}

impl CrystalError for std::str::Utf8Error {
    fn into_glitch(self) -> Glitch {
        Glitch::parse(self.to_string()).with_source(self)
    }
}

/// Extension trait for Result to convert to CrystalResult
pub trait CrystalErrorExt<T, E: CrystalError> {
    /// Converts self into a CrystalResult
    fn into_crystal(self) -> CrystalResult<T>;
}

impl<T, E: CrystalError> CrystalErrorExt<T, E> for Result<T, E> {
    fn into_crystal(self) -> CrystalResult<T> {
        self.map_err(CrystalError::into_glitch)
    }
}
EOL

    # 4. Create tests
    cat > tests/glitch/mod.rs << 'EOL'
use spark_std::glitch::{Glitch, GlitchKind, CrystalError, CrystalErrorExt};
use std::io;

#[test]
fn test_glitch_creation() {
    let err = Glitch::new(GlitchKind::Io, "test error");
    assert_eq!(err.kind(), GlitchKind::Io);
    assert_eq!(err.message(), "test error");
}

#[test]
fn test_glitch_display() {
    let err = Glitch::io("test error");
    assert_eq!(err.to_string(), "test error");
}

#[test]
fn test_glitch_conversion() {
    let io_err = io::Error::new(io::ErrorKind::NotFound, "file not found");
    let err = io_err.into_glitch();
    assert_eq!(err.kind(), GlitchKind::Io);
    assert!(err.source().is_some());
}

#[test]
fn test_result_conversion() {
    let result: Result<(), io::Error> = Err(io::Error::new(io::ErrorKind::NotFound, "not found"));
    let crystal_result = result.into_crystal();
    assert!(crystal_result.is_err());
}

#[test]
fn test_glitch_kinds() {
    let io = Glitch::io("io error");
    let parse = Glitch::parse("parse error");
    let validation = Glitch::validation("validation error");
    let config = Glitch::config("config error");
    let alignment = Glitch::alignment("alignment error");
    let memory = Glitch::memory("memory error");
    let system = Glitch::system("system error");

    assert_eq!(io.kind(), GlitchKind::Io);
    assert_eq!(parse.kind(), GlitchKind::Parse);
    assert_eq!(validation.kind(), GlitchKind::Validation);
    assert_eq!(config.kind(), GlitchKind::Config);
    assert_eq!(alignment.kind(), GlitchKind::Alignment);
    assert_eq!(memory.kind(), GlitchKind::Memory);
    assert_eq!(system.kind(), GlitchKind::System);
}

#[test]
fn test_backtrace() {
    let err = Glitch::io("test error");
    assert!(err.backtrace().is_some());
}
EOL

    print_purple "✓ Created glitch module files"
}

main() {
    print_purple "🔮 Creating Spark Glitch Module..."
    setup_glitch_module
    print_purple "✨ Glitch module created with crystal-space optimization!

Features:
- Crystal-optimized error handling
- Backtraces support
- Error source chains
- Standard error conversions
- Error type categorization
- Result type extensions
- Comprehensive testing

Run 'cargo test' to verify the implementation!"
}

main
