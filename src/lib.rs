//! Scribble Workspace Root
//! Last Updated: 2025-01-18 19:46:28 UTC
//! Author: isdood
//! Current User: isdood

#![no_std]

/// Re-export the main scribble crate
pub use scribble;

/// Workspace version
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Workspace build timestamp
pub const BUILD_TIMESTAMP: &str = "2025-01-18 19:46:28";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version() {
        assert!(!VERSION.is_empty());
    }

    #[test]
    fn test_timestamp() {
        assert_eq!(BUILD_TIMESTAMP, "2025-01-18 19:46:28");
    }
}
