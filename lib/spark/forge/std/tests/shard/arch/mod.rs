use spark_std::shard::arch::{Shard, Architecture, CpuFeature};

#[test]
fn test_shard_creation() {
    let shard = Shard::new();
    assert!(matches!(shard.architecture(),
        Architecture::X86_64 |
        Architecture::AArch64 |
        Architecture::RISCV64 |
        Architecture::Unknown
    ));
}

#[test]
fn test_feature_detection() {
    let shard = Shard::new();

    // Test for architecture-specific features
    #[cfg(target_arch = "x86_64")]
    {
        if std::is_x86_feature_detected!("avx2") {
            assert!(shard.has_feature(CpuFeature::AVX2));
        }
    }

    #[cfg(target_arch = "aarch64")]
    {
        assert!(shard.has_feature(CpuFeature::NEON));
    }
}

#[test]
fn test_dispatch_path() {
    let shard = Shard::new();
    let path = shard.get_dispatch_path();

    assert!(!path.is_empty());
    assert!(path.contains(match shard.architecture() {
        Architecture::X86_64 => "x86_64",
        Architecture::AArch64 => "aarch64",
        Architecture::RISCV64 => "riscv64",
        Architecture::Unknown => "generic",
    }));
}

#[test]
fn test_platform_info() {
    let shard = Shard::new();
    let arch = shard.architecture();

    match arch {
        Architecture::X86_64 => assert!(cfg!(target_arch = "x86_64")),
        Architecture::AArch64 => assert!(cfg!(target_arch = "aarch64")),
        Architecture::RISCV64 => assert!(cfg!(target_arch = "riscv64")),
        Architecture::Unknown => (),
    }
}
