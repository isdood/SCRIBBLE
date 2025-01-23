pub struct CrystalField {
    lattice_size: (usize, usize, usize),
}

impl CrystalField {
    pub fn new() -> Self {
        Self {
            lattice_size: (64, 64, 64),
        }
    }
}
