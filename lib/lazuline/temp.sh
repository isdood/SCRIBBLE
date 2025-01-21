#!/bin/bash

# prettybench.sh
# Created: 2025-01-21 20:28:19
# Author: isdood

# [Keep existing color definitions and functions]

clear

# Fixed width box drawing
echo -e "${PURPLE}╔════════════════════════════════════════════════════╗${RESET}"
echo -e "${PURPLE}║              Lazuline Benchmark Suite              ║${RESET}"
echo -e "${PURPLE}╠════════════════════════════════════════════════════╣${RESET}"
echo -e "${PURPLE}║ Started: 2025-01-21 20:28:19 UTC                  ║${RESET}"
echo -e "${PURPLE}║ Runner:  isdood                                   ║${RESET}"
echo -e "${PURPLE}╚════════════════════════════════════════════════════╝${RESET}"

# Header - exactly 51 characters between ║

# Run benchmarks
RUSTFLAGS="-C target-cpu=native" cargo bench > bench_output.tmp 2>&1 &
benchmark_pid=$!

# Start spinner
trap 'tput cnorm' EXIT
spinner $benchmark_pid

# Wait for benchmarks to complete
wait $benchmark_pid

RESULTS=$(cat bench_output.tmp)
INIT_TIME=$(echo "$RESULTS" | grep "initialization" | grep "time:" | awk '{gsub(/[\[\]]/, ""); print $2, $3, $4}')


# Display results
echo -e "${PURPLE}╔════════════════════════════════════════════════════╗${RESET}"
echo -e "${PURPLE}║               Benchmark
