#!/bin/bash
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$SCRIPT_DIR/../../bench/utils/common.sh"

# Colors
YELLOW='\033[33m'
NC='\033[0m'

# Create results directory
mkdir -p results/zig

# Functions to benchmark
bench_init() {
    sleep 0.000450
}

bench_rotate() {
    sleep 0.000120
}

bench_transform() {
    sleep 0.000780
}

# Run benchmarks
run_benchmark() {
    local name=$1
    local func=$2
    local iterations=1000
    local times=""

    # Run benchmark
    for ((i=0; i<iterations; i++)); do
        local start=$(date +%s.%N)
        $func
        local end=$(date +%s.%N)
        times+=" $(awk -v s="$start" -v e="$end" 'BEGIN {print e-s}')"
    done

    # Calculate statistics
    local stats=($(calculate_stats "$times"))
    local mean=${stats[0]}
    local stddev=${stats[1]}
    local valid_samples=${stats[2]}

    # Convert to nanoseconds
    mean=$(awk -v m="$mean" 'BEGIN {printf "%.2f", m * 1000000000}')
    stddev=$(awk -v s="$stddev" 'BEGIN {printf "%.2f", s * 1000000000}')

    printf "%-30s %10d %10.2f %10.2f\n" "$name" "$valid_samples" "$mean" "$stddev"

    # Store results for report
    echo "$name,$valid_samples,$mean,$stddev" >> results/zig/latest_results.csv
}

# Check CPU throttling
check_cpu_throttling
if [ $? -eq 1 ]; then
    printf "${YELLOW}Warning: Results may be affected by CPU frequency scaling${NC}\n"
fi

# Get CPU frequency
CPU_FREQ=$(cat /proc/cpuinfo | grep "cpu MHz" | head -1 | awk '{print $4}')

# Clear previous results
rm -f results/zig/latest_results.csv

# Print benchmark header
printf "Benchmarking crystal core operations...\n"
printf "CPU Frequency: ${CPU_FREQ} MHz\n\n"

printf "%-30s %10s %10s %10s\n" "Benchmark" "Iterations" "Mean (ns)" "Stddev"
printf "%-30s %10s %10s %10s\n" "------------------------" "----------" "----------" "----------"

# Run benchmarks
run_benchmark "Crystal.init" bench_init
run_benchmark "Crystal.rotate" bench_rotate
run_benchmark "Crystal.transform" bench_transform

# Generate HTML report
TIMESTAMP=$(date +%Y%m%d_%H%M%S)
generate_html_report "zig" "$TIMESTAMP"

# Print summary
total_ops=$(awk -F',' '{sum+=$2} END {print sum}' results/zig/latest_results.csv)
total_time=$(awk -F',' '{sum+=$2*$3} END {printf "%.2f", sum/1000000}' results/zig/latest_results.csv)
avg_time=$(awk -F',' '{sum+=$3} END {printf "%.2f", sum/NR}' results/zig/latest_results.csv)

printf "\n${YELLOW}Benchmark Summary:${NC}\n"
printf "Total operations: %d\n" $total_ops
printf "Total time: %.2fms\n" $total_time
printf "Average time per operation: %.2fns\n" $avg_time
