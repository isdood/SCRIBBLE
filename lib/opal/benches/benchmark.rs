#[macro_use]
extern crate criterion;
use criterion::Criterion;
use opal::harmony::HarmonyCore;

fn harmony_core_benchmark(c: &mut Criterion) {
    c.bench_function("harmony_core_optimize", |b| b.iter(|| {
        let mut core = HarmonyCore::new();
        core.optimize();
    }));
}

criterion_group!(benches, harmony_core_benchmark);
criterion_main!(benches);
