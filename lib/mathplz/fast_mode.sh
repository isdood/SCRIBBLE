#!/bin/bash

echo "📦 Creating fast_mode optimizations for MathPLZ (2025-01-23 04:42:47 UTC)"
echo "Author: isdood"

cd /home/guavabot1/scribble/scribble/lib/mathplz

# Update Cargo.toml with target-cpu features
cat > "lib/rust/Cargo.toml" << 'CARGO_EOF'
[package]
name = "mathplz"
version = "0.1.0"
edition = "2021"

[dependencies]
rand = "0.8"
num-complex = "0.4"
rayon = "1.7"
crossbeam = "0.8"
parking_lot = "0.12"
hashbrown = "0.14"

[dev-dependencies]
criterion = { version = "0.5", features = ["html_reports"] }

[profile.release]
lto = "fat"
codegen-units = 1
panic = "abort"
opt-level = 3
debug = false
CARGO_EOF

# Create optimized crystal lattice implementation
cat > "lib/rust/src/crystal.rs" << 'CRYSTAL_EOF'
use rayon::prelude::*;
use std::sync::Arc;
use parking_lot::RwLock;
use hashbrown::HashMap;

#[cfg(target_arch = "x86_64")]
use std::arch::x86_64::*;

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

    #[cfg(all(target_arch = "x86_64", target_feature = "avx2"))]
    unsafe fn calculate_shard_energy_simd(points: &[[f64; 3]]) -> f64 {
        let mut sum = _mm256_setzero_pd();

        for chunk in points.chunks(4) {
            let mut chunk_sum = _mm256_setzero_pd();

            // Load x, y, z coordinates
            for i in 0..chunk.len() {
                let point = chunk[i];
                let coord = _mm256_set_pd(0.0, point[2], point[1], point[0]);
                chunk_sum = _mm256_fmadd_pd(coord, coord, chunk_sum);
            }

            sum = _mm256_add_pd(sum, chunk_sum);
        }

        // Horizontal sum
        let sum_arr = std::mem::transmute::<__m256d, [f64; 4]>(sum);
        sum_arr.iter().sum()
    }

    #[cfg(not(all(target_arch = "x86_64", target_feature = "avx2")))]
    fn calculate_shard_energy_simd(_points: &[[f64; 3]]) -> f64 {
        unimplemented!("SIMD not available on this platform")
    }

    fn calculate_shard_energy(points: &[[f64; 3]]) -> f64 {
        #[cfg(all(target_arch = "x86_64", target_feature = "avx2"))]
        {
            if points.len() >= 4 {
                unsafe { Self::calculate_shard_energy_simd(points) }
            } else {
                Self::calculate_shard_energy_scalar(points)
            }
        }

        #[cfg(not(all(target_arch = "x86_64", target_feature = "avx2")))]
        {
            Self::calculate_shard_energy_scalar(points)
        }
    }

    fn calculate_shard_energy_scalar(points: &[[f64; 3]]) -> f64 {
        points.iter()
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
CRYSTAL_EOF

# Update lib.rs
cat > "lib/rust/src/lib.rs" << 'LIB_EOF'
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
LIB_EOF

echo "✨ Fast mode optimizations updated successfully!

Key improvements:
1. Proper conditional compilation for SIMD
2. Fallback scalar implementation
3. Architecture-specific optimizations
4. Improved error handling
5. Better feature detection

Run benchmarks:
cd lib/rust && RUSTFLAGS='-C target-cpu=native' cargo bench

Created: 2025-01-23 04:42:47 UTC
Author: isdood"

chmod +x fast_mode.sh
