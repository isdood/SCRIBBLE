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
