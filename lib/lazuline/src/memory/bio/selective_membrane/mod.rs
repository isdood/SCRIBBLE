//! Selective Membrane Cache Implementation
//! Created: 2025-01-22
//! Author: isdood

mod cache;
mod predictor;
mod permeability;

use cache::MembraneCache;
use predictor::AccessPredictor;
use permeability::PermeabilityController;

pub struct SelectiveMembraneCache {
    cache: MembraneCache,
    predictor: AccessPredictor,
    permeability: PermeabilityController,
}

impl SelectiveMembraneCache {
    pub fn new() -> Self {
        // TODO: Implementation
        unimplemented!()
    }
}
