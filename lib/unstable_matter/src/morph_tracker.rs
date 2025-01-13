use std::sync::atomic::{AtomicUsize, Ordering};
use crate::vector::Vector3D;

#[derive(Debug, Clone)]
pub struct MorphTracker {
    pub timestamp: AtomicUsize,
    pub modifier: &'static str,
    morph_types: Vec<MorphType>,
}

#[derive(Debug, Clone)]
pub struct MorphType {
    pub file_type: FileType,
    pub edge_markers: [Option<EdgeMarker>; 6],
}

#[derive(Debug, Clone)]
pub struct EdgeMarker {
    pub position: Vector3D,
    pub marker_type: MarkerType,
}

#[derive(Debug, Clone, PartialEq)]
pub enum FileType {
    Rust,
    Other,
}

#[derive(Debug, Clone)]
pub enum MarkerType {
    Point,
}

impl Default for EdgeMarker {
    fn default() -> Self {
        Self {
            position: Vector3D::new(0, 0, 0),
            marker_type: MarkerType::Point,
        }
    }
}

impl MorphTracker {
    pub fn new() -> Self {
        Self {
            timestamp: AtomicUsize::new(1705108505), // 2025-01-13 02:35:05 UTC
            modifier: "isdood",
            morph_types: Vec::new(),
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
