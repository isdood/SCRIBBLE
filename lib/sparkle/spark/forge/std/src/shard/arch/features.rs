//! CPU feature detection and management

/// CPU features relevant for crystal computing
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CpuFeature {
    // x86_64 features
    AVX2,
    AVX512F,
    // AArch64 features
    NEON,
    SVE,
    // RISC-V features
    V,
    // Common features
    FMA,
    SIMD,
}

/// Detects available CPU features
pub fn detect_features() -> Vec<CpuFeature> {
    let mut features = Vec::new();

    #[cfg(target_arch = "x86_64")]
    detect_x86_features(&mut features);

    #[cfg(target_arch = "aarch64")]
    detect_aarch64_features(&mut features);

    #[cfg(target_arch = "riscv64")]
    detect_riscv_features(&mut features);

    features
}

#[cfg(target_arch = "x86_64")]
fn detect_x86_features(features: &mut Vec<CpuFeature>) {
    if std::is_x86_feature_detected!("avx2") {
        features.push(CpuFeature::AVX2);
    }
    if std::is_x86_feature_detected!("avx512f") {
        features.push(CpuFeature::AVX512F);
    }
    if std::is_x86_feature_detected!("fma") {
        features.push(CpuFeature::FMA);
    }
}

#[cfg(target_arch = "aarch64")]
fn detect_aarch64_features(features: &mut Vec<CpuFeature>) {
    features.push(CpuFeature::NEON);
    // SVE detection would go here
    features.push(CpuFeature::SIMD);
}

#[cfg(target_arch = "riscv64")]
fn detect_riscv_features(features: &mut Vec<CpuFeature>) {
    // RISC-V vector extension detection would go here
    features.push(CpuFeature::SIMD);
}
