// lib/unstable_matter/src/morph_tracker.rs
use core::sync::atomic::{AtomicUsize, Ordering};
use crate::vector::FloatVector3D;

#[derive(Debug, Clone, Copy)]
pub struct EdgeMarker {
    pub position: FloatVector3D,
    pub marker_type: MarkerType,
}

#[derive(Debug, Clone, Copy)]
pub struct MorphTracker {
    pub timestamp: AtomicUsize,
    pub modifier: &'static str,
    markers: [Option<EdgeMarker>; 6],
}

#[derive(Debug, Clone, Copy)]
pub enum MarkerType {
    Point,
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
            timestamp: AtomicUsize::new(1705109902), // 2025-01-13 02:53:22 UTC
            modifier: "isdood",
            markers: [EMPTY_MARKER; 6],
        }
    }

    pub fn register_file_type(&mut self, file_type: FileType) -> Result<(), &'static str> {
        let morph_type = match file_type {
            FileType::Rust => self.create_rust_morph_type(),
            FileType::Other => return Err("Unsupported file type"),
        };

        self.morph_types.push(morph_type);
        Ok(())
    }

    fn create_rust_morph_type(&self) -> MorphType {
        let edge_positions = [
            Vector3D::new(-1.0, 0.0, 0.0),
            Vector3D::new(1.0, 0.0, 0.0),
            Vector3D::new(0.0, 1.0, 0.0),
            Vector3D::new(0.0, -1.0, 0.0),
            Vector3D::new(0.0, 0.0, 1.0),
            Vector3D::new(0.0, 0.0, -1.0),
        ];

        let mut markers: [Option<EdgeMarker>; 6] = [None; 6];
        for (i, pos) in edge_positions.iter().enumerate() {
            markers[i] = Some(EdgeMarker {
                position: pos.clone(),
                              marker_type: MarkerType::Point,
            });
        }

        MorphType {
            file_type: FileType::Rust,
            edge_markers: markers,
        }
    }

    pub fn get_morph_type(&self, file_type: FileType) -> Option<&MorphType> {
        self.morph_types
        .iter()
        .find(|mt| mt.file_type == file_type)
    }
}
