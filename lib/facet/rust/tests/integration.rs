//! Facet Integration Tests
//! Author: @isdood
//! Created: 2025-01-21 13:39:58 UTC

use std::time::Duration;

use facet_rust::{
    bridge::BridgeConfig,
    compute::ComputeEngine,
    crystal::CrystalLattice,
    resonance::ResonanceState,
    types::{ComputeConfig, Operation, Pattern, Result, Vec3d},
};

use proptest::prelude::*;
use test_case::test_case;

const TEST_TIMEOUT: Duration = Duration::from_secs(5);

/// Test environment setup
struct TestEnv {
    bridge_config: BridgeConfig,
    compute_config: ComputeConfig,
    crystal: CrystalLattice,
    resonance: ResonanceState,
    engine: ComputeEngine,
}

impl TestEnv {
    fn new() -> Self {
        let bridge_config = BridgeConfig {
            allocator_type: 0,
            thread_count: 2,
            enable_simd: true,
            debug_level: 1,
            _padding: [0; 4],
        };

        let compute_config = ComputeConfig {
            min_clarity: 0.85,
            min_resonance: 0.85,
            enable_simd: true,
            enable_whimsy: true,
            pattern: Pattern::Basic,
        };

        Self {
            bridge_config,
            compute_config,
            crystal: CrystalLattice::new(),
            resonance: ResonanceState::new(),
            engine: ComputeEngine::new(),
        }
    }
}

#[test]
fn test_basic_computation() {
    let env = TestEnv::new();

    let input = Vec3d::new(1.0, 2.0, 3.0);
    let result = env.engine.compute_vector(
        Operation::Add,
        input,
        &env.crystal,
        &env.resonance,
        &env.compute_config,
    );

    assert!(result.is_ok());
    let output = result.unwrap();
    assert!(output.magnitude() > 0.0);
    assert!(env.crystal.get_clarity() >= env.compute_config.min_clarity);
}

#[test_case(Pattern::Basic => 1.0; "basic pattern efficiency")]
#[test_case(Pattern::Resonant => 1.2; "resonant pattern efficiency")]
#[test_case(Pattern::Clear => 1.15; "clear pattern efficiency")]
#[test_case(Pattern::Quantum => 1.3; "quantum pattern efficiency")]
#[test_case(Pattern::Whimsy => 1.25; "whimsy pattern efficiency")]
fn test_pattern_efficiency(pattern: Pattern) -> f64 {
    pattern.efficiency()
}

#[test]
fn test_crystal_resonance() {
    let env = TestEnv::new();

    // Perform multiple computations to build resonance
    for i in 0..5 {
        let input = Vec3d::new(i as f64, i as f64, i as f64);
        let result = env.engine.compute_vector(
            Operation::Multiply,
            input,
            &env.crystal,
            &env.resonance,
            &env.compute_config,
        );
        assert!(result.is_ok());
    }

    // Check resonance has built up
    assert!(env.resonance.get_level() > 1.0);
}

#[test]
fn test_error_handling() {
    let env = TestEnv::new();

    // Test division by zero
    let input = Vec3d::new(1.0, 1.0, 1.0);
    let zero = Vec3d::default();

    let result = env.engine.compute_vector(
        Operation::Divide,
        input,
        &env.crystal,
        &env.resonance,
        &env.compute_config,
    );

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), zero);
}

proptest! {
    #[test]
    fn test_random_computations(
        x in -1000.0..1000.0f64,
        y in -1000.0..1000.0f64,
        z in -1000.0..1000.0f64
    ) {
        let env = TestEnv::new();
        let input = Vec3d::new(x, y, z);

        let result = env.engine.compute_vector(
            Operation::Add,
            input,
            &env.crystal,
            &env.resonance,
            &env.compute_config,
        );

        prop_assert!(result.is_ok());
        let output = result.unwrap();
        prop_assert!(output.magnitude().is_finite());
    }
}

#[tokio::test]
async fn test_async_computation() {
    let env = TestEnv::new();

    let computations = vec![
        Vec3d::new(1.0, 0.0, 0.0),
        Vec3d::new(0.0, 1.0, 0.0),
        Vec3d::new(0.0, 0.0, 1.0),
    ];

    let mut handles = vec![];

    for input in computations {
        let crystal = env.crystal.clone();
        let resonance = env.resonance.clone();
        let engine = env.engine.clone();
        let config = env.compute_config.clone();

        let handle = tokio::spawn(async move {
            engine.compute_vector(
                Operation::Add,
                input,
                &crystal,
                &resonance,
                &config,
            )
        });

        handles.push(handle);
    }

    for handle in handles {
        let result = tokio::time::timeout(TEST_TIMEOUT, handle).await
        .expect("computation timed out")
        .expect("task failed")
        .expect("computation failed");

        assert!(result.magnitude() > 0.0);
    }
}

#[test]
fn test_whimsy_effects() {
    let mut env = TestEnv::new();
    env.compute_config.enable_whimsy = true;
    env.compute_config.pattern = Pattern::Whimsy;

    let input = Vec3d::new(1.0, 1.0, 1.0);
    let result = env.engine.compute_vector(
        Operation::Multiply,
        input,
        &env.crystal,
        &env.resonance,
        &env.compute_config,
    );

    assert!(result.is_ok());
    let output = result.unwrap();
    assert!(output.magnitude() > input.magnitude());
}

#[test]
fn test_crystal_clarity_degradation() {
    let env = TestEnv::new();
    let initial_clarity = env.crystal.get_clarity();

    // Perform many computations
    for _ in 0..100 {
        let input = Vec3d::new(1.0, 1.0, 1.0);
        let _ = env.engine.compute_vector(
            Operation::Add,
            input,
            &env.crystal,
            &env.resonance,
            &env.compute_config,
        );
    }

    // Check clarity has decreased but remains above minimum
    let final_clarity = env.crystal.get_clarity();
    assert!(final_clarity < initial_clarity);
    assert!(final_clarity >= env.compute_config.min_clarity);
}

#[test]
fn test_simd_optimization() {
    let mut env = TestEnv::new();

    // Test with SIMD enabled
    env.compute_config.enable_simd = true;
    let simd_start = std::time::Instant::now();
    for _ in 0..1000 {
        let input = Vec3d::new(1.0, 1.0, 1.0);
        let _ = env.engine.compute_vector(
            Operation::Add,
            input,
            &env.crystal,
            &env.resonance,
            &env.compute_config,
        );
    }
    let simd_duration = simd_start.elapsed();

    // Test with SIMD disabled
    env.compute_config.enable_simd = false;
    let no_simd_start = std::time::Instant::now();
    for _ in 0..1000 {
        let input = Vec3d::new(1.0, 1.0, 1.0);
        let _ = env.engine.compute_vector(
            Operation::Add,
            input,
            &env.crystal,
            &env.resonance,
            &env.compute_config,
        );
    }
    let no_simd_duration = no_simd_start.elapsed();

    // SIMD should be faster
    assert!(simd_duration < no_simd_duration);
}
