// lib/unstable_matter/src/vector.rs
#[derive(Debug, Clone, Copy)]
pub struct Vector3D<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T> Vector3D<T> {
    pub const fn new(x: T, y: T, z: T) -> Self {
        Self { x, y, z }
    }
}

pub type IntVector3D = Vector3D<isize>;
pub type FloatVector3D = Vector3D<f64>;
