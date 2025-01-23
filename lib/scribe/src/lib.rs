//! Scribe Library
//! =============
//! Author: isdood
//! Created: 2025-01-23 02:13:29 UTC

mod native_string;

pub trait Scribe {
    fn scribe(&self) -> String;
}

pub use native_string::String;
