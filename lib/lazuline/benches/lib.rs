use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use lazuline::Lazuline;

fn bench_initialization(c: &mut Criterion) {
    c.bench_function("initialization", |b| {
        b.iter(|| {
            let instance = black_box(Lazuline::new().unwrap());
            black_box(instance);
        })
    });
}

fn bench_channel_compute(c: &mut Criterion) {
    let mut group = c.benchmark_group("channel_compute");

    for size in [10, 100, 1000, 10_000, 100_000].iter() {
        let mut instance = Lazuline::new().unwrap();
        let data = vec![1.0f64; *size];

        group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, &size| {
            b.iter(|| {
                black_box(instance.channel_compute(black_box(&data)).unwrap());
            })
        });
    }

    group.finish();
}

fn bench_multiple_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("multiple_operations");
    let mut instance = Lazuline::new().unwrap();

    for ops in [2, 5, 10].iter() {
        let data = vec![1.0f64; 1000];

        group.bench_with_input(BenchmarkId::new("sequential", ops), ops, |b, &ops| {
            b.iter(|| {
                for _ in 0..*ops {
                    black_box(instance.channel_compute(black_box(&data)).unwrap());
                }
            })
        });
    }

    group.finish();
}

criterion_group!(
    benches,
    bench_initialization,
    bench_channel_compute,
    bench_multiple_operations
);
criterion_main!(benches);
