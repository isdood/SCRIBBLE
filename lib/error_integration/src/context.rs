//! Error context module providing location and timing information
//! Created: 2025-01-20 22:12:40
//! Author: isdood

use chrono::{DateTime, Utc};
use std::fmt;

/// Stores contextual information about where and when an error occurred
#[derive(Debug, Clone)]
pub struct ErrorContext {
    /// When the error occurred
    pub timestamp: DateTime<Utc>,

    /// Source file where the error originated
    pub file: String,

    /// Line number in source file
    pub line: u32,

    /// Column number in source file
    pub column: u32,

    /// Optional module path
    pub module_path: Option<String>,

    /// Optional additional context
    pub extra: Option<String>,
}

impl ErrorContext {
    /// Creates a new ErrorContext with the current time and location information
    #[track_caller]
    pub fn new(file: impl Into<String>, line: u32, column: u32) -> Self {
        Self {
            timestamp: Utc::now(),
            file: file.into(),
            line,
            column,
            module_path: Some(std::module_path!().to_string()),
            extra: None,
        }
    }

    /// Creates a new ErrorContext from the current location
    #[track_caller]
    pub fn current() -> Self {
        let location = std::panic::Location::caller();
        Self::new(
            location.file(),
                  location.line(),
                  location.column(),
        )
    }

    /// Returns the elapsed time since the error occurred
    pub fn elapsed(&self) -> chrono::Duration {
        Utc::now() - self.timestamp
    }

    /// Adds additional context information
    pub fn with_extra(mut self, extra: impl Into<String>) -> Self {
        self.extra = Some(extra.into());
        self
    }

    /// Returns true if this context contains additional information
    pub fn has_extra(&self) -> bool {
        self.extra.is_some()
    }

    /// Returns the module path if available
    pub fn module_path(&self) -> Option<&str> {
        self.module_path.as_deref()
    }
}

impl fmt::Display for ErrorContext {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "at {}:{}:{} ({})",
               self.file,
               self.line,
               self.column,
               self.timestamp.to_rfc3339()
        )?;

        if let Some(module) = &self.module_path {
            write!(f, " in module {}", module)?;
        }

        if let Some(extra) = &self.extra {
            write!(f, " - additional context: {}", extra)?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_context_creation() {
        let context = ErrorContext::current();
        assert!(context.file.contains("context.rs"));
        assert!(context.line > 0);
        assert!(context.column > 0);
        assert!(context.module_path.is_some());
    }

    #[test]
    fn test_context_with_extra() {
        let context = ErrorContext::current()
        .with_extra("test message");
        assert!(context.has_extra());
        assert_eq!(context.extra.as_deref(), Some("test message"));
    }

    #[test]
    fn test_context_display() {
        let context = ErrorContext::new("test.rs", 10, 5)
        .with_extra("additional info");
        let display = context.to_string();
        assert!(display.contains("test.rs:10:5"));
        assert!(display.contains("additional info"));
        assert!(display.contains(&Utc::now().format("%Y").to_string()));
    }

    #[test]
    fn test_context_elapsed() {
        let context = ErrorContext::current();
        std::thread::sleep(std::time::Duration::from_millis(10));
        assert!(context.elapsed().num_milliseconds() >= 10);
    }

    #[test]
    fn test_context_module_path() {
        let context = ErrorContext::current();
        assert!(context.module_path().is_some());
        assert!(context.module_path().unwrap().contains("tests"));
    }
}
