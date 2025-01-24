#[cfg(test)]
mod tests {
    use super::super::{Crystal, Geode};
    use std::time::Duration;

    #[test]
    fn test_crystal_creation() {
        let crystal = Crystal::new("test_formation");
        assert!((crystal.facet_strength - 0.93).abs() < 1e-10);
    }

    #[test]
    fn test_geode_creation() {
        let geode = Geode::new("test_suite");
        assert!((geode.lattice_stability - 0.87).abs() < 1e-10);
    }
}
