//! Scribe - Native Formatting and Display System
//! ===========================================
//!
//! Author: Caleb J.D. Terkovics <isdood>
//! Current User: isdood
//! Created: 2025-01-19
//! Last Updated: 2025-01-19 18:02:27 UTC
//! Version: 0.1.0
//! License: MIT

pub mod native_string; // Add this line to include the new module

use native_string::String; // Use the custom String type

/// Core trait for types that can be converted to string representations
pub trait Scribe {
    /// Convert the type to its string representation
    fn scribe(&self) -> String;

    /// Get the type name for debugging and error messages
    fn type_name(&self) -> &'static str {
        core::any::type_name::<Self>()
    }

    /// Get a debug representation
    fn scribe_debug(&self) -> String {
        let mut result = String::new();
        result.push_str(self.type_name());
        result.push_str("(");
        result.push_str(&self.scribe().to_str()); // Use to_str method here
        result.push_str(")");
        result
    }
}

/// Helper functions for working with Scribe types
pub mod utils {
    use super::native_string::String; // Use the custom String type

    /// Join multiple strings with a separator
    pub fn join(items: &[String], separator: &str) -> String {
        let mut result = String::new();
        for (i, item) in items.iter().enumerate() {
            if i > 0 {
                result.push_str(separator);
            }
            result.push_str(item.to_str()); // Use to_str method here
        }
        result
    }

    /// Format multiple scribeable items
    pub fn format_multiple<T: super::Scribe>(items: &[T], separator: &str) -> String {
        let strings: std::vec::Vec<String> = items.iter().map(|item| item.scribe()).collect();
        join(&strings, separator)
    }

    /// Wrap a string with a prefix and suffix
    pub fn wrap(content: &str, prefix: &str, suffix: &str) -> String {
        let mut result = String::new();
        result.push_str(prefix);
        result.push_str(content);
        result.push_str(suffix);
        result
    }

    /// Format with brackets
    pub fn bracketed(content: &str) -> String {
        wrap(content, "[", "]")
    }

    /// Format with parentheses
    pub fn parenthesized(content: &str) -> String {
        wrap(content, "(", ")")
    }

    /// Format with braces
    pub fn braced(content: &str) -> String {
        wrap(content, "{", "}")
    }

    /// Format with quotes
    pub fn quoted(content: &str) -> String {
        wrap(content, "\"", "\"")
    }
}

/// Result type with built-in string conversion
pub type ScribeResult<T, E> = Result<T, E>;

/// Extension trait for Result types
pub trait ScribeResultExt {
    /// Convert the result to a string representation
    fn to_scribe_string(&self) -> String;
}

impl<T: Scribe, E: Scribe> ScribeResultExt for ScribeResult<T, E> {
    fn to_scribe_string(&self) -> String {
        match self {
            Ok(value) => utils::wrap(&value.scribe().to_str(), "Ok(", ")"), // Use to_str method here
            Err(error) => utils::wrap(&error.scribe().to_str(), "Err(", ")"), // Use to_str method here
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::native_string::String; // Use the custom String type

    #[derive(Debug)]
    struct TestType {
        name: String,
        value: i32,
    }

    impl Scribe for TestType {
        fn scribe(&self) -> String {
            let mut result = String::new();
            result.push_str(&self.name.to_str()); // Use to_str method here
            result.push_str(": ");
            result.push_str(&self.value.to_string());
            result
        }
    }

    #[test]
    fn test_basic_scribe() {
        let mut name = String::new();
        name.push_str("test");
        let test = TestType {
            name,
            value: 42,
        };
        assert_eq!(test.scribe().to_str(), "test: 42");
    }

    #[test]
    fn test_utils_join() {
        let mut a = String::new();
        a.push_str("a");
        let mut b = String::new();
        b.push_str("b");
        let mut c = String::new();
        c.push_str("c");
        let items = vec![a, b, c];
        assert_eq!(utils::join(&items, ", ").to_str(), "a, b, c");
    }

    #[test]
    fn test_utils_format_multiple() {
        let items = vec![
            TestType { name: "first".to_string(), value: 1 },
            TestType { name: "second".to_string(), value: 2 },
        ];
        assert_eq!(utils::format_multiple(&items, "; ").to_str(), "first: 1; second: 2");
    }

    #[test]
    fn test_utils_wrap() {
        let content = "test";
        assert_eq!(utils::bracketed(content).to_str(), "[test]");
        assert_eq!(utils::parenthesized(content).to_str(), "(test)");
        assert_eq!(utils::braced(content).to_str(), "{test}");
        assert_eq!(utils::quoted(content).to_str(), "\"test\"");
    }

    #[test]
    fn test_result_scribe() {
        let mut success_name = String::new();
        success_name.push_str("success");
        let ok_result: ScribeResult<TestType, TestType> = Ok(TestType {
            name: success_name,
            value: 1,
        });

        let mut error_name = String::new();
        error_name.push_str("error");
        let err_result: ScribeResult<TestType, TestType> = Err(TestType {
            name: error_name,
            value: 0,
        });

        assert_eq!(ok_result.to_scribe_string().to_str(), "Ok(success: 1)");
        assert_eq!(err_result.to_scribe_string().to_str(), "Err(error: 0)");
    }

    #[test]
    fn test_type_name() {
        let mut name = String::new();
        name.push_str("test");
        let test = TestType {
            name,
            value: 42,
        };
        assert!(test.type_name().contains("TestType"));
    }
}
