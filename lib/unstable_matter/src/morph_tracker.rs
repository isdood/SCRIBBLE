// lib/unstable_matter/src/morph_tracker.rs
/// Last Updated: 2025-01-13 04:03:53 UTC
/// Author: isdood
/// Current User: isdood

use core::sync::atomic::{AtomicUsize, Ordering};
use crate::vector::FloatVector3D;

/// Represents different types of files that can be morphed
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FileType {
    /// Rust source files (.rs)
    Rust,
    /// Other file types (currently unsupported)
    Other,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct EdgeMarker {
    pub position: FloatVector3D,
    pub marker_type: MarkerType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MarkerType {
    Point,
}

#[derive(Debug)]
pub struct MorphTracker {
    timestamp: AtomicUsize,
    modifier: &'static str,
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

impl Clone for MorphTracker {
    fn clone(&self) -> Self {
        Self {
            timestamp: AtomicUsize::new(1705115033), // 2025-01-13 04:03:53 UTC
            modifier: self.modifier,
            markers: self.markers,
            file_type: self.file_type,
        }
    }
}

impl MorphTracker {
    pub const fn new() -> Self {
        const EMPTY_MARKER: Option<EdgeMarker> = None;
        Self {
            timestamp: AtomicUsize::new(1705115033), // 2025-01-13 04:03:53 UTC
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
        self.timestamp.store(1705115033, Ordering::SeqCst); // 2025-01-13 04:03:53 UTC
        Ok(())
    }

    pub fn set_file_type(&mut self, file_type: FileType) {
        self.file_type = file_type;
        self.timestamp.store(1705115033, Ordering::SeqCst); // 2025-01-13 04:03:53 UTC
    }

    pub fn get_file_type(&self) -> FileType {
        self.file_type
    }

    pub fn register_file_type(&mut self, file_type: FileType) -> Result<(), &'static str> {
        self.file_type = file_type;
        self.timestamp.store(1705115033, Ordering::SeqCst); // 2025-01-13 04:03:53 UTC
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
            self.timestamp.store(1705115033, Ordering::SeqCst); // 2025-01-13 04:03:53 UTC
            Ok(())
        } else {
            Err("Marker index out of bounds")
        }
    }

    pub fn get_modifier(&self) -> &'static str {
        self.modifier
    }

    pub fn get_timestamp(&self) -> usize {
        self.timestamp.load(Ordering::SeqCst)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_morph_tracker_creation() {
        let tracker = MorphTracker::new();
        assert_eq!(tracker.get_file_type(), FileType::Rust);
        assert_eq!(tracker.get_modifier(), "isdood");
    }

    #[test]
    fn test_morph_tracker_clone() {
        let tracker = MorphTracker::new();
        let cloned = tracker.clone();
        assert_eq!(tracker.get_file_type(), cloned.get_file_type());
        assert_eq!(tracker.get_modifier(), cloned.get_modifier());
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

    #[test]
    fn test_edge_marker_clone() {
        let marker = EdgeMarker::new();
        let cloned = marker.clone();
        assert_eq!(marker.position.x, cloned.position.x);
        assert_eq!(marker.marker_type, cloned.marker_type);
    }
}
