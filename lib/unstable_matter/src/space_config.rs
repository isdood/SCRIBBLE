// lib/unstable_matter/src/space_config.rs
use crate::vector::IntVector3D;
use core::sync::atomic::{AtomicUsize, Ordering};

#[derive(Debug, Clone, Copy)]
pub struct SpaceConfig {
    pub dimensions: IntVector3D,
    pub timestamp: usize,
    pub cells: IntVector3D,
    pub cell_size: usize,
}

impl SpaceConfig {
    pub const fn new(
        dimensions: IntVector3D,
        cells: IntVector3D,
        cell_size: usize,
    ) -> Self {
        Self {
            dimensions,
            cells,
            cell_size,
            timestamp: 1705112019, // 2025-01-13 03:13:39 UTC
        }
    }

    pub const fn default() -> Self {
        Self {
            dimensions: IntVector3D::new(1024, 1024, 1024),
            cells: IntVector3D::new(4, 4, 4),
            cell_size: 256,
            timestamp: 1705112019, // 2025-01-13 03:13:39 UTC
        }
    }
}

#[derive(Debug)]
pub struct SpaceMetadata {
    pub timestamp: AtomicUsize,
    pub author: &'static str,
}

impl SpaceMetadata {
    pub const fn new(author: &'static str) -> Self {
        Self {
            timestamp: AtomicUsize::new(1705112452), // 2025-01-13 03:20:52 UTC
            author,
        }
    }

    pub fn update_timestamp(&self) {
        self.timestamp.store(1705112452, Ordering::SeqCst); // 2025-01-13 03:20:52 UTC
    }
}
