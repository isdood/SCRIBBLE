use rayon::prelude::*;
use std::sync::Arc;
use parking_lot::RwLock;
use hashbrown::HashMap;

#[derive(Clone)]
pub struct ShardedLattice {
    points: Vec<[f64; 3]>,
    shards: Vec<LatticeRegion>,
    cache: Arc<RwLock<HashMap<u64, f64>>>,
    modification_counter: u64,
}

#[derive(Clone)]
struct LatticeRegion {
    points: Vec<[f64; 3]>,
    bounds: BoundingBox,
    energy: f64,
}

#[derive(Clone, Copy)]
struct BoundingBox {
    min: [f64; 3],
    max: [f64; 3],
}

impl ShardedLattice {
    pub fn new(points: Vec<[f64; 3]>) -> Self {
        let mut lattice = ShardedLattice {
            points: points.clone(),
            shards: Vec::new(),
            cache: Arc::new(RwLock::new(HashMap::new())),
            modification_counter: 0,
        };
        lattice.create_shards();
        lattice
    }

    fn create_shards(&mut self) {
        let optimal_shard_size = (self.points.len() as f64).sqrt().ceil() as usize;

        self.shards = self.points
            .chunks(optimal_shard_size)
            .map(|chunk| {
                let points = chunk.to_vec();
                let bounds = Self::calculate_bounds(&points);
                let energy = Self::calculate_shard_energy(&points);
                LatticeRegion { points, bounds, energy }
            })
            .collect();
    }

    fn calculate_shard_energy(points: &[[f64; 3]]) -> f64 {
        points.par_iter()
            .map(|[x, y, z]| x * x + y * y + z * z)
            .sum()
    }

    fn calculate_bounds(points: &[[f64; 3]]) -> BoundingBox {
        let mut min = [f64::INFINITY; 3];
        let mut max = [f64::NEG_INFINITY; 3];

        for point in points {
            for i in 0..3 {
                min[i] = min[i].min(point[i]);
                max[i] = max[i].max(point[i]);
            }
        }

        BoundingBox { min, max }
    }

    pub fn calculate_energy(&self) -> f64 {
        let cache_key = self.modification_counter;

        if let Some(energy) = self.cache.read().get(&cache_key) {
            return *energy;
        }

        let total_energy: f64 = self.shards.par_iter()
            .map(|shard| shard.energy)
            .sum();

        self.cache.write().insert(cache_key, total_energy);
        total_energy
    }

    pub fn shatter(&mut self, impact_point: [f64; 3], force: f64) {
        let affected_shards: Vec<_> = self.shards.par_iter()
            .enumerate()
            .filter(|(_, shard)| {
                Self::calculate_impact_effect(impact_point, &shard.bounds) > force
            })
            .map(|(i, _)| i)
            .collect();

        for &shard_idx in affected_shards.iter().rev() {
            let shard = self.shards.remove(shard_idx);
            let (shard1, shard2) = self.split_shard(shard, impact_point);
            self.shards.push(shard1);
            self.shards.push(shard2);
        }

        self.modification_counter = self.modification_counter.wrapping_add(1);
        self.cache.write().clear();
    }

    fn calculate_impact_effect(impact: [f64; 3], bounds: &BoundingBox) -> f64 {
        let center = [
            (bounds.min[0] + bounds.max[0]) / 2.0,
            (bounds.min[1] + bounds.max[1]) / 2.0,
            (bounds.min[2] + bounds.max[2]) / 2.0,
        ];

        let distance = (0..3)
            .map(|i| (impact[i] - center[i]).powi(2))
            .sum::<f64>()
            .sqrt();

        1.0 / (1.0 + distance)
    }

    fn split_shard(&self, shard: LatticeRegion, impact: [f64; 3]) -> (LatticeRegion, LatticeRegion) {
        let (close, far): (Vec<_>, Vec<_>) = shard.points.into_iter()
            .partition(|point| {
                let distance = (0..3)
                    .map(|i| (point[i] - impact[i]).powi(2))
                    .sum::<f64>()
                    .sqrt();
                distance < 1.0
            });

        (
            LatticeRegion {
                points: close.clone(),
                bounds: Self::calculate_bounds(&close),
                energy: Self::calculate_shard_energy(&close),
            },
            LatticeRegion {
                points: far.clone(),
                bounds: Self::calculate_bounds(&far),
                energy: Self::calculate_shard_energy(&far),
            },
        )
    }
}
