//! Native String Implementation

#[derive(Debug, Clone)]
pub struct String(std::string::String);

impl String {
    pub fn new() -> Self {
        Self(std::string::String::new())
    }
}

impl From<std::string::String> for String {
    fn from(s: std::string::String) -> Self {
        Self(s)
    }
}

impl From<&str> for String {
    fn from(s: &str) -> Self {
        Self(s.to_owned())
    }
}

impl AsRef<str> for String {
    fn as_ref(&self) -> &str {
        self.0.as_ref()
    }
}
