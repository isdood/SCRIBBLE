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
