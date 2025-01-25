use spark_std::shimmer::{Shimmer, ShimmerError};
use spark_std::shimmer::zig::ZigFnAttrs;
use spark_std::shimmer::julia::JuliaFnAttrs;
use spark_std::shimmer::rust::RustFnAttrs;

#[test]
fn test_shimmer_basic() {
    let shimmer = Shimmer::new();
    assert!(shimmer.get_data().is_none());
}

#[test]
fn test_shimmer_function() {
    let shimmer = Shimmer::new();
    let result = shimmer.get_fn::<fn()>("test_fn");
    assert!(result.is_err());
}

#[test]
fn test_language_attrs() {
    let zig_attrs = ZigFnAttrs {
        is_export: true,
        is_extern: false,
    };
    assert!(zig_attrs.is_export);
    assert!(!zig_attrs.is_extern);

    let julia_attrs = JuliaFnAttrs {
        is_ccall: true,
        return_type: "Int64".to_string(),
    };
    assert!(julia_attrs.is_ccall);
    assert_eq!(julia_attrs.return_type, "Int64");

    let rust_attrs = RustFnAttrs {
        is_unsafe: true,
        is_extern: true,
        abi: "C".to_string(),
    };
    assert!(rust_attrs.is_unsafe);
    assert!(rust_attrs.is_extern);
    assert_eq!(rust_attrs.abi, "C");
}
