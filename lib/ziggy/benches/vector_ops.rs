use criterion::{black_box, criterion_group, criterion_main, Criterion};
use ziggy::Vector3D;

fn vector_benchmark(c: &mut Criterion) {
    let v1 = Vector3D::new(1.0, 2.0, 3.0);
    let v2 = Vector3D::new(4.0, 5.0, 6.0);

    c.bench_function("dot_product", |b| {
        b.iter(|| v1.dot(black_box(&v2)))
    });

    c.bench_function("magnitude", |b| {
        b.iter(|| v1.magnitude())
    });
}

criterion_group!(benches, vector_benchmark);
criterion_main!(benches);
