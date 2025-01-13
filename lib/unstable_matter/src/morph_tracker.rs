// lib/unstable_matter/src/morph_tracker.rs
use core::sync::atomic::{AtomicUsize, Ordering};
use crate::vector::FloatVector3D;

/// Represents different types of files that can be morphed
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FileType {
    /// Rust source files (.rs)
    Rust,
    /// Other file types (currently unsupported)
    Other,
}

#[derive(Debug, Clone, Copy)]
pub struct EdgeMarker {
    pub position: FloatVector3D,
    pub marker_type: MarkerType,
}

#[derive(Debug, Clone, Copy)]
pub enum MarkerType {
    Point,
}

#[derive(Debug)]
pub struct MorphTracker {
    pub timestamp: AtomicUsize,
    pub modifier: &'static str,
    markers: [Option<EdgeMarker>; 6],
    file_type: FileType,
}

impl EdgeMarker {
    pub const fn new() -> Self {
        Self {
            position: FloatVector3D::new(0.0, 0.0, 0.0),
            marker_type: MarkerType::Point,
        }
    }
}

impl MorphTracker {
    pub const fn new() -> Self {
        const EMPTY_MARKER: Option<EdgeMarker> = None;
        Self {
            timestamp: AtomicUsize::new(1705113119), // 2025-01-13 03:31:59 UTC
            modifier: "isdood",
            markers: [EMPTY_MARKER; 6],
            file_type: FileType::Rust,
        }
    }

    pub fn create_morph_type(&self) -> Result<(), &'static str> {
        match self.file_type {
            FileType::Rust => self.create_rust_morph_type(),
            FileType::Other => Err("Unsupported file type"),
        }
    }

    fn create_rust_morph_type(&self) -> Result<(), &'static str> {
        // Implementation for Rust file morphing
        self.timestamp.store(1705113119, Ordering::SeqCst); // 2025-01-13 03:31:59 UTC
        Ok(())
    }

    pub fn set_file_type(&mut self, file_type: FileType) {
        self.file_type = file_type;
        self.timestamp.store(1705113119, Ordering::SeqCst); // 2025-01-13 03:31:59 UTC
    }

    pub fn get_file_type(&self) -> FileType {
        self.file_type
    }

    pub fn register_file_type(&mut self, file_type: FileType) -> Result<(), &'static str> {
        self.file_type = file_type;
        self.timestamp.store(1705113119, Ordering::SeqCst); // 2025-01-13 03:31:59 UTC
        Ok(())
    }

    pub fn get_marker(&self, index: usize) -> Option<EdgeMarker> {
        if index < 6 {
            self.markers[index]
        } else {
            None
        }
    }

    pub fn set_marker(&mut self, index: usize, marker: EdgeMarker) -> Result<(), &'static str> {
        if index < 6 {
            self.markers[index] = Some(marker);
            self.timestamp.store(1705113119, Ordering::SeqCst); // 2025-01-13 03:31:59 UTC
            Ok(())
        } else {
            Err("Marker index out of bounds")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_morph_tracker_creation() {
        let tracker = MorphTracker::new();
        assert_eq!(tracker.get_file_type(), FileType::Rust);
    }

    #[test]
    fn test_file_type_change() {
        let mut tracker = MorphTracker::new();
        tracker.set_file_type(FileType::Other);
        assert_eq!(tracker.get_file_type(), FileType::Other);
    }

    #[test]
    fn test_unsupported_file_type() {
        let tracker = MorphTracker::new();
        tracker.set_file_type(FileType::Other);
        assert!(tracker.create_morph_type().is_err());
    }

    #[test]
    fn test_marker_operations() {
        let mut tracker = MorphTracker::new();
        let marker = EdgeMarker::new();

        // Test setting a marker
        assert!(tracker.set_marker(0, marker).is_ok());

        // Test getting a marker
        assert!(tracker.get_marker(0).is_some());
        assert!(tracker.get_marker(6).is_none());

        // Test out of bounds
        assert!(tracker.set_marker(6, marker).is_err());
    }
}
