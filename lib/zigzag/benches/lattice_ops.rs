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

    let data_sizes = [16, 64, 256];

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
            let mut group = LatticeGroup::new();
            group.add_operation(Box::new(CubicLattice::new()));
            group.add_operation(Box::new(TetragonalLattice::new()));
            b.iter(|| group.apply_group(black_box(&data)))
        });
    }

    group.finish();
}

criterion_group!(benches, lattice_benchmark);
criterion_main!(benches);
