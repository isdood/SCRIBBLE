use criterion::{black_box, criterion_group, criterion_main, Criterion};
use lazuline::prelude::*;

fn initialization_benchmark(c: &mut Criterion) {
    c.bench_function("init", |b| {
        b.iter(|| {
            let _ = black_box(init()).unwrap();
        })
    });
}

criterion_group!(benches, initialization_benchmark);
criterion_main!(benches);
