//! Crystal-Space Type Reflection

use std::any::type_name;
use super::TypeId;
use crate::align::Alignment;

/// Type information for crystal-space reflection
#[derive(Debug)]
pub struct TypeInfo {
    name: &'static str,
    type_id: TypeId,
    alignment: Alignment,
    size: usize,
}

impl TypeInfo {
    /// Creates new type information
    pub fn new<T: 'static>(alignment: Alignment) -> TypeInfo {
        TypeInfo {
            name: type_name::<T>(),
            type_id: TypeId::of::<T>(),
            alignment,
            size: std::mem::size_of::<T>(),
        }
    }

    /// Returns the type's name
    #[inline]
    pub const fn name(&self) -> &'static str {
        self.name
    }

    /// Returns the type's ID
    #[inline]
    pub const fn type_id(&self) -> TypeId {
        self.type_id
    }

    /// Returns the type's alignment requirement
    #[inline]
    pub const fn alignment(&self) -> Alignment {
        self.alignment
    }

    /// Returns the size of the type
    #[inline]
    pub const fn size(&self) -> usize {
        self.size
    }
}

/// A trait for types that can be used in crystal-space
pub trait CrystalType: 'static {
    /// Returns the type information for this type
    fn type_info() -> &'static TypeInfo;
}
