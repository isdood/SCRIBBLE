use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use zigzag::lattice::*;

fn lattice_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("lattice_operations");
    group.sample_size(100);
    group.warm_up_time(std::time::Duration::from_millis(500));

    // Use power-of-two sizes for better memory alignment
    let data_sizes = [32, 64, 256];

    for size in &data_sizes {
        let data: Vec<f32> = (0..*size).map(|i| (i as f32).cos()).collect();

        for (name, lattice) in [
            ("cubic", Box::new(CubicLattice::new()) as Box<dyn Lattice<f32>>),
            ("tetragonal", Box::new(TetragonalLattice::new())),
            ("hexagonal", Box::new(HexagonalLattice::new())),
        ] {
            group.bench_with_input(
                BenchmarkId::new(name, size),
                &data,
                |b, data| {
                    b.iter(|| lattice.apply_symmetry(black_box(data)))
                },
            );
        }

        group.bench_with_input(
            BenchmarkId::new("group", size),
            &data,
            |b, data| {
                let mut group = LatticeGroup::new();
                group.add_operation(Box::new(CubicLattice::new()));
                group.add_operation(Box::new(TetragonalLattice::new()));
                b.iter(|| group.apply_group(black_box(data)))
            },
        );
    }

    group.finish();
}

criterion_group!(
    name = benches;
    config = Criterion::default()
        .significance_level(0.01)
        .noise_threshold(0.02);
    targets = lattice_benchmark
);
criterion_main!(benches);
