//! Symbol information implementation

use std::fmt;
use std::path::{Path, PathBuf};

/// Symbol information with crystal-aligned storage
#[derive(Clone, Debug)]
pub struct Symbol {
    name: Option<String>,
    filename: Option<PathBuf>,
    lineno: Option<u32>,
    colno: Option<u32>,
}

impl Symbol {
    /// Creates a new symbol from raw backtrace data
    pub(crate) fn from_raw(symbol: &backtrace::Symbol) -> Self {
        Self {
            name: symbol.name().map(|s| s.to_string()),
            filename: symbol.filename().map(|p| p.to_owned()),
            lineno: symbol.lineno(),
            colno: symbol.colno(),
        }
    }

    /// Returns the demangled symbol name
    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    /// Returns the source file path
    pub fn filename(&self) -> Option<&Path> {
        self.filename.as_deref()
    }

    /// Returns the line number
    pub fn lineno(&self) -> Option<u32> {
        self.lineno
    }

    /// Returns the column number
    pub fn colno(&self) -> Option<u32> {
        self.colno
    }
}

impl fmt::Display for Symbol {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(name) = &self.name {
            write!(f, "{}", name)?;
        } else {
            write!(f, "<unknown>")?;
        }

        if let Some(filename) = &self.filename {
            write!(f, " at {}", filename.display())?;
            if let Some(lineno) = self.lineno {
                write!(f, ":{}", lineno)?;
                if let Some(colno) = self.colno {
                    write!(f, ":{}", colno)?;
                }
            }
        }

        Ok(())
    }
}
