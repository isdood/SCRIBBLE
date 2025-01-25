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
