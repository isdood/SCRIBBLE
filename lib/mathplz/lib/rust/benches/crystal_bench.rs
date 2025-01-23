use criterion::{black_box, criterion_group, criterion_main, Criterion};
use mathplz::CrystalLattice;
use rand::Rng;

fn generate_random_points(n: usize) -> Vec<[f64; 3]> {
    let mut rng = rand::thread_rng();
    (0..n)
        .map(|_| [
            rng.gen_range(-10.0..10.0),
            rng.gen_range(-10.0..10.0),
            rng.gen_range(-10.0..10.0),
        ])
        .collect()
}

fn bench_crystal_lattice(c: &mut Criterion) {
    let mut group = c.benchmark_group("CrystalLattice");

    for size in [10, 100, 1000, 10000].iter() {
        let points = generate_random_points(*size);
        let mut lattice = CrystalLattice::new(points.clone());

        // Benchmark energy calculation
        group.bench_function(&format!("energy_{}", size), |b| {
            b.iter(|| black_box(lattice.calculate_energy()));
        });

        // Benchmark shattering
        group.bench_function(&format!("shatter_{}", size), |b| {
            b.iter(|| {
                let mut l = lattice.clone();
                black_box(l.shatter([0.0, 0.0, 0.0], 1.0));
            });
        });
    }
    group.finish();
}

criterion_group!(benches, bench_crystal_lattice);
criterion_main!(benches);
