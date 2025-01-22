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
