#!/bin/bash
# setup_benchmarks.sh
# Created: 2025-01-21 18:56:45
# Author: isdood

echo "[$(date -u '+%Y-%m-%d %H:%M:%S UTC')] Setting up benchmark infrastructure..."

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

    for size in [10, 100, 1000, 10_000, 100_000].iter() {
        let mut instance = Lazuline::new().unwrap();
        let data = vec![1.0f64; *size];

        group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, &size| {
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

    for ops in [2, 5, 10].iter() {
        let data = vec![1.0f64; 1000];

        group.bench_with_input(BenchmarkId::new("sequential", ops), ops, |b, &ops| {
            b.iter(|| {
                for _ in 0..*ops {
                    black_box(instance.channel_compute(black_box(&data)).unwrap());
                }
            })
        });
    }

    group.finish();
}

criterion_group!(
    benches,
    bench_initialization,
    bench_channel_compute,
    bench_multiple_operations
);
criterion_main!(benches);
END

# Create benchmark runner script
cat > bench.sh << 'END'
#!/bin/bash

echo "[$(date -u '+%Y-%m-%d %H:%M:%S UTC')] Running Lazuline benchmarks..."

# Run benchmarks
cargo bench

# Generate benchmark report
echo -e "\nGenerating benchmark report..."
REPORT_DIR="target/criterion/report"
mkdir -p "$REPORT_DIR"

echo "<!DOCTYPE html>
<html>
<head>
    <title>Lazuline Benchmark Results</title>
    <style>
        body { font-family: Arial, sans-serif; margin: 2em; }
        .benchmark { margin-bottom: 2em; }
        .chart { margin: 1em 0; }
    </style>
</head>
<body>
    <h1>Lazuline Benchmark Results</h1>
    <p>Generated: $(date -u '+%Y-%m-%d %H:%M:%S UTC')</p>
    <div class='benchmark'>
        <h2>Results</h2>
        <pre>" > "$REPORT_DIR/index.html"

# Append benchmark results
cargo bench --no-run --message-format=json \
    | jq -r 'select(.profile.test == true) | .filenames[]' \
    | while read -r benchmark; do
        "$benchmark" --bench | tee -a "$REPORT_DIR/index.html"
    done

echo "</pre>
    </div>
</body>
</html>" >> "$REPORT_DIR/index.html"

echo "[$(date -u '+%Y-%m-%d %H:%M:%S UTC')] ✨ Benchmarks completed!"
echo "View the report at target/criterion/report/index.html"
END

chmod +x bench.sh

# Update Cargo.toml to ensure benchmark configuration
cat >> Cargo.toml << 'END'

[dev-dependencies]
criterion = { version = "0.5", features = ["html_reports"] }

[[bench]]
name = "lib"
harness = false
END

echo "✨ Benchmark infrastructure created!"
echo "Run './bench.sh' to execute benchmarks."
echo "View results in 'target/criterion/report/index.html'"
