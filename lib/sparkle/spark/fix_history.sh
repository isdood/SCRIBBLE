#!/bin/bash

# Spark History Fix Script
# Author: isdood
# Created: 2025-01-25 18:30:41 UTC
# Repository: isdood/scribble
# Description: Fixes history module implementation issues

PURPLE='\033[0;35m'
NC='\033[0m'

print_purple() {
    echo -e "${PURPLE}$1${NC}"
}

fix_history_module() {
    cd forge/std || exit 1

    # 1. Update Frame implementation to fix Default and Try issues
    cat > src/history/frame.rs << 'EOL'
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
EOL

    # 2. Update history module to fix extend and unused imports
    cat > src/history/mod.rs << 'EOL'
//! Crystal-optimized backtrace handling
//!
//! This module provides SIMD-accelerated stack trace operations with proper crystal alignment.

use crate::align::Alignment;
use crate::array::CrystalArray;
use std::fmt;

mod frame;
mod symbol;

pub use frame::Frame;
pub use symbol::Symbol;

/// Maximum frames to capture in a single trace
const MAX_FRAMES: usize = 256;

/// A crystal-space aligned backtrace
#[derive(Clone)]
pub struct CrystalTrace {
    frames: CrystalArray<Frame>,
    system_base: bool,
}

impl CrystalTrace {
    /// Captures a new backtrace
    pub fn capture() -> Self {
        Self::with_options(false)
    }

    /// Captures a new backtrace with system frames
    pub fn with_system() -> Self {
        Self::with_options(true)
    }

    fn with_options(system_base: bool) -> Self {
        let mut frames = CrystalArray::new(Alignment::Crystal16);

        // Pre-allocate space for frames
        for _ in 0..MAX_FRAMES {
            frames.push(Frame::default());
        }

        // Capture actual frames
        let mut actual_count = 0;
        backtrace::trace(|frame| {
            if actual_count < MAX_FRAMES {
                if let Some(frame_data) = unsafe { Frame::from_raw(frame) } {
                    frames[actual_count] = frame_data;
                    actual_count += 1;
                    true
                } else {
                    false
                }
            } else {
                false
            }
        });

        // Truncate to actual size
        while frames.len() > actual_count {
            frames.pop();
        }

        Self {
            frames,
            system_base,
        }
    }

    /// Returns the frames in this trace
    pub fn frames(&self) -> &[Frame] {
        &self.frames
    }

    /// Returns true if this trace includes system frames
    pub fn has_system_frames(&self) -> bool {
        self.system_base
    }

    /// Returns the number of frames in this trace
    pub fn len(&self) -> usize {
        self.frames.len()
    }

    /// Returns true if this trace has no frames
    pub fn is_empty(&self) -> bool {
        self.frames.is_empty()
    }

    /// Resolves all frame symbols
    pub fn resolve(&mut self) {
        for frame in self.frames.iter_mut() {
            frame.resolve();
        }
    }
}

impl fmt::Debug for CrystalTrace {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("CrystalTrace")
            .field("frames", &self.frames)
            .field("system_base", &self.system_base)
            .finish()
    }
}

impl fmt::Display for CrystalTrace {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Stack backtrace:")?;
        for (idx, frame) in self.frames.iter().enumerate() {
            write!(f, "{:4}: ", idx)?;
            if frame.is_resolved() {
                writeln!(f, "{}", frame)?;
            } else {
                writeln!(f, "<unresolved>")?;
            }
        }
        Ok(())
    }
}

impl std::ops::Index<usize> for CrystalTrace {
    type Output = Frame;

    fn index(&self, index: usize) -> &Self::Output {
        &self.frames[index]
    }
}

impl std::ops::IndexMut<usize> for CrystalTrace {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.frames[index]
    }
}
EOL

    # 3. Update symbol implementation to remove unused imports
    cat > src/history/symbol.rs << 'EOL'
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
EOL

    print_purple "✓ Fixed history module implementation"
}

main() {
    print_purple "🔮 Fixing Spark History Module..."
    fix_history_module
    print_purple "✨ History module fixes applied!

Fixed Issues:
- Removed unnecessary Default derive
- Added manual Default implementation for Frame
- Fixed Try operator usage
- Removed unnecessary extend method usage
- Cleaned up unused imports
- Added proper null pointer checks
- Added Index/IndexMut implementations

Run 'cargo test' to verify the fixes!"
}

main
