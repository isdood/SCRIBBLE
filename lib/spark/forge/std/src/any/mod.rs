//! Crystal-Space Type Reflection System
//!
//! Provides dynamic typing capabilities optimized for crystal-space computing.

mod type_id;
mod reflection;

pub use type_id::TypeId;
pub use reflection::{CrystalType, TypeInfo};

/// A trait for types that can be dynamically reflected
pub trait CrystalAny: 'static {
    /// Returns the TypeId of the underlying type
    fn type_id(&self) -> TypeId;

    /// Returns type information for runtime reflection
    fn type_info(&self) -> &'static TypeInfo;
}

/// Trait object wrapper for CrystalAny
pub struct CrystalAnyRef<'a>(pub &'a (dyn CrystalAny + 'static));

impl<'a> CrystalAnyRef<'a> {
    /// Creates a new reference to a CrystalAny trait object
    pub fn new<T: CrystalAny>(value: &'a T) -> Self {
        CrystalAnyRef(value)
    }

    /// Attempts to downcast this type to a concrete type
    pub fn downcast_ref<T: CrystalAny>(&self) -> Option<&'a T> {
        if self.0.type_id() == TypeId::of::<T>() {
            // Safety: type_id match guarantees the type is correct
            unsafe { Some(&*(self.0 as *const (dyn CrystalAny) as *const T)) }
        } else {
            None
        }
    }

    /// Gets the underlying type information
    pub fn type_info(&self) -> &'static TypeInfo {
        self.0.type_info()
    }

    /// Gets the type ID of the underlying value
    pub fn type_id(&self) -> TypeId {
        self.0.type_id()
    }
}
