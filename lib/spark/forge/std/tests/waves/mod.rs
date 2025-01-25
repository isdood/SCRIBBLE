//! Integration tests for waves module

use spark_std::waves::{Wave, WaveResult};

#[test]
fn test_wave_integration() -> WaveResult<()> {
    let data = [1, 2, 3, 4, 5];
    let wave = Wave::new(&data);

    // Test basic transmission
    let transmitted = wave.transmit(&[0.0, 0.0, 10.0])?;
    assert_eq!(transmitted, data.len());

    // Test wave optimization
    wave.optimize_hpc()?;
    wave.enable_simd()?;
    wave.enable_gpu()?;

    // Test parallel synchronization
    let wave2 = Wave::new(&[6, 7, 8]);
    wave.sync_parallel(&[&wave2])?;

    Ok(())
}
