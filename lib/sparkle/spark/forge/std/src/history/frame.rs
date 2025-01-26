//! Stack frame implementation

use super::Symbol;
use std::fmt;

/// A stack frame with crystal-aligned storage
#[derive(Clone, Debug)]
pub struct Frame {
    ip: *mut std::ffi::c_void,
    symbol_address: *mut std::ffi::c_void,
    symbol: Option<Symbol>,
}

impl Default for Frame {
    fn default() -> Self {
        Self {
            ip: std::ptr::null_mut(),
            symbol_address: std::ptr::null_mut(),
            symbol: None,
        }
    }
}

impl Frame {
    /// Creates a new frame from raw backtrace data
    pub(crate) unsafe fn from_raw(frame: &backtrace::Frame) -> Option<Self> {
        let ip = frame.ip();
        let symbol_address = frame.symbol_address();

        if ip.is_null() || symbol_address.is_null() {
            None
        } else {
            Some(Self {
                ip,
                symbol_address,
                symbol: None,
            })
        }
    }

    /// Returns the instruction pointer for this frame
    pub fn ip(&self) -> *mut std::ffi::c_void {
        self.ip
    }

    /// Returns the symbol address for this frame
    pub fn symbol_address(&self) -> *mut std::ffi::c_void {
        self.symbol_address
    }

    /// Returns true if this frame has resolved symbol information
    pub fn is_resolved(&self) -> bool {
        self.symbol.is_some()
    }

    /// Resolves symbol information for this frame
    pub fn resolve(&mut self) {
        if self.symbol.is_none() {
            backtrace::resolve(self.ip, |symbol| {
                self.symbol = Some(Symbol::from_raw(symbol));
            });
        }
    }

    /// Returns the symbol information for this frame
    pub fn symbol(&self) -> Option<&Symbol> {
        self.symbol.as_ref()
    }
}

impl fmt::Display for Frame {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(symbol) = &self.symbol {
            write!(f, "{}", symbol)
        } else {
            write!(f, "<unresolved>")
        }
    }
}
