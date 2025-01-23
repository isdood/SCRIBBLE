use criterion::{black_box, criterion_group, criterion_main, Criterion};
use opal::harmony::HarmonyCore;

fn harmony_core_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("harmony_operations");
    group.sample_size(200); // Increase sample size
    group.measurement_time(std::time::Duration::from_secs(3));

    group.bench_function("harmony_core_optimize", |b| {
        b.iter_with_setup(
            || HarmonyCore::new(),
            |mut core| {
                black_box(core.optimize());
            }
        )
    });

    group.finish();
}

criterion_group! {
    name = benches;
    config = Criterion::default()
        .noise_threshold(0.03) // More stringent noise filtering
        .significance_level(0.01)
        .sample_size(200);
    targets = harmony_core_benchmark
}
criterion_main!(benches);
