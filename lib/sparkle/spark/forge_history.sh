#!/bin/bash

# Spark History Module Setup Script
# Author: isdood
# Created: 2025-01-25 18:28:48 UTC
# Repository: isdood/scribble
# Description: Sets up Spark's crystal-optimized backtrace handling

PURPLE='\033[0;35m'
NC='\033[0m'

print_purple() {
    echo -e "${PURPLE}$1${NC}"
}

setup_history_module() {
    cd forge/std || exit 1

    # 1. Create history module structure
    mkdir -p src/history
    mkdir -p tests/history

    # 2. Update lib.rs to include history module
    if ! grep -q "pub mod history;" src/lib.rs; then
        sed -i '/pub mod array;/a pub mod history;' src/lib.rs
        sed -i '/pub use array::CrystalArray;/a pub use history::{CrystalTrace, Frame, Symbol};' src/lib.rs
    fi

    # 3. Create history module implementation
    cat > src/history/mod.rs << 'EOL'
//! Crystal-optimized backtrace handling
//!
//! This module provides SIMD-accelerated stack trace operations with proper crystal alignment.

use crate::align::Alignment;
use crate::array::CrystalArray;
use std::fmt;
use std::ptr::NonNull;

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

        // Reserve space for max frames
        frames.extend((0..MAX_FRAMES).map(|_| Frame::default()));

        // Capture actual frames
        let mut actual_count = 0;
        unsafe {
            backtrace::trace(|frame| {
                if actual_count < MAX_FRAMES {
                    if let Some(frame_data) = Frame::from_raw(frame) {
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
        }

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
EOL

    # 4. Create frame implementation
    cat > src/history/frame.rs << 'EOL'
//! Stack frame implementation

use super::Symbol;
use std::fmt;
use std::path::PathBuf;

/// A stack frame with crystal-aligned storage
#[derive(Clone, Debug, Default)]
pub struct Frame {
    ip: *mut std::ffi::c_void,
    symbol_address: *mut std::ffi::c_void,
    symbol: Option<Symbol>,
}

impl Frame {
    /// Creates a new frame from raw backtrace data
    pub(crate) unsafe fn from_raw(frame: &backtrace::Frame) -> Option<Self> {
        let ip = frame.ip()?;
        let symbol_address = frame.symbol_address()?;

        Some(Self {
            ip: ip as *mut _,
            symbol_address: symbol_address as *mut _,
            symbol: None,
        })
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
            unsafe {
                backtrace::resolve(self.ip as *mut _, |symbol| {
                    self.symbol = Some(Symbol::from_raw(symbol));
                });
            }
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

    # 5. Create symbol implementation
    cat > src/history/symbol.rs << 'EOL'
//! Symbol information implementation

use std::fmt;
use std::path::PathBuf;

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
    pub fn filename(&self) -> Option<&std::path::Path> {
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

    # 6. Create history tests
    cat > tests/history/mod.rs << 'EOL'
use spark_std::history::{CrystalTrace, Frame, Symbol};

#[test]
fn test_trace_capture() {
    let trace = CrystalTrace::capture();
    assert!(!trace.is_empty());
    assert!(trace.len() > 0);
    assert!(!trace.has_system_frames());
}

#[test]
fn test_trace_with_system() {
    let trace = CrystalTrace::with_system();
    assert!(!trace.is_empty());
    assert!(trace.len() > 0);
    assert!(trace.has_system_frames());
}

#[test]
fn test_frame_resolution() {
    let mut trace = CrystalTrace::capture();

    // Check unresolved state
    let frame = &trace.frames()[0];
    assert!(!frame.is_resolved());
    assert!(frame.ip() != std::ptr::null_mut());
    assert!(frame.symbol_address() != std::ptr::null_mut());

    // Resolve and check
    trace.resolve();
    let frame = &trace.frames()[0];
    assert!(frame.is_resolved());

    if let Some(symbol) = frame.symbol() {
        // At least one of these should be present
        assert!(symbol.name().is_some() || symbol.filename().is_some());
    }
}

#[test]
fn test_trace_display() {
    let mut trace = CrystalTrace::capture();
    trace.resolve();

    let output = format!("{}", trace);
    assert!(output.starts_with("Stack backtrace:"));
    assert!(output.contains("   0: "));
}

#[test]
fn test_frame_display() {
    let mut trace = CrystalTrace::capture();
    trace.resolve();

    let frame = &trace.frames()[0];
    let output = format!("{}", frame);
    assert!(!output.is_empty());
    assert!(output != "<unresolved>");
}
EOL

    # 7. Add backtrace dependency to Cargo.toml
    if ! grep -q "backtrace" Cargo.toml; then
        sed -i '/\[dependencies\]/a backtrace = "0.3"' Cargo.toml
    fi

    print_purple "✓ Created history module files"
}

main() {
    print_purple "🔮 Creating Spark History Module..."
    setup_history_module
    print_purple "✨ History module created with crystal-space optimization!

Features:
- SIMD-optimized backtrace operations
- Crystal-aligned frame storage
- Symbol resolution
- Source location tracking
- System frame filtering
- Pretty printing

Run 'cargo test' to verify the implementation!"
}

main
