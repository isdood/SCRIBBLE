mod crystal;

pub use crystal::ShardedLattice as CrystalLattice;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_crystal_lattice() {
        let points = vec![[0.0, 0.0, 0.0], [1.0, 1.0, 1.0]];
        let lattice = CrystalLattice::new(points);
        assert!(lattice.calculate_energy() > 0.0);
    }
}
