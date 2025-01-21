use criterion::{black_box, criterion_group, criterion_main, Criterion};
use zigzag::Vector3D;

fn bench_dot_product(c: &mut Criterion) {
    let v1 = Vector3D::new(1.0, 2.0, 3.0);
    let v2 = Vector3D::new(4.0, 5.0, 6.0);

    c.bench_function("dot_product", |b| {
        b.iter(|| v1.dot(black_box(&v2)))
    });
}

criterion_group!(benches, bench_dot_product);
criterion_main!(benches);
