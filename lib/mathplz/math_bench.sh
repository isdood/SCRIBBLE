#!/bin/bash
# math_bench.sh
# Created: 2025-01-23 04:11:56 UTC
# Author: isdood

echo "📊 Setting up MathPLZ benchmarking suite..."

# Ensure we're in the correct directory
cd /home/guavabot1/scribble/scribble/lib/mathplz

# Create lib/rust directory if it doesn't exist
mkdir -p lib/rust
cd lib/rust

# Update Cargo.toml with benchmarking dependencies
cat > Cargo.toml << EOL
[package]
name = "mathplz"
version = "0.1.0"
edition = "2021"
authors = ["isdood"]
description = "Rust bindings for MathPLZ"

[dependencies]
thiserror = "1.0"
num-complex = "0.4"
libc = "0.2"
rand = "0.8"
rayon = "1.7"

[dev-dependencies]
criterion = { version = "0.5", features = ["html_reports"] }
iai = "0.1"
quickcheck = "1.0"
quickcheck_macros = "1.0"

[[bench]]
name = "crystal_bench"
harness = false

[[bench]]
name = "quantum_bench"
harness = false

[[bench]]
name = "dna_bench"
harness = false
EOL

# Create benchmark directory
mkdir -p benches

# Create benchmark files
# ... (previous benchmark file contents remain the same) ...

# Create the run_benchmarks.sh script in the lib/rust directory
cat > run_benchmarks.sh << 'EOL'
#!/bin/bash
# Run all benchmarks and generate reports
# Created: 2025-01-23 04:11:56 UTC
# Author: isdood

echo "🏃 Running benchmarks..."

# Run criterion benchmarks
cargo bench

# Generate summary report
echo "📝 Generating summary report..."
cat > benchmark_report.md << 'REPORT'
# MathPLZ Benchmark Results
Generated: $(date -u +"%Y-%m-%d %H:%M:%S UTC")

## Crystal Lattice Performance
\`\`\`
$(cargo bench -p mathplz crystal_bench 2>&1 | grep "time:")
\`\`\`

## Quantum State Performance
\`\`\`
$(cargo bench -p mathplz quantum_bench 2>&1 | grep "time:")
\`\`\`

## DNA Sequence Performance
\`\`\`
$(cargo bench -p mathplz dna_bench 2>&1 | grep "time:")
\`\`\`

## System Information
- CPU: $(lscpu | grep "Model name" | cut -d: -f2- | sed 's/^[ \t]*//')
- Memory: $(free -h | awk '/^Mem:/ {print $2}')
- Rust Version: $(rustc --version)
REPORT

echo "✅ Benchmarks complete! Results saved to benchmark_report.md"
echo "📊 View detailed HTML reports in target/criterion/report/index.html"
EOL

# Make the script executable
chmod +x run_benchmarks.sh

# Update .gitignore
cat > .gitignore << EOL
target/
Cargo.lock
*.criterion.json
EOL

echo "
✨ Benchmarking suite setup complete! The following files have been created:

📁 Project Structure:
  └── lib/rust/
      ├── benches/
      │   ├── crystal_bench.rs
      │   ├── quantum_bench.rs
      │   └── dna_bench.rs
      ├── run_benchmarks.sh
      └── .gitignore

📊 To run benchmarks:
  cd lib/rust
  ./run_benchmarks.sh              # Run all benchmarks
  cargo bench --bench crystal_bench # Run crystal benchmarks only
  cargo bench --bench quantum_bench # Run quantum benchmarks only
  cargo bench --bench dna_bench    # Run DNA benchmarks only

📈 Reports will be generated in:
  - Summary: lib/rust/benchmark_report.md
  - Detailed: lib/rust/target/criterion/report/index.html

Last updated: $(date -u +"%Y-%m-%d %H:%M:%S UTC")
Current user: $USER
"
