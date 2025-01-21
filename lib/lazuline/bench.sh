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
