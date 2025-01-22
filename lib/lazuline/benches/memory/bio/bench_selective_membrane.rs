//! Selective Membrane Benchmark
//! Created: 2025-01-22
//! Author: isdood

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use lazuline::memory::bio::selective_membrane::{MembraneCache, AccessPredictor, PermeabilityController};

pub fn bench_membrane_cache(c: &mut Criterion) {
    let mut cache = MembraneCache::new(100);
    c.bench_function("membrane_cache_put", |b| {
        b.iter(|| {
            for i in 0..100 {
                cache.put(format!("key{}", i), format!("value{}", i));
            }
        })
    });
    c.bench_function("membrane_cache_get", |b| {
        b.iter(|| {
            for i in 0..100 {
                black_box(cache.get(&format!("key{}", i)));
            }
        })
    });
}

pub fn bench_access_predictor(c: &mut Criterion) {
    let mut predictor = AccessPredictor::new();
    c.bench_function("access_predictor_record", |b| {
        b.iter(|| {
            for i in 0..100 {
                predictor.record_access(format!("key{}", i));
            }
        })
    });
    c.bench_function("access_predictor_predict", |b| {
        b.iter(|| {
            black_box(predictor.predict());
        })
    });
}

pub fn bench_permeability_controller(c: &mut Criterion) {
    let mut controller = PermeabilityController::new();
    c.bench_function("permeability_control", |b| {
        b.iter(|| {
            controller.control(black_box(0.5));
        })
    });
}

criterion_group!(benches, bench_membrane_cache, bench_access_predictor, bench_permeability_controller);
criterion_main!(benches);
