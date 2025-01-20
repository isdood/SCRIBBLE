//! Error context module providing location and timing information for errors
//! Created: 2025-01-20 21:37:15
//! Author: isdood

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fmt;

/// Stores contextual information about where and when an error occurred
#[derive(Debug, Clone, Serialize, Deserialize)]
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
    pub extra: Option<serde_json::Value>,
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
            module_path: std::module_path().map(String::from),
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

    /// Adds additional context data
    pub fn with_extra(mut self, extra: impl Serialize) -> Result<Self, serde_json::Error> {
        self.extra = Some(serde_json::to_value(extra)?);
        Ok(self)
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
    }

    #[test]
    fn test_context_with_extra() {
        #[derive(Serialize)]
        struct Extra {
            value: i32,
        }

        let context = ErrorContext::current()
        .with_extra(Extra { value: 42 })
        .unwrap();

        if let Some(extra) = context.extra {
            assert_eq!(extra["value"], 42);
        } else {
            panic!("Extra data not set");
        }
    }

    #[test]
    fn test_context_display() {
        let context = ErrorContext::new("test.rs", 10, 5);
        let display = context.to_string();
        assert!(display.contains("test.rs:10:5"));
        assert!(display.contains(&Utc::now().format("%Y").to_string()));
    }
}
