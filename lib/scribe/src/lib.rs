//! Scribe Library
//! =============
//! Author: isdood
//! Created: 2025-01-23 02:24:21 UTC

mod native_string;
pub use native_string::String;

pub trait Scribe {
    fn scribe(&self) -> String;
}
