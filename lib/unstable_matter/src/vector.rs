use core::cell::UnsafeCell;
use core::ops::{Add, Sub, Mul};
use crate::align::{AlignedSpace, vector_align};
use crate::Helium;
use crate::cube::Box; // Import custom Box
use crate::constants::{MESH_TIMESTAMP, PLANCK_LENGTH, VECTOR_QUANTUM_STATE, QUANTUM_THRESHOLD, LIGHT_SPEED};

/// Vector3D for 3-dimensional vectors
#[derive(Debug)]
pub struct Vector3D<T> {
    pub x: T,
    pub y: T,
    pub z: T,
    timestamp: UnsafeCell<usize>,
    aligned_space: Box<AlignedSpace>, // Use Box to break the recursive type
}

// Manual implementation for Clone
impl<T: Clone> Clone for Vector3D<T> {
    fn clone(&self) -> Self {
        Self {
            x: self.x.clone(),
            y: self.y.clone(),
            z: self.z.clone(),
            timestamp: UnsafeCell::new(unsafe { *self.timestamp.get() }),
            aligned_space: self.aligned_space.clone(),
        }
    }
}

// Manual implementation for PartialEq
impl<T: PartialEq> PartialEq for Vector3D<T> {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x
        && self.y == other.y
        && self.z == other.z
        && unsafe { *self.timestamp.get() } == unsafe { *other.timestamp.get() }
        && self.aligned_space == other.aligned_space
    }
}

impl<T> Vector3D<T>
where
T: PartialEq + Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Copy + Default,
{
    pub fn new(x: T, y: T, z: T) -> Self {
        let alignment = vector_align();
        let aligned_space = Box::new(AlignedSpace::new(
            MESH_TIMESTAMP,
            core::mem::size_of::<T>() * 3,
                                                       alignment,
        ));

        Self {
            x,
            y,
            z,
            timestamp: UnsafeCell::new(MESH_TIMESTAMP),
            aligned_space,
        }
    }

    pub fn get_timestamp(&self) -> usize {
        unsafe { *self.timestamp.get() }
    }

    pub fn update_timestamp(&mut self) {
        unsafe { *self.timestamp.get() = MESH_TIMESTAMP; }
        self.aligned_space.reset_coherence();
    }

    pub fn get_coherence(&self) -> f64 {
        self.aligned_space.get_coherence()
    }
}

// Implement Vector3D<f64> specific operations
impl Vector3D<f64> {
    pub fn magnitude(&mut self) -> f64 {
        self.update_timestamp();
        self.aligned_space.decay_coherence();
        libm::sqrt(self.x * self.x + self.y * self.y + self.z * self.z)
    }

    pub fn normalize(&mut self) -> Self {
        let mag = self.magnitude();
        if mag == 0.0 {
            return self.clone();
        }
        let mut result = self.clone();
        result.x /= mag;
        result.y /= mag;
        result.z /= mag;
        result.update_timestamp();
        result
    }

    pub fn dot(&mut self, other: &mut Self) -> f64 {
        self.update_timestamp();
        other.update_timestamp();
        self.aligned_space.decay_coherence();
        other.aligned_space.decay_coherence();
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross(&mut self, other: &mut Self) -> Self {
        let alignment = vector_align();
        let mut result = Self {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
            timestamp: UnsafeCell::new(MESH_TIMESTAMP),
            aligned_space: Box::new(AlignedSpace::new(
                MESH_TIMESTAMP,
                core::mem::size_of::<f64>() * 3,
                                                      alignment,
            )),
        };
        result.update_timestamp();
        result.aligned_space.reset_coherence();
        result
    }

    pub fn quantum_distance(&mut self, other: &mut Self) -> (f64, f64) {
        self.update_timestamp();
        other.update_timestamp();

        let distance = {
            let dx = self.x - other.x;
            let dy = self.y - other.y;
            let dz = self.z - other.z;
            libm::sqrt(dx * dx + dy * dy + dz * dz)
        };

        let space_coherence = (self.aligned_space.get_coherence()
        + other.aligned_space.get_coherence())
        / 2.0;

        let t1 = self.get_timestamp();
        let t2 = other.get_timestamp();
        let time_diff = if t1 > t2 { t1 - t2 } else { t2 - t1 } as f64;

        let time_coherence = (VECTOR_QUANTUM_STATE as f64)
        / (time_diff + VECTOR_QUANTUM_STATE as f64);

        let coherence = (space_coherence + time_coherence) / 2.0;
        (distance, coherence)
    }

    pub fn is_quantum(&self) -> bool {
        let mag = self.x * self.x + self.y * self.y + self.z * self.z;
        mag < PLANCK_LENGTH * PLANCK_LENGTH && self.aligned_space.is_quantum_stable()
    }

    pub fn quantize(&mut self) -> Self {
        if !self.is_quantum() {
            return self.clone();
        }

        let alignment = vector_align();
        let mut result = Self {
            x: libm::floor(self.x / PLANCK_LENGTH + 0.5) * PLANCK_LENGTH,
            y: libm::floor(self.y / PLANCK_LENGTH + 0.5) * PLANCK_LENGTH,
            z: libm::floor(self.z / PLANCK_LENGTH + 0.5) * PLANCK_LENGTH,
            timestamp: UnsafeCell::new(MESH_TIMESTAMP),
            aligned_space: Box::new(AlignedSpace::new(
                MESH_TIMESTAMP,
                core::mem::size_of::<f64>() * 3,
                                                      alignment,
            )),
        };
        result.update_timestamp();
        result.aligned_space.reset_coherence();
        result
    }

    pub fn quantum_coherence(&self, other: &Self) -> f64 {
        let space_coherence = (self.aligned_space.get_coherence()
        + other.aligned_space.get_coherence())
        / 2.0;

        let t1 = self.get_timestamp();
        let t2 = other.get_timestamp();
        let time_diff = if t1 > t2 { t1 - t2 } else { t2 - t1 } as f64;

        let time_coherence = (VECTOR_QUANTUM_STATE as f64)
        / (time_diff + VECTOR_QUANTUM_STATE as f64);

        (space_coherence + time_coherence) / 2.0
    }
}

// Implement standard arithmetic operations
impl<T: Copy + Add<Output = T>> Add for Vector3D<T> {
    type Output = Self;

    fn add(mut self, other: Self) -> Self {
        let mut result = Self::new(
            self.x + other.x,
            self.y + other.y,
            self.z + other.z,
        );
        result.update_timestamp();
        result
    }
}

impl<T: Copy + Sub<Output = T>> Sub for Vector3D<T> {
    type Output = Self;

    fn sub(mut self, other: Self) -> Self {
        let mut result = Self::new(
            self.x - other.x,
            self.y - other.y,
            self.z - other.z,
        );
        result.update_timestamp();
        result
    }
}

/// Standard operations for Vector4D
#[derive(Debug, Clone)]
pub struct Vector4D<T> {
    pub t: T,
    pub x: T,
    pub y: T,
    pub z: T,
    aligned_space: Box<AlignedSpace>, // Use Box to break the recursive type
}

impl<T> Vector4D<T>
where
T: PartialEq + Add<Output = T> + Mul<Output = T> + Copy + Default,
{
    pub fn new(t: T, x: T, y: T, z: T) -> Self {
        let alignment = vector_align();
        let aligned_space = Box::new(AlignedSpace::new(
            MESH_TIMESTAMP,
            core::mem::size_of::<T>() * 4,
                                                       alignment,
        ));

        Self { t, x, y, z, aligned_space }
    }

    pub fn inner_product(&mut self, other: &mut Self) -> T
    where
    T: Sub<Output = T>,
    {
        self.aligned_space.decay_coherence();
        other.aligned_space.decay_coherence();

        let spatial = self.x * other.x + self.y * other.y + self.z * other.z;
        spatial - (self.t * other.t) // Note the minus sign for time component
    }

    pub fn interval_squared(&mut self) -> T
    where
    T: Sub<Output = T>,
    {
        self.aligned_space.decay_coherence();
        self.inner_product(self)
    }

    pub fn get_coherence(&self) -> f64 {
        self.aligned_space.get_coherence()
    }

    pub fn is_quantum_stable(&self) -> bool {
        self.aligned_space.is_quantum_stable()
    }
}

/// Specialized implementation for f64 vectors
impl Vector4D<f64> {
    pub fn proper_time(&mut self) -> f64 {
        self.aligned_space.decay_coherence();
        let interval = self.interval_squared();
        if interval < -PLANCK_LENGTH {
            libm::sqrt(-interval)
        } else {
            0.0
        }
    }

    pub fn boost_x(&mut self, beta: f64) -> Self {
        assert!(beta.abs() < 1.0, "Speed must be less than light speed");
        let gamma = 1.0 / libm::sqrt(1.0 - beta * beta);

        let alignment = vector_align();
        let aligned_space = Box::new(AlignedSpace::new(
            MESH_TIMESTAMP,
            core::mem::size_of::<f64>() * 4,
                                                       alignment,
        ));

        let mut result = Self {
            t: gamma * (self.t - beta * self.x / LIGHT_SPEED),
            x: gamma * (self.x - beta * self.t * LIGHT_SPEED),
            y: self.y,
            z: self.z,
            aligned_space,
        };
        result.aligned_space.reset_coherence();
        result
    }

    pub fn is_timelike(&mut self) -> bool {
        self.aligned_space.decay_coherence();
        self.interval_squared() < -QUANTUM_THRESHOLD
    }

    pub fn is_spacelike(&mut self) -> bool {
        self.aligned_space.decay_coherence();
        self.interval_squared() > QUANTUM_THRESHOLD
    }

    pub fn is_null(&mut self) -> bool {
        self.aligned_space.decay_coherence();
        self.interval_squared().abs() < QUANTUM_THRESHOLD
    }

    pub fn to_quantum(&self) -> Helium<Self> {
        self.aligned_space.reset_coherence();
        Helium::new(self.clone())
    }

    pub fn spatial_part(&mut self) -> Vector3D<f64> {
        self.aligned_space.decay_coherence();
        Vector3D::new(self.x, self.y, self.z)
    }

    pub fn magnitude(&mut self) -> f64 {
        self.aligned_space.decay_coherence();
        let squared = self.x * self.x + self.y * self.y +
        self.z * self.z - self.t * self.t * LIGHT_SPEED * LIGHT_SPEED;
        if squared.abs() < PLANCK_LENGTH * PLANCK_LENGTH {
            PLANCK_LENGTH
        } else if squared < 0.0 {
            0.0
        } else {
            libm::sqrt(squared)
        }
    }

    pub fn is_quantum(&mut self) -> bool {
        self.magnitude() < PLANCK_LENGTH && self.aligned_space.is_quantum_stable()
    }

    pub fn quantize(&mut self) -> Self {
        if !self.is_quantum() {
            return self.clone();
        }

        let alignment = vector_align();
        let aligned_space = Box::new(AlignedSpace::new(
            MESH_TIMESTAMP,
            core::mem::size_of::<f64>() * 4,
                                                       alignment,
        ));

        let mut result = Self {
            t: libm::floor(self.t / PLANCK_LENGTH + 0.5) * PLANCK_LENGTH,
            x: libm::floor(self.x / PLANCK_LENGTH + 0.5) * PLANCK_LENGTH,
            y: libm::floor(self.y / PLANCK_LENGTH + 0.5) * PLANCK_LENGTH,
            z: libm::floor(self.z / PLANCK_LENGTH + 0.5) * PLANCK_LENGTH,
            aligned_space,
        };
        result.aligned_space.reset_coherence();
        result
    }

    pub fn quantum_coherence(&self, other: &Self) -> f64 {
        (self.aligned_space.get_coherence()
        + other.aligned_space.get_coherence())
        / 2.0
    }
}

/// Standard arithmetic operations
impl<T: PartialEq + Add<Output = T>> Add for Vector4D<T> {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            t: self.t + other.t,
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
            aligned_space: self.aligned_space.clone(),
        }
    }
}

impl<T: PartialEq + Sub<Output = T>> Sub for Vector4D<T> {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            t: self.t - other.t,
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
            aligned_space: self.aligned_space.clone(),
        }
    }
}

impl<T: PartialEq + Mul<Output = T> + Copy> Mul<T> for Vector4D<T> {
    type Output = Self;

    fn mul(self, scalar: T) -> Self {
        Self {
            t: self.t * scalar,
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar,
            aligned_space: self.aligned_space.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vector3d_creation() {
        let v = Vector3D::new(1.0, 2.0, 3.0);
        assert_eq!(v.x, 1.0);
        assert_eq!(v.y, 2.0);
        assert_eq!(v.z, 3.0);
    }

    #[test]
    fn test_vector4d_creation() {
        let v = Vector4D::new(1.0, 2.0, 3.0, 4.0);
        assert_eq!(v.t, 1.0);
        assert_eq!(v.x, 2.0);
        assert_eq!(v.y, 3.0);
        assert_eq!(v.z, 4.0);
    }

    #[test]
    fn test_vector3d_magnitude() {
        let mut v = Vector3D::new(1.0, 2.0, 2.0);
        assert_eq!(v.magnitude(), 3.0);
    }

    #[test]
    fn test_vector4d_proper_time() {
        let mut v = Vector4D::new(1.0, 2.0, 2.0, 2.0);
        assert!(v.proper_time() > 0.0);
    }
}
