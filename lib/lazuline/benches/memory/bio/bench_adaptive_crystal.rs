//! Adaptive Crystal Benchmark
//! Created: 2025-01-22
//! Author: isdood

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use lazuline::memory::bio::adaptive_crystal::{CrystalLattice, StructureOptimizer, EnergyTracker};

pub fn bench_crystal_lattice(c: &mut Criterion) {
    let mut lattice = CrystalLattice::new(100);
    c.bench_function("crystal_lattice_restructure", |b| {
        b.iter(|| {
            lattice.restructure();
        })
    });
}

pub fn bench_structure_optimizer(c: &mut Criterion) {
    let mut optimizer = StructureOptimizer::new(0.1);
    c.bench_function("structure_optimizer_optimize", |b| {
        b.iter(|| {
            optimizer.optimize();
        })
    });
}

pub fn bench_energy_tracker(c: &mut Criterion) {
    let mut tracker = EnergyTracker::new();
    c.bench_function("energy_tracker_track", |b| {
        b.iter(|| {
            tracker.track(black_box(1.0));
        })
    });
}

criterion_group!(benches, bench_crystal_lattice, bench_structure_optimizer, bench_energy_tracker);
criterion_main!(benches);
