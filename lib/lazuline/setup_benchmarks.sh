#!/bin/bash
# setup_benchmarks.sh
# Created: 2025-01-21 19:13:55
# Author: isdood

echo "[$(date -u '+%Y-%m-%d %H:%M:%S UTC')] Setting up benchmark infrastructure..."

# Create backup of original Cargo.toml
cp Cargo.toml Cargo.toml.bak

# Remove any existing benchmark configurations
sed -i '/\[dev-dependencies\]/,/^$/d' Cargo.toml
sed -i '/\[\[bench\]\]/,/^$/d' Cargo.toml
sed -i '/criterion =/d' Cargo.toml

# Clean up existing benchmark files
rm -f benches/lazuline_benchmarks.rs
rm -rf target/criterion

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
    config = Criterion::default().sample_size(10).without_plots();
    targets = bench_initialization, bench_channel_compute, bench_multiple_operations
);
criterion_main!(benches);
END

# Create benchmark runner script
cat > bench.sh << 'END'
#!/bin/bash

echo "[$(date -u '+%Y-%m-%d %H:%M:%S UTC')] Running Lazuline benchmarks..."

# Create results directory
mkdir -p results

# Run benchmarks and capture output
RUSTFLAGS="-C target-cpu=native" cargo bench | tee results/benchmark_output.txt

echo "[$(date -u '+%Y-%m-%d %H:%M:%S UTC')] ✨ Benchmarks completed!"

# Parse and display summary
echo -e "\n📊 Performance Summary:"
echo "========================"

# Extract median times
echo "Initialization:"
grep "initialization" results/benchmark_output.txt | grep "time:" | awk -F'[\\[\\]]' '{print "  Median: " $2}'

echo -e "\nChannel Compute:"
grep "channel_compute/" results/benchmark_output.txt | grep "time:" | while read -r line; do
    size=$(echo $line | grep -o 'channel_compute/[0-9]*' | cut -d'/' -f2)
    time=$(echo $line | awk -F'[\\[\\]]' '{print $2}')
    echo "  Size $size: $time"
done

echo -e "\nMultiple Operations:"
grep "multiple_operations/sequential" results/benchmark_output.txt | grep "time:" | while read -r line; do
    ops=$(echo $line | grep -o 'sequential/[0-9]*' | cut -d'/' -f2)
    time=$(echo $line | awk -F'[\\[\\]]' '{print $2}')
    echo "  Ops $ops: $time"
done

echo -e "\nDetailed results saved in: results/benchmark_output.txt"
END

chmod +x bench.sh

# Create a temporary file for the new Cargo.toml content
echo "# Cargo.toml for Lazuline
# Updated: $(date -u '+%Y-%m-%d %H:%M:%S UTC')
# Author: $USER" > Cargo.toml.new

# Add existing content without dev-dependencies and bench sections
grep -v -e '\[dev-dependencies\]' -e 'criterion =' -e '\[\[bench\]\]' Cargo.toml >> Cargo.toml.new

# Add benchmark configuration
cat >> Cargo.toml.new << END

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
