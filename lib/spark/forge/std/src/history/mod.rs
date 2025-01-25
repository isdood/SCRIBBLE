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
