// lib/carve/benches/translation_benchmarks.rs
// Last Updated: 2025-01-19 08:15:40 UTC
// Author: isdood
// Current User: isdood

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use carve::{UnifiedTranslator, ZigTranslator};
use harmony_core::Quantum;

fn zig_translation_benchmark(c: &mut Criterion) {
    let mut translator = UnifiedTranslator::new();
    let zig_source = r#"!zig!
    fn Vector => struct {
    x: f32,
    y: f32,

    fn init(x: f32, y: f32) => Self {
    return .{ .x = x, .y = y };
}
}

fn main() => void {
const v1 := Vector.init(1.0, 2.0);
}
!zig!"#;

c.bench_function("zig_translation", |b| {
    b.iter(|| translator.translate(black_box(zig_source)))
});
}

fn mixed_translation_benchmark(c: &mut Criterion) {
    let mut translator = UnifiedTranslator::new();
    let mixed_source = r#"
    !zig!
    fn add(a: i32, b: i32) => i32 {
    return a + b;
}
!zig!
!sql!
SELECT * FROM table;
!sql!
"#;

c.bench_function("mixed_translation", |b| {
    b.iter(|| translator.translate(black_box(mixed_source)))
});
}

criterion_group!(benches, zig_translation_benchmark, mixed_translation_benchmark);
criterion_main!(benches);
