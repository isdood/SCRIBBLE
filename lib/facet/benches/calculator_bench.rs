// benches/calculator_bench.rs
use criterion::{criterion_group, criterion_main, Criterion};
use facet_rust::calculator::Calculator;

fn benchmark_calculations(c: &mut Criterion) {
    let calc = setup_calculator();

    c.bench_function("simple_add", |b| {
        b.iter(|| calc.compute("2 + 2", None))
    });

    c.bench_function("complex_expression", |b| {
        b.iter(|| calc.compute("(2 + 3) * 4 / 2 ^ 2", None))
    });

    c.bench_function("crystal_resonance", |b| {
        b.iter(|| {
            calc.compute("2 * 2", Some(ComputeOptions {
                check_resonance: true,
                maintain_resonance: true,
            }))
        })
    });
}

criterion_group!(benches, benchmark_calculations);
criterion_main!(benches);
