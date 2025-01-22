//! Superpurple benchmarks
//! Created: 2025-01-22 00:21:06
//! Author: isdood

use criterion::{
    black_box,
    criterion_group,
    criterion_main,
    Criterion,
    BenchmarkId,
};

// Import from the crate root
use zigzag::{
    Vec3f,
    Vec3d,
    LatticeSymmetry,
    QuantumState,
    SIMDOps,
};
use rand::prelude::*;

mod core_bench;
mod quantum_bench;
mod memory_bench;

const SIZES: &[usize] = &[1024, 4096, 16384, 65536];

fn bench_vector_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("vector_operations");

    for &size in SIZES {
        // Generate random data
        let mut rng = StdRng::seed_from_u64(42);
        let data_f32: Vec<f32> = (0..size).map(|_| rng.gen()).collect();
        let data_f64: Vec<f64> = (0..size).map(|_| rng.gen()).collect();

        // Benchmark f32 operations
        group.bench_with_input(
            BenchmarkId::new("dot_product_f32", size),
            &data_f32,
            |b, data| {
                let vec = Vec3f::new(1.0, 2.0, 3.0);
                b.iter(|| {
                    black_box(vec.dot_product_simd(black_box(data)));
                })
            },
        );

        // Benchmark f64 operations
        group.bench_with_input(
            BenchmarkId::new("dot_product_f64", size),
            &data_f64,
            |b, data| {
                let vec = Vec3d::new(1.0, 2.0, 3.0);
                b.iter(|| {
                    black_box(vec.dot_product_simd(black_box(data)));
                })
            },
        );
    }

    group.finish();
}

criterion_group!(
    benches,
    bench_vector_operations,
    core_bench::bench_core_ops,
    quantum_bench::bench_quantum_ops,
    memory_bench::bench_memory_ops
);
criterion_main!(benches);
