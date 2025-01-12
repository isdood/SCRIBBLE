/// VectorSpace: Memory as Mathematical Spaces with UFO Protection
/// Last Updated: 2025-01-12 23:49:42 UTC
/// Author: Caleb J.D. Terkovics (isdood)
/// Current User: isdood

use crate::{
    SpaceTime,
    ufo::{Flying, TrackedUFO},
};
use core::{
    sync::atomic::{AtomicUsize, Ordering},
    marker::PhantomData,
    ops::{Add, Sub, Mul, Div},
};

#[derive(Debug)]
pub struct VectorSpace<S = Flying> {
    /// Base address of the physical memory
    pub(crate) origin: usize,
    /// 3D mesh representing our memory space
    pub(crate) mesh: SpaceTime<MeshCell>,
    /// Spatial configuration
    pub(crate) config: SpaceConfig,
    /// UFO tracking for memory safety
    ufo: TrackedUFO<MeshCell>,
    /// Creation metadata
    metadata: SpaceMetadata,
    /// State marker
    _state: PhantomData<S>,
}

#[derive(Debug)]
pub struct SpaceMetadata {
    creation_time: AtomicUsize,
    last_modified: AtomicUsize,
    creator: &'static str,
    last_modifier: &'static str,
}

impl SpaceMetadata {
    pub fn creation_time(&self) -> usize {
        self.creation_time.load(Ordering::SeqCst)
    }

    pub fn last_modified(&self) -> usize {
        self.last_modified.load(Ordering::SeqCst)
    }

    pub fn creator(&self) -> &'static str {
        self.creator
    }

    pub fn last_modifier(&self) -> &'static str {
        self.last_modifier
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Vector3D {
    pub(crate) x: isize,
    pub(crate) y: isize,
    pub(crate) z: isize,
}

impl Vector3D {
    pub const fn new(x: isize, y: isize, z: isize) -> Self {
        Self { x, y, z }
    }

    pub const fn zero() -> Self {
        Self::new(0, 0, 0)
    }

    pub const fn one() -> Self {
        Self::new(1, 1, 1)
    }

    pub const fn unit_x() -> Self { Self::new(1, 0, 0) }
    pub const fn unit_y() -> Self { Self::new(0, 1, 0) }
    pub const fn unit_z() -> Self { Self::new(0, 0, 1) }

    pub const fn x(&self) -> isize { self.x }
    pub const fn y(&self) -> isize { self.y }
    pub const fn z(&self) -> isize { self.z }

    pub fn dot(&self, other: &Vector3D) -> isize {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross(&self, other: &Vector3D) -> Vector3D {
        Vector3D::new(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x,
        )
    }

    pub fn magnitude_squared(&self) -> isize {
        self.dot(self)
    }

    pub fn scale(&self, factor: isize) -> Vector3D {
        Vector3D::new(
            self.x * factor,
            self.y * factor,
            self.z * factor,
        )
    }

    pub fn hadamard(&self, other: &Vector3D) -> Vector3D {
        Vector3D::new(
            self.x * other.x,
            self.y * other.y,
            self.z * other.z,
        )
    }
}

impl Add for Vector3D {
    type Output = Vector3D;
    fn add(self, other: Vector3D) -> Vector3D {
        Vector3D::new(
            self.x + other.x,
            self.y + other.y,
            self.z + other.z,
        )
    }
}

impl Sub for Vector3D {
    type Output = Vector3D;
    fn sub(self, other: Vector3D) -> Vector3D {
        Vector3D::new(
            self.x - other.x,
            self.y - other.y,
            self.z - other.z,
        )
    }
}

impl Mul<isize> for Vector3D {
    type Output = Vector3D;
    fn mul(self, rhs: isize) -> Vector3D {
        self.scale(rhs)
    }
}

impl Div<isize> for Vector3D {
    type Output = Vector3D;
    fn div(self, rhs: isize) -> Vector3D {
        Vector3D::new(
            self.x / rhs,
            self.y / rhs,
            self.z / rhs,
        )
    }
}

#[derive(Debug, Clone, Copy)]
pub struct MeshCell {
    pub(crate) position: Vector3D,
    pub(crate) state: CellState,
    pub(crate) links: [Option<Vector3D>; 6],
    timestamp: usize,
    modifier: &'static str,
}

impl MeshCell {
    pub fn new(position: Vector3D, state: CellState, timestamp: usize, modifier: &'static str) -> Self {
        Self {
            position,
            state,
            links: [None; 6],
            timestamp,
            modifier,
        }
    }

    pub fn with_links(mut self, links: [Option<Vector3D>; 6]) -> Self {
        self.links = links;
        self
    }

    pub fn position(&self) -> Vector3D {
        self.position
    }

    pub fn state(&self) -> CellState {
        self.state
    }

    pub fn timestamp(&self) -> usize {
        self.timestamp
    }

    pub fn modifier(&self) -> &'static str {
        self.modifier
    }

    pub fn links(&self) -> &[Option<Vector3D>; 6] {
        &self.links
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CellState {
    Free,
    Allocated,
    Reserved,
    Protected,
    TimeLocked,
}

impl CellState {
    pub fn is_protected(&self) -> bool {
        matches!(self, Self::Protected | Self::TimeLocked)
    }

    pub fn is_available(&self) -> bool {
        matches!(self, Self::Free)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct SpaceConfig {
    pub(crate) dimensions: Vector3D,
    pub(crate) cell_size: usize,
    pub(crate) cells: Vector3D,
}

impl SpaceConfig {
    pub fn dimensions(&self) -> Vector3D {
        self.dimensions
    }

    pub fn cell_size(&self) -> usize {
        self.cell_size
    }

    pub fn cells(&self) -> Vector3D {
        self.cells
    }
}

impl VectorSpace {
    pub fn new(origin: usize, size: usize) -> Self {
        let config = SpaceConfig {
            dimensions: Vector3D::new(size as isize, size as isize, size as isize),
            cell_size: 256,
            cells: Vector3D::new(16, 16, 16),
        };

        let mesh_size = (config.cells.x * config.cells.y * config.cells.z) as usize;
        let current_time = 1705099782; // 2025-01-12 23:49:42 UTC

        let metadata = SpaceMetadata {
            creation_time: AtomicUsize::new(current_time),
            last_modified: AtomicUsize::new(current_time),
            creator: "isdood",
            last_modifier: "isdood",
        };

        Self {
            origin,
            mesh: SpaceTime::new(origin + size, mesh_size, 0),
            config,
            ufo: TrackedUFO::new(origin),
            metadata,
            _state: PhantomData,
        }
    }

    pub fn init_mesh(&mut self) -> Result<(), &'static str> {
        let current_time = 1705099782; // 2025-01-12 23:49:42 UTC

        for z in 0..self.config.cells.z {
            for y in 0..self.config.cells.y {
                for x in 0..self.config.cells.x {
                    let position = Vector3D::new(x, y, z);
                    let idx = self.position_to_index(&position);
                    let links = self.calculate_cell_links(&position);
                    let cell = MeshCell::new(
                        position,
                        CellState::Free,
                        current_time,
                        "isdood",
                    ).with_links(links);

                    unsafe { self.mesh.write_at(idx, cell); }
                }
            }
        }

        self.metadata.last_modified.store(current_time, Ordering::SeqCst);
        self.metadata.last_modifier = "isdood";
        Ok(())
    }

    pub fn calculate_cell_links(&self, pos: &Vector3D) -> [Option<Vector3D>; 6] {
        let mut links = [None; 6];
        let directions = [
            Vector3D::new(-1, 0, 0), // Left
            Vector3D::new(1, 0, 0),  // Right
            Vector3D::new(0, -1, 0), // Down
            Vector3D::new(0, 1, 0),  // Up
            Vector3D::new(0, 0, -1), // Back
            Vector3D::new(0, 0, 1),  // Front
        ];

        for (i, dir) in directions.iter().enumerate() {
            let neighbor = *pos + *dir;
            if self.is_valid_position(&neighbor) {
                links[i] = Some(neighbor);
            }
        }

        links
    }

    pub fn is_valid_position(&self, pos: &Vector3D) -> bool {
        pos.x >= 0 && pos.x < self.config.cells.x &&
        pos.y >= 0 && pos.y < self.config.cells.y &&
        pos.z >= 0 && pos.z < self.config.cells.z
    }

    pub fn position_to_index(&self, pos: &Vector3D) -> usize {
        (pos.z * self.config.cells.x * self.config.cells.y +
        pos.y * self.config.cells.x +
        pos.x) as usize
    }

    pub fn index_to_position(&self, idx: usize) -> Vector3D {
        let x = (idx % self.config.cells.x as usize) as isize;
        let y = ((idx / self.config.cells.x as usize) % self.config.cells.y as usize) as isize;
        let z = (idx / (self.config.cells.x * self.config.cells.y) as usize) as isize;
        Vector3D::new(x, y, z)
    }

    pub fn addr_to_cell(&self, addr: usize) -> Option<Vector3D> {
        if addr < self.origin || addr >= self.origin + self.config.cell_size * self.mesh.size() {
            return None;
        }
        let idx = (addr - self.origin) / self.config.cell_size;
        Some(self.index_to_position(idx))
    }

    pub fn config(&self) -> &SpaceConfig {
        &self.config
    }

    pub fn metadata(&self) -> &SpaceMetadata {
        &self.metadata
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vector_operations() {
        let v1 = Vector3D::new(1, 2, 3);
        let v2 = Vector3D::new(4, 5, 6);
        let sum = v1 + v2;
        let diff = v2 - v1;
        let scaled = v1 * 2;

        assert_eq!(sum, Vector3D::new(5, 7, 9));
        assert_eq!(diff, Vector3D::new(3, 3, 3));
        assert_eq!(scaled, Vector3D::new(2, 4, 6));

        let dot = v1.dot(&v2);
        assert_eq!(dot, 32);

        let cross = v1.cross(&v2);
        assert_eq!(cross, Vector3D::new(-3, 6, -3));
    }

    #[test]
    fn test_cell_state() {
        let state = CellState::Protected;
        assert!(state.is_protected());
        assert!(!state.is_available());

        let state = CellState::Free;
        assert!(!state.is_protected());
        assert!(state.is_available());
    }

    #[test]
    fn test_mesh_initialization() {
        let mut space = VectorSpace::new(0x1000, 0x10000);
        assert!(space.init_mesh().is_ok());
        let pos = Vector3D::new(1, 1, 1);
        let idx = space.position_to_index(&pos);
        let cell = unsafe { space.mesh.read_at(idx) };

        assert_eq!(cell.position(), pos);
        assert_eq!(cell.state(), CellState::Free);

        // Test cell links
        let right_link = cell.links[1];
        assert_eq!(right_link, Some(Vector3D::new(2, 1, 1)));
    }
}
