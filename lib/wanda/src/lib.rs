/// Wanda AI Assistant Library
/// Last Updated: 2025-01-15 22:44:30 UTC
/// Author: isdood
/// Current User: isdood
///
/// This library provides the core functionality for the Wanda AI Assistant.
/// It includes modules for service management and type definitions.

// Module declarations
pub mod paths;
pub mod service;
pub mod types;
pub mod prolog;
pub mod brain;

// Re-exports
pub use paths::*;
pub use service::WandaService;
pub use types::*;
pub use brain::WandaBrain;
pub use prolog::PrologBridge;

// Library-wide constants
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version() {
        assert!(!VERSION.is_empty(), "Version should not be empty");
    }
}
