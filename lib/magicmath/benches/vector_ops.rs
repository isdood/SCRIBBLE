//! MagicMath Vector Operations Benchmarks
//! ===================================
//!
//! Author: Caleb J.D. Terkovics <isdood>
//! Current User: isdood
//! Created: 2025-01-20
//! Last Updated: 2025-01-20 17:52:34 UTC
//! Version: 0.1.0
//! License: MIT

use criterion::{criterion_group, criterion_main, Criterion};
use std::time::Duration;
use magicmath::vector3d::Vector3D;

#[derive(Copy, Clone)]
#[repr(align(64))]
struct AlignedVectors {
    data: [(f64, f64, f64); 4]
}

static SAMPLE_VECTORS: AlignedVectors = AlignedVectors {
    data: [
        (1.0, 2.0, 3.0),
        (4.0, 5.0, 6.0),
        (3.0, 4.0, 0.0),
        (-1.0, 2.0, -3.0),
    ]
};

fn setup_benchmark_group<'a>(
    c: &'a mut Criterion,
    name: &str,
) -> criterion::BenchmarkGroup<'a, criterion::measurement::WallTime> {
    let mut group = c.benchmark_group(name);
    group.sample_size(1000)
    .warm_up_time(Duration::from_secs(3))
    .measurement_time(Duration::from_secs(8))
    .noise_threshold(0.005)
    .significance_level(0.001)
    .confidence_level(0.999);
    group
}

fn dot_product(c: &mut Criterion) {
    let mut group = setup_benchmark_group(c, "Dot Product");

    for (i, &(x1, y1, z1)) in SAMPLE_VECTORS.data.iter().enumerate() {
        for (j, &(x2, y2, z2)) in SAMPLE_VECTORS.data.iter().enumerate() {
            if i < j {
                let v1 = Vector3D::new(x1, y1, z1);
                let v2 = Vector3D::new(x2, y2, z2);
                let id = format!("v{}·v{}", i + 1, j + 1);
                group.bench_function(id, |b| {
                    b.iter(|| {
                        criterion::black_box(v1.dot(&v2))
                    })
                });
            }
        }
    }
    group.finish();
}

fn magnitude(c: &mut Criterion) {
    let mut group = setup_benchmark_group(c, "Magnitude");

    for (i, &(x, y, z)) in SAMPLE_VECTORS.data.iter().enumerate() {
        let v = Vector3D::new(x, y, z);
        let id = format!("v{}", i + 1);
        group.bench_function(id, |b| {
            b.iter(|| {
                criterion::black_box(v.magnitude())
            })
        });
    }
    group.finish();
}

criterion_group! {
    name = benches;
    config = Criterion::default()
    .with_plots()
    .sample_size(1000)
    .warm_up_time(Duration::from_secs(3))
    .measurement_time(Duration::from_secs(8));
    targets = dot_product, magnitude
}
criterion_main!(benches);
