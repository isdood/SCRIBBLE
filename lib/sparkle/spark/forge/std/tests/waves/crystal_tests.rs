use spark_std::waves::crystal::Crystal;
use spark_std::waves::WaveError;

#[test]
fn test_crystal_creation() {
    let crystal = Crystal::new([32, 32, 32], 1.0);
    assert!(crystal.spacing > 0.0);
}

#[test]
fn test_crystal_alignment() -> Result<(), WaveError> {
    let crystal = Crystal::new([32, 32, 32], 1.0);
    crystal.align(&[0.0, 0.0, 0.0])?;
    assert!(crystal.align(&[1000.0, 0.0, 0.0]).is_err());
    Ok(())
}

#[test]
fn test_crystal_focus() -> Result<(), WaveError> {
    let crystal = Crystal::new([32, 32, 32], 1.0);
    crystal.focus()?;
    Ok(())
}

#[test]
fn test_crystal_enhancement() -> Result<(), WaveError> {
    let mut crystal = Crystal::new([32, 32, 32], 1.0);
    crystal.enhance(2.0)?;
    assert!(crystal.enhance(-1.0).is_err());
    Ok(())
}

#[test]
fn test_crystal_resonance() -> Result<(), WaveError> {
    let mut crystal = Crystal::new([32, 32, 32], 1.0);
    crystal.tune_resonance(1.0e9)?;
    assert!(crystal.tune_resonance(-1.0).is_err());
    Ok(())
}

#[test]
fn test_crystal_optimization() -> Result<(), WaveError> {
    let mut crystal = Crystal::new([32, 32, 32], 0.05);
    crystal.optimize_alignment()?;
    assert!(crystal.spacing >= 0.1);
    Ok(())
}

#[test]
fn test_crystal_simd() -> Result<(), WaveError> {
    let mut crystal = Crystal::new([32, 32, 32], 1.0);
    crystal.vectorize()?;
    Ok(())
}

#[test]
fn test_crystal_gpu() -> Result<(), WaveError> {
    let mut crystal = Crystal::new([32, 32, 32], 1.0);
    crystal.gpu_optimize()?;
    Ok(())
}

#[test]
fn test_crystal_merge() -> Result<(), WaveError> {
    let mut crystal1 = Crystal::new([32, 32, 32], 1.0);
    let crystal2 = Crystal::new([32, 32, 32], 1.0);
    let crystal3 = Crystal::new([64, 64, 64], 1.0);

    crystal1.merge(&crystal2)?;
    assert!(crystal1.merge(&crystal3).is_err());
    Ok(())
}
