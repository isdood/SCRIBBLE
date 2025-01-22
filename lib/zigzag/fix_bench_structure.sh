#!/bin/bash
# fix_bench_structure.sh
# Created by: isdood
# Date: 2025-01-22 00:21:06
# Author: isdood

echo "Fixing benchmark structure and imports..."

# First, ensure the main lib.rs exists and exports the superpurple module
cat > src/lib.rs << 'EOF'
//! ZigZag library
//! Created: 2025-01-22 00:21:06
//! Author: isdood

pub mod superpurple;

// Re-export main types for convenience
pub use superpurple::{
    Vec3f,
    Vec3d,
    LatticeSymmetry,
    QuantumState,
    SIMDOps,
};
EOF

# Update the main benchmark file
cat > benches/superpurple/main.rs << 'EOF'
//! Superpurple benchmarks
//! Created: 2025-01-22 00:21:06
//! Author: isdood

use criterion::{
    black_box,
    criterion_group,
    criterion_main,
    Criterion,
    BenchmarkId,
};

// Import from the crate root
use zigzag::{
    Vec3f,
    Vec3d,
    LatticeSymmetry,
    QuantumState,
    SIMDOps,
};
use rand::prelude::*;

mod core_bench;
mod quantum_bench;
mod memory_bench;

const SIZES: &[usize] = &[1024, 4096, 16384, 65536];

fn bench_vector_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("vector_operations");

    for &size in SIZES {
        // Generate random data
        let mut rng = StdRng::seed_from_u64(42);
        let data_f32: Vec<f32> = (0..size).map(|_| rng.gen()).collect();
        let data_f64: Vec<f64> = (0..size).map(|_| rng.gen()).collect();

        // Benchmark f32 operations
        group.bench_with_input(
            BenchmarkId::new("dot_product_f32", size),
            &data_f32,
            |b, data| {
                let vec = Vec3f::new(1.0, 2.0, 3.0);
                b.iter(|| {
                    black_box(vec.dot_product_simd(black_box(data)));
                })
            },
        );

        // Benchmark f64 operations
        group.bench_with_input(
            BenchmarkId::new("dot_product_f64", size),
            &data_f64,
            |b, data| {
                let vec = Vec3d::new(1.0, 2.0, 3.0);
                b.iter(|| {
                    black_box(vec.dot_product_simd(black_box(data)));
                })
            },
        );
    }

    group.finish();
}

criterion_group!(
    benches,
    bench_vector_operations,
    core_bench::bench_core_ops,
    quantum_bench::bench_quantum_ops,
    memory_bench::bench_memory_ops
);
criterion_main!(benches);
EOF

# Update component benchmark files
cat > benches/superpurple/core_bench.rs << 'EOF'
use criterion::Criterion;
use zigzag::superpurple::core::*;

pub fn bench_core_ops(c: &mut Criterion) {
    let group = c.benchmark_group("core_operations");
    // TODO: Implement core benchmarks
    group.finish();
}
EOF

cat > benches/superpurple/quantum_bench.rs << 'EOF'
use criterion::Criterion;
use zigzag::superpurple::quantum::*;

pub fn bench_quantum_ops(c: &mut Criterion) {
    let group = c.benchmark_group("quantum_operations");
    // TODO: Implement quantum benchmarks
    group.finish();
}
EOF

cat > benches/superpurple/memory_bench.rs << 'EOF'
use criterion::Criterion;
use zigzag::superpurple::memory::*;

pub fn bench_memory_ops(c: &mut Criterion) {
    let group = c.benchmark_group("memory_operations");
    // TODO: Implement memory benchmarks
    group.finish();
}
EOF

# Remove old vector_ops.rs benchmark if it exists
rm -f benches/vector_ops.rs

# Update Cargo.toml
cat > Cargo.toml << 'EOF'
[package]
name = "zigzag"
version = "0.1.0"
edition = "2021"

[workspace]

[dependencies]
parking_lot = "0.12"

[features]
default = []
superpurple = []

[dev-dependencies]
criterion = { version = "0.5", features = ["html_reports"] }
rand = "0.8"

[[bench]]
name = "superpurple_bench"
harness = false
path = "benches/superpurple/main.rs"
EOF

echo "Fixed benchmark structure and imports."
echo "Key changes:"
echo "1. Added lib.rs with proper exports"
echo "2. Updated benchmark imports"
echo "3. Removed mutable warnings"
echo "4. Cleaned up old benchmarks"
echo ""
echo "You can now run: cargo bench --features superpurple"
