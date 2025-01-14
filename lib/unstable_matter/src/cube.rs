use core::sync::atomic::Ordering;
use core::ptr::NonNull;
use core::alloc::Layout;
use crate::align::{AlignedSpace, vector_align};
use crate::Helium;
use crate::vector::Vector4D;
use crate::constants::{MESH_TIMESTAMP, PLANCK_LENGTH, VECTOR_QUANTUM_STATE, QUANTUM_THRESHOLD};

#[derive(Debug)]
pub struct Box<T> {
    ptr: NonNull<T>,
    layout: Layout,
}

impl<T> Box<T> {
    pub fn new(value: T) -> Self {
        let layout = Layout::new::<T>();
        unsafe {
            let ptr = AlignedSpace::alloc(layout) as *mut T;
            if ptr.is_null() {
                panic!("Allocation failed");
            }
            ptr.write(value);
            Box {
                ptr: NonNull::new(ptr).unwrap(),
                layout,
            }
        }
    }
}

impl<T> Drop for Box<T> {
    fn drop(&mut self) {
        unsafe {
            core::ptr::drop_in_place(self.ptr.as_ptr());
            AlignedSpace::dealloc(self.ptr.as_ptr() as *mut u8, self.layout);
        }
    }
}

impl<T> core::ops::Deref for Box<T> {
    type Target = T;

    fn deref(&self) -> &T {
        unsafe { self.ptr.as_ref() }
    }
}

impl<T> core::ops::DerefMut for Box<T> {
    fn deref_mut(&mut self) -> &mut T {
        unsafe { self.ptr.as_mut() }
    }
}

impl<T> Clone for Box<T>
where
T: Clone,
{
    fn clone(&self) -> Self {
        Box::new((**self).clone())
    }
}

/// Metric tensor for spacetime calculations
#[derive(Debug)]
pub struct MetricTensor {
    components: [[f64; 4]; 4],
    timestamp: Helium<usize>,
    aligned_space: Box<AlignedSpace>, // Use Box to break the recursive type
}

impl Clone for MetricTensor {
    fn clone(&self) -> Self {
        Self {
            components: self.components,
            timestamp: Helium::new(self.timestamp.quantum_load(Ordering::SeqCst).0),
            aligned_space: self.aligned_space.clone(),
        }
    }
}

impl MetricTensor {
    pub fn minkowski() -> Self {
        let mut components = [[0.0; 4]; 4];
        components[0][0] = -1.0; // Time component
        components[1][1] = 1.0;  // Spatial components
        components[2][2] = 1.0;
        components[3][3] = 1.0;

        let alignment = vector_align();
        let aligned_space = Box::new(AlignedSpace::new(
            MESH_TIMESTAMP,
            core::mem::size_of::<f64>() * 16, // 4x4 matrix
                                                       alignment,
        ));

        Self {
            components,
            timestamp: Helium::new(MESH_TIMESTAMP),
            aligned_space,
        }
    }

    pub fn contract(&mut self, v1: &Vector4D<f64>, v2: &Vector4D<f64>) -> f64 {
        self.timestamp.store(MESH_TIMESTAMP, Ordering::SeqCst);
        self.aligned_space.decay_coherence();

        // Consider coherence of input vectors
        let spatial_coherence = (
            v1.get_coherence() +
            v2.get_coherence() +
            self.aligned_space.get_coherence()
        ) / 3.0;

        let v1_components = [v1.t, v1.x, v1.y, v1.z];
        let v2_components = [v2.t, v2.x, v2.y, v2.z];

        let mut result = 0.0;
        for i in 0..4 {
            for j in 0..4 {
                result += self.components[i][j] * v1_components[i] * v2_components[j];
            }
        }

        if spatial_coherence < QUANTUM_THRESHOLD {
            result = libm::floor(result / PLANCK_LENGTH + 0.5) * PLANCK_LENGTH;
        }

        result
    }

    pub fn get_timestamp(&self) -> usize {
        self.timestamp.quantum_load(Ordering::SeqCst).0
    }

    pub fn get_coherence(&self) -> f64 {
        self.aligned_space.get_coherence()
    }

    pub fn reset_coherence(&mut self) {
        self.aligned_space.reset_coherence();
    }

    pub fn is_quantum_stable(&self) -> bool {
        self.aligned_space.is_quantum_stable()
    }

    pub fn quantum_coherence(&self, other: &Self) -> f64 {
        let spatial_coherence = (
            self.aligned_space.get_coherence() +
            other.aligned_space.get_coherence()
        ) / 2.0;

        let (t1, _) = self.timestamp.quantum_load(Ordering::SeqCst);
        let (t2, _) = other.timestamp.quantum_load(Ordering::SeqCst);
        let time_diff = if t1 > t2 { t1 - t2 } else { t2 - t1 } as f64;
        let temporal_coherence = (VECTOR_QUANTUM_STATE as f64) /
        (time_diff + VECTOR_QUANTUM_STATE as f64);

        (spatial_coherence + temporal_coherence) / 2.0
    }

    pub fn quantize(&mut self) {
        if self.is_quantum_stable() {
            return;
        }

        for i in 0..4 {
            for j in 0..4 {
                self.components[i][j] = libm::floor(
                    self.components[i][j] / PLANCK_LENGTH + 0.5
                ) * PLANCK_LENGTH;
            }
        }

        self.reset_coherence();
        self.timestamp.store(MESH_TIMESTAMP, Ordering::SeqCst);
    }

    pub fn realign(&mut self) {
        self.aligned_space.realign();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::sync::atomic::Ordering;

    const TEST_TIMESTAMP: usize = 1705250993; // 2025-01-14 17:29:53 UTC

    #[test]
    fn test_vector3d_timestamp() {
        let v = Vector3D::new(1.0, 2.0, 3.0);
        assert_eq!(v.get_timestamp(), MESH_TIMESTAMP);
        assert!(v.is_quantum_stable());
        assert_eq!(v.get_coherence(), 1.0);
    }

    #[test]
    fn test_proper_time() {
        let v = Vector4D::new(1.0, 0.0, 0.0, 0.0);
        assert!(v.proper_time() > 0.0);
        assert!(v.is_quantum_stable());

        let quantum_v = Vector4D::new(PLANCK_LENGTH / 2.0, 0.0, 0.0, 0.0);
        assert_eq!(quantum_v.proper_time(), 0.0);
        assert!(quantum_v.is_quantum());
    }

    #[test]
    fn test_boost() {
        let v = Vector4D::new(1.0, 0.5, 0.0, 0.0);
        let boosted = v.boost_x(0.5);
        assert!(boosted.is_timelike());
        assert!(boosted.is_quantum_stable());
        assert_eq!(
            v.interval_squared(),
                   boosted.interval_squared(),
                   "Lorentz invariance violation"
        );
        assert!(boosted.get_coherence() > 0.0);
    }

    #[test]
    fn test_metric_tensor() {
        let metric = MetricTensor::minkowski();
        let v1 = Vector4D::new(1.0, 0.0, 0.0, 0.0);
        let v2 = Vector4D::new(1.0, 0.0, 0.0, 0.0);

        assert_eq!(metric.contract(&v1, &v2), -1.0);
        assert_eq!(metric.get_timestamp(), MESH_TIMESTAMP);
        assert!(metric.is_quantum_stable());
        assert!(metric.get_coherence() > 0.0);
    }

    #[test]
    fn test_quantum_coherence() {
        let mut metric1 = MetricTensor::minkowski();
        let mut metric2 = MetricTensor::minkowski();

        // Test initial coherence
        let coherence = metric1.quantum_coherence(&metric2);
        assert!(coherence > 0.0 && coherence <= 1.0);

        // Test coherence decay
        metric1.aligned_space.decay_coherence();
        let decayed_coherence = metric1.quantum_coherence(&metric2);
        assert!(decayed_coherence < coherence);

        // Test coherence reset
        metric1.reset_coherence();
        let reset_coherence = metric1.quantum_coherence(&metric2);
        assert!(reset_coherence > decayed_coherence);
    }

    #[test]
    fn test_vector3d_quantum_distance() {
        let v1 = Vector3D::new(0.0, 0.0, 0.0);
        let v2 = Vector3D::new(1.0, 0.0, 0.0);

        // Test distance and coherence
        let (distance, coherence) = v1.quantum_distance(&v2);
        assert_eq!(distance, 1.0);
        assert!(coherence > 0.0 && coherence <= 1.0);

        // Test alignment
        assert!(v1.is_quantum_stable());
        assert!(v2.is_quantum_stable());

        // Test memory alignment
        assert_eq!(
            v1.aligned_space.get_alignment().get_value() % VECTOR_ALIGN,
                   0
        );
    }

    #[test]
    fn test_vector3d_operations() {
        let v1 = Vector3D::new(1.0, 0.0, 0.0);
        let v2 = Vector3D::new(0.0, 1.0, 0.0);

        // Test cross product
        let v3 = v1.cross(&v2);
        assert_eq!(v3.z, 1.0);
        assert_eq!(v3.get_timestamp(), MESH_TIMESTAMP);
        assert!(v3.is_quantum_stable());

        // Test coherence after operation
        assert!(v3.get_coherence() > 0.0);

        // Test alignment preservation
        assert_eq!(
            v3.aligned_space.get_alignment().get_value() % VECTOR_ALIGN,
                   0
        );
    }

    #[test]
    fn test_alignment_preservation() {
        let v = Vector3D::new(1.0, 2.0, 3.0);
        let normalized = v.normalize();

        // Test alignment after normalization
        assert!(normalized.is_quantum_stable());
        assert_eq!(
            normalized.aligned_space.get_alignment().get_value() % VECTOR_ALIGN,
                   0
        );

        // Test coherence preservation
        assert!(normalized.get_coherence() > 0.0);
    }

    #[test]
    fn test_quantum_corrections() {
        let small_v = Vector3D::new(
            PLANCK_LENGTH / 2.0,
            PLANCK_LENGTH / 2.0,
            PLANCK_LENGTH / 2.0
        );

        let quantized = small_v.quantize();
        assert!(quantized.is_quantum());
        assert!(quantized.is_quantum_stable());
        assert!(quantized.get_coherence() > 0.0);
    }
}
