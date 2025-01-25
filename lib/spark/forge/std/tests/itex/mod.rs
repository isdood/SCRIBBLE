use spark_std::itex::{Itex, ItexResult};

#[test]
fn test_itex_creation() {
    let iter = 0..5;
    let itex = Itex::new(iter);
    assert!(itex.crystal().frequency() > 0.0);
    assert!(itex.flux().intensity() > 0.0);
}

#[test]
fn test_crystal_map() {
    let iter = 0..5;
    let itex = Itex::new(iter);

    let result: Vec<_> = itex
        .crystal_map(|x| x * 2)
        .collect();

    assert_eq!(result, vec![0, 2, 4, 6, 8]);
}

#[test]
fn test_flux_filter() {
    let iter = 0..5;
    let itex = Itex::new(iter);

    let result: Vec<_> = itex
        .flux_filter(|&x| x % 2 == 0)
        .collect();

    assert_eq!(result, vec![0, 2, 4]);
}

#[test]
fn test_quantum_collect() -> ItexResult<()> {
    let iter = 0..5;
    let itex = Itex::new(iter);

    let result: Vec<_> = itex.quantum_collect()?;
    assert_eq!(result, vec![0, 1, 2, 3, 4]);

    Ok(())
}

#[test]
fn test_parallel_process() -> ItexResult<()> {
    let iter = 0..10;
    let itex = Itex::new(iter);

    let result = itex.parallel_process(2, |x| x * 2)?;
    assert_eq!(result.len(), 10);

    Ok(())
}
