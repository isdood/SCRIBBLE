use spark_std::scribe::{Scribe, ResonancePoint};

#[test]
fn test_resonance_point() {
    let point = ResonancePoint::new(1.0, 2.0, 3.0, 1.0, 0.0);
    assert!(point.intensity() > 0.0);
}

#[test]
fn test_scribe_creation() {
    let scribe = Scribe::new();
    assert_eq!(scribe.resonance().total_intensity(), 0.0);
    assert!(scribe.resonance().lattice_nodes().is_empty());
}

#[test]
fn test_wave_format() {
    let mut scribe = Scribe::new();
    scribe.add_point(0.0, 0.0, 0.0, 1.0, 0.0);
    let result = scribe.format("test");
    assert!(!result.is_empty());
    assert!(!scribe.resonance().lattice_nodes().is_empty());
}

#[test]
fn test_lattice_connections() {
    let mut scribe = Scribe::new();
    scribe.add_point(0.0, 0.0, 0.0, 1.0, 0.0);
    scribe.add_point(1.0, 0.0, 0.0, 1.0, 0.0);

    let nodes = scribe.resonance().lattice_nodes();
    assert_eq!(nodes.len(), 2);
    assert!(!nodes[1].connections().is_empty());
}

#[test]
fn test_resonance_propagation() {
    let mut scribe = Scribe::new();
    scribe.add_point(0.0, 0.0, 0.0, 1.0, 0.0);
    let nodes = scribe.resonance().lattice_nodes();
    assert!(nodes[0].resonance() > 0.0);
}
