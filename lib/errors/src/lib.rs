//! Error handling for the Scribble ecosystem
//! Author: isdood
//! Created: 2025-01-23 02:13:29 UTC

use scribe::String;

#[derive(Debug)]
pub struct Error {
    message: String,
}

impl Error {
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}
