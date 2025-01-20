//! Vector Operations Benchmarks
//! ==========================
//!
//! Author: Caleb J.D. Terkovics <isdood>
//! Current User: isdood
//! Created: 2025-01-19
//! Last Updated: 2025-01-20 16:52:44 UTC
//! Version: 0.1.0
//! License: MIT

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use ziggy::Vector3D;

fn dot_product(c: &mut Criterion) {
    let v1 = Vector3D::new(1.0, 2.0, 3.0);
    let v2 = Vector3D::new(4.0, 5.0, 6.0);

    c.bench_function("dot_product", |b| {
        b.iter(|| {
            black_box(v1.dot(&v2))
        })
    });
}

fn magnitude(c: &mut Criterion) {
    let v = Vector3D::new(3.0, 4.0, 0.0);

    c.bench_function("magnitude", |b| {
        b.iter(|| {
            black_box(v.magnitude())
        })
    });
}

criterion_group!{
    name = benches;
    config = Criterion::default()
    .sample_size(200)  // Increased sample size
    .warm_up_time(std::time::Duration::from_millis(500))  // Longer warm-up
    .measurement_time(std::time::Duration::from_secs(2))  // Longer measurement
    .noise_threshold(0.05);  // More strict noise threshold
    targets = dot_product, magnitude
}
criterion_main!(benches);
