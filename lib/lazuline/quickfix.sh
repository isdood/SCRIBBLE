#!/bin/bash

cat > prettybench.sh << 'EOF'
#!/bin/bash
# Created: 2025-01-21 20:39:55
# Author: isdood

[... rest of your working script ...]

# After the benchmark results and before cleanup, add:
echo
echo -e "${PURPLE}╔${BOX}╗${RESET}"
echo -e "${PURPLE}║${RESET}                Export Results                  ${PURPLE}║${RESET}"
echo -e "${PURPLE}╠${BOX}╣${RESET}"
echo -e "${PURPLE}║${RESET} Saving to results/${RESULTS_FILE}             ${PURPLE}║${RESET}"
echo -e "${PURPLE}╚${BOX}╝${RESET}"

# Export results to timestamped file
RESULTS_FILE="benchmark-$(date +%Y%m%d-%H%M%S).txt"
mkdir -p results
{
    echo "Lazuline Benchmark Results"
    echo "========================="
    echo "Date: $(date -u +%Y-%m-%d\ %H:%M:%S) UTC"
    echo "Runner: isdood"
    echo
    echo "Initialization:"
    echo "$INIT_TIME"
    echo
    echo "Channel Compute:"
    echo "$RESULTS" | grep "channel_compute/" | grep "time:"
    echo
    echo "Multiple Operations:"
    echo "$RESULTS" | grep "multiple_operations/sequential" | grep "time:"
    echo
    echo "System Information:"
    echo "CPU: $CPU_INFO"
    echo "Memory: $MEM_INFO"
    echo "Rust: $RUST_VERSION"
} > "results/${RESULTS_FILE}"

# Clean up
rm bench_output.tmp
EOF

chmod +x prettybench.sh
echo "Updated prettybench.sh to include results export functionality"
