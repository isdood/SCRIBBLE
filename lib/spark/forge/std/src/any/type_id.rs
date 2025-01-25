//! Crystal-Space Type Identification

use std::hash::{Hash, Hasher};
use std::any::TypeId as StdTypeId;

/// A unique identifier for a type in crystal-space
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TypeId {
    std_id: StdTypeId,
    crystal_hash: u64,
}

impl TypeId {
    /// Gets the TypeId of a type
    #[inline]
    pub fn of<T: 'static + ?Sized>() -> TypeId {
        let std_id = StdTypeId::of::<T>();
        let crystal_hash = Self::compute_crystal_hash::<T>();

        TypeId {
            std_id,
            crystal_hash,
        }
    }

    /// Computes a crystal-space optimized hash for a type
    #[inline]
    fn compute_crystal_hash<T: 'static + ?Sized>() -> u64 {
        use std::collections::hash_map::DefaultHasher;

        let mut hasher = DefaultHasher::new();
        std::any::type_name::<T>().hash(&mut hasher);
        hasher.finish()
    }
}
