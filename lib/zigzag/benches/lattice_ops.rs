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
