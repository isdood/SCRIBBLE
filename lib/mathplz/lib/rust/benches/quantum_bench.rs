use criterion::{black_box, criterion_group, criterion_main, Criterion};
use mathplz::QuantumState;
use num_complex::Complex64;
use rand::Rng;

fn bench_quantum_state(c: &mut Criterion) {
    let mut rng = rand::thread_rng();
    let state = QuantumState::new(Complex64::new(
        rng.gen_range(-1.0..1.0),
        rng.gen_range(-1.0..1.0),
    ));

    c.bench_function("quantum_probability", |b| {
        b.iter(|| black_box(state.get_probability()))
    });
}

criterion_group!(benches, bench_quantum_state);
criterion_main!(benches);
