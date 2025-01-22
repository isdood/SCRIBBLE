use criterion::{black_box, criterion_group, criterion_main, Criterion};
use zigzag::quantum::{
    QuantumState,
    QuantumOp,
    HadamardGate,
    CNOTGate,
    SWAPGate,
    ControlledPhaseGate,
    SqrtNOTGate,
};

fn quantum_gate_benchmark(c: &mut Criterion) {
    let state = QuantumState::new(1.0);
    let mut group = c.benchmark_group("quantum_gates");

    let data_sizes = [16, 64, 256];

    for size in data_sizes.iter() {
        let data: Vec<f32> = (0..*size).map(|i| (i as f32).sin()).collect();

        group.bench_function(format!("hadamard_gate_{}", size), |b| {
            let gate = HadamardGate;
            b.iter(|| {
                let gate: &dyn QuantumOp<f32> = &gate;
                gate.apply(black_box(&state), black_box(&data))
            })
        });

        group.bench_function(format!("cnot_gate_{}", size), |b| {
            let gate = CNOTGate;
            b.iter(|| {
                let gate: &dyn QuantumOp<f32> = &gate;
                gate.apply(black_box(&state), black_box(&data))
            })
        });

        group.bench_function(format!("swap_gate_{}", size), |b| {
            let gate = SWAPGate;
            b.iter(|| {
                let gate: &dyn QuantumOp<f32> = &gate;
                gate.apply(black_box(&state), black_box(&data))
            })
        });

        group.bench_function(format!("controlled_phase_{}", size), |b| {
            let gate = ControlledPhaseGate::new(std::f64::consts::PI / 4.0);
            b.iter(|| {
                let gate: &dyn QuantumOp<f32> = &gate;
                gate.apply(black_box(&state), black_box(&data))
            })
        });

        group.bench_function(format!("sqrt_not_{}", size), |b| {
            let gate = SqrtNOTGate;
            b.iter(|| {
                let gate: &dyn QuantumOp<f32> = &gate;
                gate.apply(black_box(&state), black_box(&data))
            })
        });
    }

    group.finish();
}

criterion_group!(benches, quantum_gate_benchmark);
criterion_main!(benches);
