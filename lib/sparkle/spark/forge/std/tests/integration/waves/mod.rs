//! Integration tests for waves module

use spark_std::waves::{Wave, WaveResult};
use spark_std::waves::crystal::Crystal;
use crate::integration::util::{setup_optimized_wave, setup_sync_waves};

#[test]
fn test_wave_integration() -> WaveResult<()> {
    let wave = setup_optimized_wave(&[1, 2, 3, 4, 5])?;
    let transmitted = wave.transmit(&[0.0, 0.0, 10.0])?;
    assert_eq!(transmitted, 5);
    Ok(())
}

#[test]
fn test_crystal_integration() -> WaveResult<()> {
    let mut crystal = Crystal::new([32, 32, 32], 1.0);

    // Test basic operations
    crystal.align(&[0.0, 0.0, 0.0])?;
    crystal.focus()?;
    crystal.enhance(2.0)?;
    crystal.tune_resonance(1.0e9)?;

    // Test optimizations
    crystal.optimize_alignment()?;
    crystal.vectorize()?;
    crystal.gpu_optimize()?;

    // Test merging with compatible crystal
    let mut crystal2 = Crystal::new([32, 32, 32], 1.0);
    crystal2.tune_resonance(1.0e9)?;
    crystal.merge(&crystal2)?;

    Ok(())
}

#[test]
fn test_wave_sync_compatibility() -> WaveResult<()> {
    // Create two waves with same initial configuration
    let (wave1, wave2) = setup_sync_waves(&[1, 2, 3], &[4, 5, 6])?;

    // Test synchronization after proper setup
    wave1.sync_parallel(&[&wave2])?;
    Ok(())
}

#[test]
fn test_wave_optimization_order() -> WaveResult<()> {
    let wave = Wave::new(&[1, 2, 3]);

    // Test optimization order with proper spacing adjustment
    wave.optimize_hpc()?;
    wave.transmit(&[0.0, 0.0, 10.0])?;

    wave.enable_simd()?;
    wave.transmit(&[0.0, 0.0, 10.0])?;

    wave.enable_gpu()?;
    wave.transmit(&[0.0, 0.0, 10.0])?;

    Ok(())
}
