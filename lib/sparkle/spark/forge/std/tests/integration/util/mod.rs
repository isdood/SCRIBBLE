//! Test utilities for integration tests

use spark_std::waves::{Wave, WaveResult};

/// Sets up a wave with standard optimizations and spacing
pub fn setup_optimized_wave(data: &[u8]) -> WaveResult<Wave> {
    let wave = Wave::new(data);
    wave.optimize_hpc()?;
    wave.enable_simd()?;
    wave.enable_gpu()?;
    wave.transmit(&[0.0, 0.0, 1.0])?; // Initial transmission to set spacing
    Ok(wave)
}

/// Prepares two waves for synchronization testing with matched settings
pub fn setup_sync_waves(data1: &[u8], data2: &[u8]) -> WaveResult<(Wave, Wave)> {
    let wave1 = Wave::new(data1);
    let wave2 = Wave::new(data2);

    // Configure both waves identically
    for wave in [&wave1, &wave2] {
        wave.optimize_hpc()?;
        wave.enable_simd()?;
        wave.enable_gpu()?;
        wave.transmit(&[0.0, 0.0, 1.0])?; // Ensure same spacing
    }

    Ok((wave1, wave2))
}
