//! Vector Operations Benchmarks
//! ==========================
//!
//! Author: Caleb J.D. Terkovics <isdood>
//! Current User: isdood
//! Created: 2025-01-19
//! Last Updated: 2025-01-20 17:08:00 UTC
//! Version: 0.1.0
//! License: MIT

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::time::Duration;
use ziggy::Vector3D;

#[inline(never)]
fn aligned_vector() -> [(f64, f64, f64); 4] {
    #[repr(align(64))]  // Cache line alignment
    static SAMPLE_VECTORS: [(f64, f64, f64); 4] = [
        (1.0, 2.0, 3.0),
        (4.0, 5.0, 6.0),
        (3.0, 4.0, 0.0),
        (-1.0, 2.0, -3.0),
    ];
    SAMPLE_VECTORS
}

fn setup_benchmark_group<'a>(
    c: &'a mut Criterion,
    name: &str,
) -> criterion::BenchmarkGroup<'a, criterion::measurement::WallTime> {
    let mut group = c.benchmark_group(name);
    group.sample_size(1000)           // Keep sample size
    .warm_up_time(Duration::from_secs(3))  // Increased warm-up
    .measurement_time(Duration::from_secs(8))  // Increased measurement time
    .noise_threshold(0.005)       // Even stricter noise threshold
    .significance_level(0.001)    // More strict significance
    .confidence_level(0.999);     // Higher confidence
    group
}

fn dot_product(c: &mut Criterion) {
    let mut group = setup_benchmark_group(c, "Dot Product");
    let vectors = aligned_vector();

    for (i, &(x1, y1, z1)) in vectors.iter().enumerate() {
        for (j, &(x2, y2, z2)) in vectors.iter().enumerate() {
            if i < j {
                let v1 = Vector3D::new(x1, y1, z1);
                let v2 = Vector3D::new(x2, y2, z2);
                let id = format!("v{}·v{}", i + 1, j + 1);
                group.bench_function(id, |b| {
                    b.iter_with_setup(
                        || (v1, v2),
                                      |(v1, v2)| {
                                          criterion::black_box(v1.dot(&v2))
                                      }
                    )
                });
            }
        }
    }
    group.finish();
}

fn magnitude(c: &mut Criterion) {
    let mut group = setup_benchmark_group(c, "Magnitude");
    let vectors = aligned_vector();

    for (i, &(x, y, z)) in vectors.iter().enumerate() {
        let v = Vector3D::new(x, y, z);
        let id = format!("v{}", i + 1);
        group.bench_function(id, |b| {
            b.iter_with_setup(
                || v,
                |v| {
                    criterion::black_box(v.magnitude())
                }
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
    .warm_up_time(Duration::from_secs(3))
    .measurement_time(Duration::from_secs(8));
    targets = dot_product, magnitude
}
criterion_main!(benches);
