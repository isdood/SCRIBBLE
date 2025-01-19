//! Scribe - Native Formatting and Display System
//! ===========================================
//!
//! Author: Caleb J.D. Terkovics <isdood>
//! Current User: isdood
//! Created: 2025-01-19
//! Last Updated: 2025-01-19 14:25:58 UTC
//! Version: 0.1.0
//! License: MIT

/// Core trait for types that can be converted to string representations
pub trait Scribe {
    /// Convert the type to its string representation
    fn scribe(&self) -> String;

    /// Get the type name for debugging and error messages
    fn type_name(&self) -> &'static str {
        std::any::type_name::<Self>()
    }

    /// Get a debug representation
    fn scribe_debug(&self) -> String {
        format!("{}({:?})", self.type_name(), self.scribe())
    }
}

/// Helper functions for working with Scribe types
pub mod utils {
    /// Join multiple strings with a separator
    pub fn join(items: &[String], separator: &str) -> String {
        let mut result = String::new();
        for (i, item) in items.iter().enumerate() {
            if i > 0 {
                result.push_str(separator);
            }
            result.push_str(item);
        }
        result
    }

    /// Format multiple scribeable items
    pub fn format_multiple<T: super::Scribe>(items: &[T], separator: &str) -> String {
        let strings: Vec<String> = items.iter().map(|item| item.scribe()).collect();
        join(&strings, separator)
    }

    /// Wrap a string with a prefix and suffix
    pub fn wrap(content: &str, prefix: &str, suffix: &str) -> String {
        let mut result = String::with_capacity(
            prefix.len() + content.len() + suffix.len()
        );
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
            Ok(value) => utils::wrap(&value.scribe(), "Ok(", ")"),
            Err(error) => utils::wrap(&error.scribe(), "Err(", ")"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug)]
    struct TestType {
        name: String,
        value: i32,
    }

    impl Scribe for TestType {
        fn scribe(&self) -> String {
            format!("{}: {}", self.name, self.value)
        }
    }

    #[test]
    fn test_basic_scribe() {
        let test = TestType {
            name: "test".to_string(),
            value: 42,
        };
        assert_eq!(test.scribe(), "test: 42");
    }

    #[test]
    fn test_utils_join() {
        let items = vec!["a".to_string(), "b".to_string(), "c".to_string()];
        assert_eq!(utils::join(&items, ", "), "a, b, c");
    }

    #[test]
    fn test_utils_format_multiple() {
        let items = vec![
            TestType { name: "first".to_string(), value: 1 },
            TestType { name: "second".to_string(), value: 2 },
        ];
        assert_eq!(utils::format_multiple(&items, "; "), "first: 1; second: 2");
    }

    #[test]
    fn test_utils_wrap() {
        let content = "test";
        assert_eq!(utils::bracketed(content), "[test]");
        assert_eq!(utils::parenthesized(content), "(test)");
        assert_eq!(utils::braced(content), "{test}");
        assert_eq!(utils::quoted(content), "\"test\"");
    }

    #[test]
    fn test_result_scribe() {
        let ok_result: ScribeResult<TestType, TestType> = Ok(TestType {
            name: "success".to_string(),
                                                             value: 1,
        });
        let err_result: ScribeResult<TestType, TestType> = Err(TestType {
            name: "error".to_string(),
                                                               value: 0,
        });

        assert_eq!(ok_result.to_scribe_string(), "Ok(success: 1)");
        assert_eq!(err_result.to_scribe_string(), "Err(error: 0)");
    }

    #[test]
    fn test_type_name() {
        let test = TestType {
            name: "test".to_string(),
            value: 42,
        };
        assert!(test.type_name().contains("TestType"));
    }
}
