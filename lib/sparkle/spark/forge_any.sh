#!/bin/bash

# Spark Any Module Setup Script
# Author: isdood
# Created: 2025-01-25 17:58:29 UTC
# Repository: isdood/scribble
# Description: Sets up Spark's crystal-space type reflection system

PURPLE='\033[0;35m'
NC='\033[0m'

print_purple() {
    echo -e "${PURPLE}$1${NC}"
}

create_directory_structure() {
    print_purple "🔮 Creating Spark Any Module structure..."
    mkdir -p forge/std/src/any
    mkdir -p forge/std/tests/any
}

setup_any_module() {
    # Update lib.rs with feature flag
    cat > forge/std/src/lib.rs << 'EOL'
//! Spark Standard Library - Where Magic Begins ✨

#![feature(const_type_name)]

pub mod math;
pub mod types;
pub mod align;
pub mod any;

pub use types::*;
pub use math::operations;
pub use align::space;
EOL

    cat > forge/std/src/any/mod.rs << 'EOL'
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
EOL

    cat > forge/std/src/any/type_id.rs << 'EOL'
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
EOL

    cat > forge/std/src/any/reflection.rs << 'EOL'
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
EOL

    cat > forge/std/tests/any/mod.rs << 'EOL'
use spark_std::any::{CrystalAny, CrystalType, TypeId, TypeInfo, CrystalAnyRef};
use spark_std::align::Alignment;
use std::sync::OnceLock;

#[derive(Debug)]
struct Crystal {
    value: i32
}

#[derive(Debug)]
struct OtherType {
    #[allow(dead_code)]
    name: String
}

impl CrystalType for Crystal {
    fn type_info() -> &'static TypeInfo {
        static INFO: OnceLock<TypeInfo> = OnceLock::new();
        INFO.get_or_init(|| TypeInfo::new::<Crystal>(Alignment::Crystal16))
    }
}

impl CrystalAny for Crystal {
    fn type_id(&self) -> TypeId {
        TypeId::of::<Crystal>()
    }

    fn type_info(&self) -> &'static TypeInfo {
        <Crystal as CrystalType>::type_info()
    }
}

impl CrystalType for OtherType {
    fn type_info() -> &'static TypeInfo {
        static INFO: OnceLock<TypeInfo> = OnceLock::new();
        INFO.get_or_init(|| TypeInfo::new::<OtherType>(Alignment::Vector32))
    }
}

impl CrystalAny for OtherType {
    fn type_id(&self) -> TypeId {
        TypeId::of::<OtherType>()
    }

    fn type_info(&self) -> &'static TypeInfo {
        <OtherType as CrystalType>::type_info()
    }
}

#[test]
fn test_crystal_type_reflection() {
    let crystal = Crystal { value: 42 };
    let any_ref = CrystalAnyRef::new(&crystal);

    // Type name will include the module path
    assert!(any_ref.type_info().name().contains("Crystal"));
    assert_eq!(any_ref.type_info().alignment(), Alignment::Crystal16);
    assert!(any_ref.type_info().size() >= std::mem::size_of::<i32>());
}

#[test]
fn test_crystal_downcasting() {
    let crystal = Crystal { value: 42 };
    let any_ref = CrystalAnyRef::new(&crystal);

    let downcasted = any_ref.downcast_ref::<Crystal>().unwrap();
    assert_eq!(downcasted.value, 42);

    // Try downcasting to wrong type
    let other = OtherType { name: "test".to_string() };
    let other_ref = CrystalAnyRef::new(&other);
    assert!(other_ref.downcast_ref::<Crystal>().is_none());
}

#[test]
fn test_type_safety() {
    let crystal = Crystal { value: 42 };
    let any_ref = CrystalAnyRef::new(&crystal);

    // Verify type name contains Crystal
    assert!(any_ref.type_info().name().contains("Crystal"));

    // Verify alignment
    assert_eq!(any_ref.type_info().alignment(), Alignment::Crystal16);

    // Verify type ID consistency
    assert_eq!(any_ref.type_id(), TypeId::of::<Crystal>());
}

#[test]
fn test_different_alignments() {
    let crystal = Crystal { value: 42 };
    let other = OtherType { name: "test".to_string() };

    let crystal_ref = CrystalAnyRef::new(&crystal);
    let other_ref = CrystalAnyRef::new(&other);

    assert_eq!(crystal_ref.type_info().alignment(), Alignment::Crystal16);
    assert_eq!(other_ref.type_info().alignment(), Alignment::Vector32);
}

#[test]
fn test_type_name_paths() {
    let crystal = Crystal { value: 42 };
    let any_ref = CrystalAnyRef::new(&crystal);

    // The full type name includes the module path
    let type_name = any_ref.type_info().name();
    assert!(type_name.contains("primitive_tests"));
    assert!(type_name.contains("any"));
    assert!(type_name.contains("Crystal"));
}
EOL

    # Update main test file to include any module tests
    if ! grep -q "mod any;" forge/std/tests/primitive_tests.rs; then
        echo -e "\nmod any;" >> forge/std/tests/primitive_tests.rs
    fi

    print_purple "✓ Created any module files"
}

main() {
    print_purple "🔮 Creating Spark Any Module..."
    create_directory_structure
    setup_any_module
    print_purple "✨ Any module created with crystal-space type reflection!

Features:
- Crystal-space type identification
- Runtime type reflection
- Safe type downcasting
- Crystal alignment integration

Type Reflection:
- TypeId for unique type identification
- TypeInfo for runtime type information
- CrystalAny for dynamic typing
- CrystalType for crystal-space types

Run 'cd forge/std && cargo test' to verify the type reflection system!"
}

main
