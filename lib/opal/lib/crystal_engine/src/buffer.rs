use harmony_core::Vector3D;
use magicmath::MeshValue;

pub struct HarmonicBuffer<T: MeshValue> {
    data: Vec<T>,
    size: usize,
    phi_aligned_size: usize,
}

impl<T: MeshValue> HarmonicBuffer<T> {
    pub fn new(size: usize) -> Self {
        let phi = 1.618033988749895;
        let phi_aligned_size = (size as f64 * phi).ceil() as usize;

        Self {
            data: Vec::with_capacity(phi_aligned_size),
            size,
            phi_aligned_size,
        }
    }
}
