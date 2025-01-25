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
