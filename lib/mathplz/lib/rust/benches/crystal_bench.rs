use criterion::{black_box, criterion_group, criterion_main, Criterion};
use mathplz::CrystalLattice;
use rand::Rng;

fn bench_crystal_lattice(c: &mut Criterion) {
    let mut group = c.benchmark_group("CrystalLattice");

    for size in [10, 100, 1000].iter() {
        let points: Vec<[f64; 3]> = (0..*size)
            .map(|_| {
                let mut rng = rand::thread_rng();
                [
                    rng.gen_range(-10.0..10.0),
                    rng.gen_range(-10.0..10.0),
                    rng.gen_range(-10.0..10.0),
                ]
            })
            .collect();

        let lattice = CrystalLattice::new(points);

        group.bench_function(&format!("energy_{}", size), |b| {
            b.iter(|| black_box(lattice.calculate_energy()))
        });
    }

    group.finish();
}

criterion_group!(benches, bench_crystal_lattice);
criterion_main!(benches);
