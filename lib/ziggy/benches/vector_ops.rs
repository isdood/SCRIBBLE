//! Vector Operations Benchmarks
//! ==========================
//!
//! Author: Caleb J.D. Terkovics <isdood>
//! Current User: isdood
//! Created: 2025-01-19
//! Last Updated: 2025-01-20 17:03:25 UTC
//! Version: 0.1.0
//! License: MIT

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::time::Duration;
use ziggy::Vector3D;

const SAMPLE_VECTORS: [(f64, f64, f64); 4] = [
    (1.0, 2.0, 3.0),
    (4.0, 5.0, 6.0),
    (3.0, 4.0, 0.0),
    (-1.0, 2.0, -3.0),
];

fn setup_benchmark_group<'a>(
    c: &'a mut Criterion,
    name: &str,
) -> criterion::BenchmarkGroup<'a, criterion::measurement::WallTime> {
    let mut group = c.benchmark_group(name);
    group.sample_size(1000)           // Increased sample size
    .warm_up_time(Duration::from_secs(2))
    .measurement_time(Duration::from_secs(5))
    .noise_threshold(0.01)        // Stricter noise threshold
    .significance_level(0.01)     // Stricter significance level
    .confidence_level(0.99);      // Higher confidence level
    group
}

fn dot_product(c: &mut Criterion) {
    let mut group = setup_benchmark_group(c, "Dot Product");

    for (i, &(x1, y1, z1)) in SAMPLE_VECTORS.iter().enumerate() {
        for (j, &(x2, y2, z2)) in SAMPLE_VECTORS.iter().enumerate() {
            if i < j {
                let v1 = Vector3D::new(x1, y1, z1);
                let v2 = Vector3D::new(x2, y2, z2);
                let id = format!("v{}·v{}", i + 1, j + 1);
                group.bench_function(id, |b| {
                    b.iter_batched(
                        || (v1, v2),
                                   |(v1, v2)| black_box(v1.dot(&v2)),
                                   criterion::BatchSize::SmallInput,
                    )
                });
            }
        }
    }
    group.finish();
}

fn magnitude(c: &mut Criterion) {
    let mut group = setup_benchmark_group(c, "Magnitude");

    for (i, &(x, y, z)) in SAMPLE_VECTORS.iter().enumerate() {
        let v = Vector3D::new(x, y, z);
        let id = format!("v{}", i + 1);
        group.bench_function(id, |b| {
            b.iter_batched(
                || v,
                |v| black_box(v.magnitude()),
                           criterion::BatchSize::SmallInput,
            )
        });
    }
    group.finish();
}

criterion_group! {
    name = benches;
    config = Criterion::default()
    .with_plots()
    .sample_size(1000)
    .warm_up_time(Duration::from_secs(2))
    .measurement_time(Duration::from_secs(5));
    targets = dot_product, magnitude
}
criterion_main!(benches);
