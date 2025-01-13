#[derive(Debug, Clone)]
pub struct Vector3D {
    pub x: isize,
    pub y: isize,
    pub z: isize,
}

impl Vector3D {
    pub fn new(x: isize, y: isize, z: isize) -> Self {
        Self { x, y, z }
    }
}
