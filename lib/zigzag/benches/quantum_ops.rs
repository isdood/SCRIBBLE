use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use zigzag::quantum::*;

fn quantum_gate_benchmark(c: &mut Criterion) {
    let state = QuantumState::new(1.0);
    let mut group = c.benchmark_group("quantum_gates");
    group.sample_size(50);
    group.warm_up_time(std::time::Duration::from_millis(500));

    let data_sizes = [16, 64, 256];

    for size in &data_sizes {
        let data: Vec<f32> = (0..*size).map(|i| (i as f32).sin()).collect();

        let gates: Vec<(&str, Box<dyn QuantumOp<f32>>)> = vec![
            ("hadamard", Box::new(HadamardGate)),
            ("cnot", Box::new(CNOTGate)),
            ("swap", Box::new(SWAPGate)),
            ("controlled_phase", Box::new(ControlledPhaseGate::new(std::f64::consts::PI / 4.0))),
            ("sqrt_not", Box::new(SqrtNOTGate)),
        ];

        for (name, gate) in gates {
            group.bench_with_input(
                BenchmarkId::new(name, size),
                &data,
                |b, data| {
                    b.iter(|| gate.apply(black_box(&state), black_box(data)))
                },
            );
        }
    }

    group.finish();
}

criterion_group!(
    name = benches;
    config = Criterion::default()
        .significance_level(0.01)
        .noise_threshold(0.02);
    targets = quantum_gate_benchmark
);
criterion_main!(benches);
