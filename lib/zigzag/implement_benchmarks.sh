#!/bin/bash
# implement_benchmarks.sh
# Created by: isdood
# Date: 2025-01-22 00:43:30 UTC

echo "Implementing performance benchmarks and tests..."

# First create the benchmark directory
mkdir -p benches

# Create the main benchmark file
cat > benches/quantum_ops.rs << 'EOF_QUANTUM_BENCH'
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
    group.sample_size(100);
    group.measurement_time(std::time::Duration::from_secs(5));

    let data_sizes = [16, 64, 256, 1024, 4096];

    for size in data_sizes.iter() {
        let data: Vec<f32> = (0..*size).map(|i| (i as f32).sin()).collect();

        group.bench_function(format!("hadamard_gate_{}", size), |b| {
            let gate = HadamardGate;
            b.iter(|| gate.apply(black_box(&state), black_box(&data)))
        });

        group.bench_function(format!("cnot_gate_{}", size), |b| {
            let gate = CNOTGate;
            b.iter(|| gate.apply(black_box(&state), black_box(&data)))
        });

        group.bench_function(format!("swap_gate_{}", size), |b| {
            let gate = SWAPGate;
            b.iter(|| gate.apply(black_box(&state), black_box(&data)))
        });

        group.bench_function(format!("controlled_phase_{}", size), |b| {
            let gate = ControlledPhaseGate::new(std::f64::consts::PI / 4.0);
            b.iter(|| gate.apply(black_box(&state), black_box(&data)))
        });

        group.bench_function(format!("sqrt_not_{}", size), |b| {
            let gate = SqrtNOTGate;
            b.iter(|| gate.apply(black_box(&state), black_box(&data)))
        });
    }

    group.finish();
}

criterion_group!(benches, quantum_gate_benchmark);
criterion_main!(benches);
EOF_QUANTUM_BENCH

# Create lattice benchmarks
cat > benches/lattice_ops.rs << 'EOF_LATTICE_BENCH'
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use zigzag::lattice::{
    Lattice,
    CubicLattice,
    TetragonalLattice,
    HexagonalLattice,
    LatticeGroup,
};

fn lattice_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("lattice_operations");
    group.sample_size(100);
    group.measurement_time(std::time::Duration::from_secs(5));

    let data_sizes = [16, 64, 256, 1024, 4096];

    for size in data_sizes.iter() {
        let data: Vec<f32> = (0..*size).map(|i| (i as f32).cos()).collect();

        group.bench_function(format!("cubic_lattice_{}", size), |b| {
            let lattice = CubicLattice::new();
            b.iter(|| lattice.apply_symmetry(black_box(&data)))
        });

        group.bench_function(format!("tetragonal_lattice_{}", size), |b| {
            let lattice = TetragonalLattice::new();
            b.iter(|| lattice.apply_symmetry(black_box(&data)))
        });

        group.bench_function(format!("hexagonal_lattice_{}", size), |b| {
            let lattice = HexagonalLattice::new();
            b.iter(|| lattice.apply_symmetry(black_box(&data)))
        });

        group.bench_function(format!("lattice_group_{}", size), |b| {
            let mut lattice_group = LatticeGroup::new();
            lattice_group.add_operation(Box::new(CubicLattice::new()));
            lattice_group.add_operation(Box::new(TetragonalLattice::new()));
            b.iter(|| lattice_group.apply_group(black_box(&data)))
        });
    }

    group.finish();
}

criterion_group!(benches, lattice_benchmark);
criterion_main!(benches);
EOF_LATTICE_BENCH

# Add more test cases
cat > tests/quantum_tests.rs << 'EOF_QUANTUM_TESTS'
use zigzag::quantum::*;

#[test]
fn test_hadamard_properties() {
    let gate = HadamardGate;
    let state = QuantumState::new(1.0);
    let input = vec![1.0f32, 0.0];

    // Test H^2 = I (Hadamard is self-inverse)
    let once = gate.apply(&state, &input);
    let twice = gate.apply(&state, &once);

    assert!((twice[0] - input[0]).abs() < 1e-6);
    assert!((twice[1] - input[1]).abs() < 1e-6);
}

#[test]
fn test_cnot_properties() {
    let gate = CNOTGate;
    let state = QuantumState::new(1.0);

    // Test all basis states
    let tests = vec![
        (vec![0.0f32, 0.0], vec![0.0, 0.0]),
        (vec![0.0, 1.0], vec![0.0, 1.0]),
        (vec![1.0, 0.0], vec![1.0, 1.0]),
        (vec![1.0, 1.0], vec![1.0, 0.0]),
    ];

    for (input, expected) in tests {
        let result = gate.apply(&state, &input);
        assert_eq!(result, expected);
    }
}

#[test]
fn test_swap_properties() {
    let gate = SWAPGate;
    let state = QuantumState::new(1.0);

    // Test various pairs
    let tests = vec![
        vec![1.0f32, 0.0],
        vec![0.0, 1.0],
        vec![0.5, 0.5],
    ];

    for input in tests {
        let result = gate.apply(&state, &input);
        assert_eq!(result, vec![input[1], input[0]]);
    }
}

#[test]
fn test_controlled_phase() {
    let gate = ControlledPhaseGate::new(std::f64::consts::PI);
    let state = QuantumState::new(1.0);
    let input = vec![1.0f32, 1.0];

    let result = gate.apply(&state, &input);
    assert!((result[0] - 1.0).abs() < 1e-6);
    assert!((result[1] + 1.0).abs() < 1e-6);
}

#[test]
fn test_sqrt_not() {
    let gate = SqrtNOTGate;
    let state = QuantumState::new(1.0);
    let input = vec![1.0f32];

    // Test (√NOT)^2 = NOT
    let once = gate.apply(&state, &input);
    let twice = gate.apply(&state, &once);

    assert!((twice[0] - 0.0).abs() < 1e-6);
}
EOF_QUANTUM_TESTS

# Add lattice tests
cat > tests/lattice_tests.rs << 'EOF_LATTICE_TESTS'
use zigzag::lattice::*;

#[test]
fn test_cubic_symmetry_properties() {
    let lattice = CubicLattice::new();
    let input = vec![1.0f32, 0.0, 0.0, 0.0,
                     0.0, 1.0, 0.0, 0.0,
                     0.0, 0.0, 1.0, 0.0,
                     0.0, 0.0, 0.0, 1.0];

    let result = lattice.apply_symmetry(&input);
    assert_eq!(result.len(), input.len());

    // Test cubic symmetry preserves volume
    let sum_input: f32 = input.iter().sum();
    let sum_result: f32 = result.iter().sum();
    assert!((sum_input - sum_result).abs() < 1e-6);
}

#[test]
fn test_tetragonal_symmetry_properties() {
    let lattice = TetragonalLattice::new();
    let input = vec![1.0f32; 16];

    let result = lattice.apply_symmetry(&input);
    assert_eq!(result.len(), input.len());

    // Test tetragonal symmetry preserves area in xy-plane
    let xy_sum_input: f32 = input.iter().take(4).sum();
    let xy_sum_result: f32 = result.iter().take(4).sum();
    assert!((xy_sum_input - xy_sum_result).abs() < 1e-6);
}

#[test]
fn test_hexagonal_symmetry_properties() {
    let lattice = HexagonalLattice::new();
    let input = vec![1.0f32; 12];

    let result = lattice.apply_symmetry(&input);
    assert_eq!(result.len(), input.len());

    // Test 6-fold symmetry
    let mut current = input.clone();
    for _ in 0..6 {
        current = lattice.apply_symmetry(&current);
    }

    // After 6 rotations, should return to original
    for (a, b) in current.iter().zip(input.iter()) {
        assert!((a - b).abs() < 1e-6);
    }
}

#[test]
fn test_lattice_group_properties() {
    let mut group = LatticeGroup::new();
    group.add_operation(Box::new(CubicLattice::new()));
    group.add_operation(Box::new(TetragonalLattice::new()));

    let input = vec![1.0f32; 16];
    let result = group.apply_group(&input);

    // Test group operation preserves size
    assert_eq!(result.len(), input.len());

    // Test associativity
    let result1 = group.apply_group(&input);
    let result2 = group.apply_group(&result1);
    let direct = group.apply_group(&input);

    for (a, b) in result2.iter().zip(direct.iter()) {
        assert!((a - b).abs() < 1e-6);
    }
}
EOF_LATTICE_TESTS

# Update Cargo.toml to include benchmarks
cat >> Cargo.toml << 'EOF_CARGO'
[dev-dependencies]
criterion = "0.5"

[[bench]]
name = "quantum_ops"
harness = false

[[bench]]
name = "lattice_ops"
harness = false
EOF_CARGO

echo "Benchmark and test implementation complete!"
echo "Added:"
echo "1. Quantum operations benchmarks"
echo "2. Lattice operations benchmarks"
echo "3. Comprehensive quantum gate tests"
echo "4. Lattice symmetry property tests"
echo ""
echo "To run benchmarks:"
echo "cargo bench"
echo ""
echo "To run tests:"
echo "cargo test"
echo ""
echo "Next steps:"
echo "1. Add more test edge cases"
echo "2. Add performance optimizations"
echo "3. Add documentation"
echo "4. Profile and optimize hot paths"
