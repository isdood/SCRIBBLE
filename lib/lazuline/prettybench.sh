#!/bin/bash
# Created: 2025-01-21 20:32:35
# Author: isdood

# Colors and spinner setup
COLORS=(
    '\033[38;5;196m' '\033[38;5;202m' '\033[38;5;226m'
    '\033[38;5;82m'  '\033[38;5;21m'  '\033[38;5;93m'
)
GREEN='\033[38;5;82m'
PURPLE='\033[38;5;135m'
RESET='\033[0m'
SPINSTR='⠋⠙⠹⠸⠼⠴⠦⠧⠇⠏'
RAINBOW=true

# Box width (exactly 50 characters total)
BOX="═══════════════════════════════════════════════════"

# Spinner function
spinner() {
    local pid=$1
    local delay=0.1
    local spinlen=${#SPINSTR}
    local cidx=0
    local clen=${#COLORS[@]}

    tput civis
    while ps -p $pid > /dev/null; do
        for (( i=0; i<spinlen; i++ )); do
            if [ "$RAINBOW" = true ]; then
                cidx=$(( (cidx + 1) % clen ))
                printf "\r${COLORS[$cidx]}Running benchmarks %s${RESET}" "${SPINSTR:$i:1}"
            else
                printf "\r${ORANGE}Running benchmarks %s${RESET}" "${SPINSTR:$i:1}"
            fi
            sleep $delay
        done
    done
    tput cnorm
    printf "\r${GREEN}Benchmarks completed!${RESET}                     \n"
}

# Progress bar function
create_progress_bar() {
    local value=$1
    local max=$2
    local width=32
    local bars=$((value * width / max))
    printf "%${bars}s" | tr " " "#"
    printf "%$((width-bars))s" | tr " " "."
}

# Clear and show header
clear
echo -e "${PURPLE}╔${BOX}╗${RESET}"
echo -e "${PURPLE}║${RESET}           Lazuline Benchmark Suite            ${PURPLE}    ║${RESET}"
echo -e "${PURPLE}╠${BOX}╣${RESET}"
echo -e "${PURPLE}║${RESET} Started: 2025-01-21 20:32:35 UTC             ${PURPLE}     ║${RESET}"
echo -e "${PURPLE}║${RESET} Runner:  isdood                              ${PURPLE}     ║${RESET}"
echo -e "${PURPLE}╚${BOX}╝${RESET}"
echo

# Run benchmarks
RUSTFLAGS="-C target-cpu=native" cargo bench > bench_output.tmp 2>&1 &
benchmark_pid=$!

# Start spinner
trap 'tput cnorm' EXIT
spinner $benchmark_pid

# Wait for benchmarks to complete
wait $benchmark_pid

# Parse results
RESULTS=$(cat bench_output.tmp)
INIT_TIME=$(echo "$RESULTS" | grep "initialization" | grep "time:" | awk '{gsub(/[\[\]]/, ""); print $2}')

# Display results
echo -e "${PURPLE}╔${BOX}╗${RESET}"
echo -e "${PURPLE}║${RESET}              Benchmark Results                ${PURPLE}║${RESET}"
echo -e "${PURPLE}╠${BOX}╣${RESET}"
echo -e "${PURPLE}║${RESET} ${GREEN}Initialization:${RESET}                         ${PURPLE}║${RESET}"
printf "${PURPLE}║${RESET} %-43s ${PURPLE}║${RESET}\n" "$INIT_TIME [$(create_progress_bar 1 1)]"

echo -e "${PURPLE}╟${BOX/═/-}╢${RESET}"
echo -e "${PURPLE}║${RESET} ${GREEN}Channel Compute:${RESET}                       ${PURPLE}║${RESET}"
echo "$RESULTS" | grep "channel_compute/" | grep "time:" | while read -r line; do
    size=$(echo $line | grep -o "channel_compute/[0-9]*" | cut -d"/" -f2)
    time=$(echo $line | awk '{gsub(/[\[\]]/, ""); print $2}')
    printf "${PURPLE}║${RESET} Size %6d: %-29s ${PURPLE}║${RESET}\n" "$size" "[$(create_progress_bar $size 100000)]"
done

echo -e "${PURPLE}╟${BOX/═/-}╢${RESET}"
echo -e "${PURPLE}║${RESET} ${GREEN}Multiple Operations:${RESET}                   ${PURPLE}║${RESET}"
echo "$RESULTS" | grep "multiple_operations/sequential" | grep "time:" | while read -r line; do
    ops=$(echo $line | grep -o "sequential/[0-9]*" | cut -d"/" -f2)
    time=$(echo $line | awk '{gsub(/[\[\]]/, ""); print $2}')
    printf "${PURPLE}║${RESET} Ops %7d: %-29s ${PURPLE}║${RESET}\n" "$ops" "[$(create_progress_bar $ops 10)]"
done

echo -e "${PURPLE}╟${BOX/═/-}╢${RESET}"
echo -e "${PURPLE}║${RESET} ${GREEN}System Information:${RESET}                    ${PURPLE}║${RESET}"
CPU_INFO=$(grep "model name" /proc/cpuinfo | head -n1 | cut -d: -f2 | xargs)
MEM_INFO=$(free -h | awk '/^Mem:/ {print $2}')
RUST_VERSION=$(rustc --version)
printf "${PURPLE}║${RESET} %-43s ${PURPLE}║${RESET}\n" "CPU: ${CPU_INFO:0:39}..."
printf "${PURPLE}║${RESET} %-43s ${PURPLE}║${RESET}\n" "Memory: ${MEM_INFO} total"
printf "${PURPLE}║${RESET} %-43s ${PURPLE}║${RESET}\n" "Rust: ${RUST_VERSION}"

echo -e "${PURPLE}╚${BOX}╝${RESET}"

# Clean up
rm bench_output.tmp
