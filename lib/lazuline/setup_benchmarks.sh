#!/bin/bash
# setup_benchmarks.sh
# Created: 2025-01-21 19:09:45
# Author: isdood

echo "[$(date -u '+%Y-%m-%d %H:%M:%S UTC')] Setting up benchmark infrastructure..."

# Clean up existing benchmark files
rm -f benches/lazuline_benchmarks.rs
rm -rf target/criterion

# First, read the existing Cargo.toml
if [ -f Cargo.toml ]; then
    # Create backup
    cp Cargo.toml Cargo.toml.bak
    # Remove existing sections we'll be adding
    sed -i '/\[dev-dependencies\]/,/^$/d' Cargo.toml
    sed -i '/\[\[bench\]\]/,/^$/d' Cargo.toml
fi

# Create benches directory
mkdir -p benches

# Create benchmark file
cat > benches/lib.rs << 'END'
use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use lazuline::Lazuline;

fn bench_initialization(c: &mut Criterion) {
    c.bench_function("initialization", |b| {
        b.iter(|| {
            let instance = black_box(Lazuline::new().unwrap());
            black_box(instance);
        })
    });
}

fn bench_channel_compute(c: &mut Criterion) {
    let mut group = c.benchmark_group("channel_compute");
    let sizes = [10usize, 100, 1000, 10_000, 100_000];

    for &size in sizes.iter() {
        let mut instance = Lazuline::new().unwrap();
        let data = vec![1.0f64; size];

        group.bench_with_input(BenchmarkId::from_parameter(size), &size, |b, &_| {
            b.iter(|| {
                black_box(instance.channel_compute(black_box(&data)).unwrap());
            })
        });
    }

    group.finish();
}

fn bench_multiple_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("multiple_operations");
    let mut instance = Lazuline::new().unwrap();
    let operations = [2usize, 5, 10];

    for &ops in operations.iter() {
        let data = vec![1.0f64; 1000];

        group.bench_with_input(BenchmarkId::new("sequential", ops), &ops, |b, &_| {
            b.iter(|| {
                for _ in 0..ops {
                    black_box(instance.channel_compute(black_box(&data)).unwrap());
                }
            })
        });
    }

    group.finish();
}

criterion_group!(
    name = benches;
    config = Criterion::default().sample_size(10);
    targets = bench_initialization, bench_channel_compute, bench_multiple_operations
);
criterion_main!(benches);
END

# Create benchmark runner script
cat > bench.sh << 'END'
#!/bin/bash

echo "[$(date -u '+%Y-%m-%d %H:%M:%S UTC')] Running Lazuline benchmarks..."

# Clean any previous benchmark results
rm -rf target/criterion/report

# Run benchmarks
RUSTFLAGS="-C target-cpu=native" cargo bench

# Wait for criterion to finish generating reports
sleep 2

echo "[$(date -u '+%Y-%m-%d %H:%M:%S UTC')] ✨ Benchmarks completed!"

# List available reports
echo "Available benchmark reports:"
find target/criterion -name "report" -type d | while read -r dir; do
    echo "  - file://$PWD/$dir/index.html"
done
END

chmod +x bench.sh

# Update Cargo.toml
cat > Cargo.toml.new << END
$(cat Cargo.toml)

[dev-dependencies]
criterion = { version = "0.5", features = ["html_reports"] }

[[bench]]
name = "lib"
harness = false
END

# Replace the original with the new version
mv Cargo.toml.new Cargo.toml

echo "✨ Benchmark infrastructure created!"
echo "Run './bench.sh' to execute benchmarks."
