use std::sync::atomic::AtomicUsize;
use crate::vector::Vector3D;

#[derive(Debug, Clone)]
pub struct SpaceConfig {
    pub dimensions: Vector3D,
    pub cell_size: usize,
    pub cells: Vector3D,
}

#[derive(Debug, Clone)]
pub struct SpaceMetadata {
    pub creation_time: AtomicUsize,
    pub last_modified: AtomicUsize,
    pub creator: &'static str,
    pub last_modifier: &'static str,
}
