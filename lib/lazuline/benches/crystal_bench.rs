use criterion::{black_box, criterion_group, criterion_main, Criterion};
use lazuline::{CrystalBridge, HarmonyField, WhimsyEngine};

fn crystal_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("Lazuline Crystal Formation");

    group.bench_function("harmony_enhancement", |b| {
        b.iter(|| {
            let mut field = HarmonyField::new(black_box(1.0));
            field.enhance_harmony().unwrap();
        });
    });

    group.bench_function("resonance_addition", |b| {
        let mut field = HarmonyField::new(1.0);
        b.iter(|| {
            field.add_resonance(black_box(42.0));
        });
    });

    group.bench_function("crystal_bridge_operations", |b| {
        let mut bridge = CrystalBridge::new();
        b.iter(|| {
            bridge.set_harmony(black_box(42.0));
            black_box(bridge.get_harmony());
        });
    });

    group.finish();
}

criterion_group!(benches, crystal_benchmark);
criterion_main!(benches);
