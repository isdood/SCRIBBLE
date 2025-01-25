use spark_std::shimmer::{Shimmer, ShimmerError};
use spark_std::shimmer::zig::ZigFnAttrs;
use spark_std::shimmer::julia::JuliaFnAttrs;
use spark_std::shimmer::rust::RustFnAttrs;

#[test]
fn test_shimmer_creation() {
    let shimmer = Shimmer::new();
    assert!(true, "Shimmer instance created successfully");
}

#[test]
fn test_library_loading() {
    let mut shimmer = Shimmer::new();

    #[cfg(unix)]
    let result = shimmer.load("libtest.so");

    #[cfg(windows)]
    let result = shimmer.load("test.dll");

    assert!(result.is_err(), "Loading non-existent library should fail");
}

#[test]
fn test_zig_interface() {
    let shimmer = Shimmer::new();
    let attrs = ZigFnAttrs {
        is_export: true,
        is_extern: true,
    };

    let result = shimmer.zig_fn::<fn()>("test", attrs);
    assert!(result.is_err(), "Unimplemented Zig interface should error");
}

#[test]
fn test_julia_interface() {
    let shimmer = Shimmer::new();
    let attrs = JuliaFnAttrs {
        is_ccall: true,
        return_type: String::from("Cvoid"),
    };

    let result = shimmer.julia_fn::<fn()>("test", attrs);
    assert!(result.is_err(), "Unimplemented Julia interface should error");
}

#[test]
fn test_rust_interface() {
    let shimmer = Shimmer::new();
    let attrs = RustFnAttrs {
        is_unsafe: true,
        is_extern: true,
        abi: String::from("C"),
    };

    let result = shimmer.rust_fn::<fn()>("test", attrs);
    assert!(result.is_err(), "Unimplemented Rust interface should error");
}
