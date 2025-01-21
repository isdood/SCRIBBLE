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
