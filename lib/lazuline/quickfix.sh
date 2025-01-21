#!/bin/bash

# Backup the original if it exists
if [ -f "prettybench.sh" ]; then
    cp prettybench.sh prettybench.sh.bak
fi

# Create the new prettybench.sh
cat > prettybench.sh << 'EOF'
#!/bin/bash
# prettybench.sh
# Created: 2025-01-21 19:52:25
# Author: isdood

# Create results directory if it doesn't exist
RESULTS_DIR="results"
mkdir -p "$RESULTS_DIR"

# Color array for spinner
COLORS=(
    '\033[38;5;196m'  # red
    '\033[38;5;202m'  # orange
    '\033[38;5;226m'  # yellow
    '\033[38;5;82m'   # green
    '\033[38;5;21m'   # blue
    '\033[38;5;93m'   # purple
)

# Static colors
ORANGE='\033[38;5;214m'
GREEN='\033[38;5;82m'
PURPLE='\033[38;5;135m'
RESET='\033[0m'

# Spinner configuration
SPINSTR='⠋⠙⠹⠸⠼⠴⠦⠧⠇⠏'
RAINBOW=true
DELAY=0.1

# Function to create progress bar
create_progress_bar() {
    local value=$1
    local max=$2
    local width=32
    local bars=$((value * width / max))
    printf "%${bars}s" | tr " " "#"
    printf "%$((width-bars))s" | tr " " "."
}

# Spinner function
spinner() {
    local pid=$1
    local length=${#SPINSTR}
    local color_index=0
    local color_length=${#COLORS[@]}

    tput civis
    while ps -p $pid > /dev/null; do
        for (( i=0; i<length; i++ )); do
            if [ "$RAINBOW" = true ]; then
                color_index=$(( (color_index + 1) % color_length ))
                printf "\r${COLORS[$color_index]}Running benchmarks ${SPINSTR:$i:1}${RESET}"
            else
                printf "\r${ORANGE}Running benchmarks ${SPINSTR:$i:1}${RESET}"
            fi
            sleep $DELAY
        done
    done
    tput cnorm
    printf "\r${GREEN}Benchmarks completed!${RESET}                     \n"
}

# Clear screen and show header
clear

# Header - exactly 49 chars between ║
echo -e "${PURPLE}╔═════════════════════════════════════════════════════╗${RESET}"
echo -e "${PURPLE}║${RESET}              Lazuline Benchmark Suite               ${PURPLE}║${RESET}"
echo -e "${PURPLE}╠═════════════════════════════════════════════════════╣${RESET}"
echo -e "${PURPLE}║${RESET} Started: 2025-01-21 19:52:25 UTC                    ${PURPLE}║${RESET}"
echo -e "${PURPLE}║${RESET} Runner:  isdood                                     ${PURPLE}║${RESET}"
echo -e "${PURPLE}╚═════════════════════════════════════════════════════╝${RESET}"
echo

# Run benchmarks
RUSTFLAGS="-C target-cpu=native" cargo bench > bench_output.tmp 2>&1 &
benchmark_pid=$!

# Start spinner
trap 'tput cnorm' EXIT
spinner $benchmark_pid

# Wait for benchmarks to complete
wait $benchmark_pid

RESULTS=$(cat bench_output.tmp)
INIT_TIME=$(echo "$RESULTS" | grep "initialization" | grep "time:" | awk -F'[\\[\\]]' '{print $2}')

# Display results - exactly 49 chars between ║
echo -e "${PURPLE}╔═════════════════════════════════════════════════════╗${RESET}"
echo -e "${PURPLE}║${RESET}                 Benchmark Results                    ${PURPLE}║${RESET}"
echo -e "${PURPLE}╠═════════════════════════════════════════════════════╣${RESET}"

echo -e "${PURPLE}║${RESET} ${GREEN}Initialization:${RESET}                                    ${PURPLE}║${RESET}"
printf "${PURPLE}║${RESET} %-11s [%-32s] ${PURPLE}║${RESET}\n" "$INIT_TIME" "$(create_progress_bar 1 1)"

echo -e "${PURPLE}╟─────────────────────────────────────────────────────╢${RESET}"
echo -e "${PURPLE}║${RESET} ${GREEN}Channel Compute:${RESET}                                  ${PURPLE}║${RESET}"
echo "$RESULTS" | grep "channel_compute/" | grep "time:" | while read -r line; do
    size=$(echo $line | grep -o "channel_compute/[0-9]*" | cut -d"/" -f2)
    time=$(echo $line | awk -F"[\\[\\]]" '{print $2}')
    printf "${PURPLE}║${RESET} Size %6d: %-11s [%-32s] ${PURPLE}║${RESET}\n" "$size" "$time" "$(create_progress_bar $size 100000)"
done

echo -e "${PURPLE}╟─────────────────────────────────────────────────────╢${RESET}"
echo -e "${PURPLE}║${RESET} ${GREEN}Multiple Operations:${RESET}                              ${PURPLE}║${RESET}"
echo "$RESULTS" | grep "multiple_operations/sequential" | grep "time:" | while read -r line; do
    ops=$(echo $line | grep -o "sequential/[0-9]*" | cut -d"/" -f2)
    time=$(echo $line | awk -F"[\\[\\]]" '{print $2}')
    printf "${PURPLE}║${RESET} Ops %7d: %-11s [%-32s] ${PURPLE}║${RESET}\n" "$ops" "$time" "$(create_progress_bar $ops 10)"
done

echo -e "${PURPLE}╟─────────────────────────────────────────────────────╢${RESET}"
echo -e "${PURPLE}║${RESET} ${GREEN}System Information:${RESET}                               ${PURPLE}║${RESET}"
CPU_INFO=$(grep "model name" /proc/cpuinfo | head -n1 | cut -d: -f2 | xargs)
MEM_INFO=$(free -h | awk '/^Mem:/ {print $2}')
RUST_VERSION=$(rustc --version)
printf "${PURPLE}║${RESET} %-45s ${PURPLE}║${RESET}\n" "CPU: ${CPU_INFO:0:41}..."
printf "${PURPLE}║${RESET} %-45s ${PURPLE}║${RESET}\n" "Memory: ${MEM_INFO} total"
printf "${PURPLE}║${RESET} %-45s ${PURPLE}║${RESET}\n" "Rust: ${RUST_VERSION}"

echo -e "${PURPLE}╚═════════════════════════════════════════════════════╝${RESET}"

# Clean up
rm bench_output.tmp
EOF

# Make both scripts executable
chmod +x prettybench.sh

echo "Updated prettybench.sh with:"
echo "- Current time: 2025-01-21 19:52:25"
echo "- Current user: isdood"
echo "- Fixed alignment (49 chars between borders)"
echo "- Rainbow spinner"
echo "- Proper box drawing"
echo "Original script backed up as prettybench.sh.bak"
