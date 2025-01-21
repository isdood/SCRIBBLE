#!/bin/bash

# benchmark.sh - Prismancer Engine Benchmark Suite
# Created: 2025-01-21 18:16:57 UTC
# Author: isdood

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Timestamp for results
TIMESTAMP=$(date -u +"%Y-%m-%d_%H-%M-%S")
RESULTS_DIR="benches/results/${TIMESTAMP}"
mkdir -p "${RESULTS_DIR}"

# Configuration
ITERATIONS=1000
WARMUP_ITERATIONS=100
MIN_COHERENCE=0.85
TARGET_COHERENCE=0.95

# Logging function
log() {
    echo -e "${2:-$BLUE}[$(date -u +"%Y-%m-%d %H:%M:%S UTC")] $1${NC}"
}

# Error handling
handle_error() {
    log "Error: $1" "$RED"
    exit 1
}

# Check required tools
check_dependencies() {
    log "Checking dependencies..."

    local missing_deps=()

    # Check for required commands
    for cmd in cargo zig julia hyperfine; do
        if ! command -v "$cmd" &> /dev/null; then
            missing_deps+=("$cmd")
        fi
    done

    if [ ${#missing_deps[@]} -ne 0 ]; then
        handle_error "Missing dependencies: ${missing_deps[*]}"
    fi

    log "All dependencies found" "$GREEN"
}

# Prepare benchmark environment
prepare_environment() {
    log "Preparing benchmark environment..."

    # Clean previous builds
    cargo clean
    rm -rf zig-cache

    # Build in release mode
    cargo build --release || handle_error "Failed to build Rust components"
    zig build -Drelease-fast || handle_error "Failed to build Zig components"

    log "Environment prepared" "$GREEN"
}

# Memory benchmarks
benchmark_memory() {
    log "Running memory benchmarks..."

    hyperfine --warmup "$WARMUP_ITERATIONS" \
              --runs "$ITERATIONS" \
              --export-json "${RESULTS_DIR}/memory.json" \
              --prepare "sync; echo 3 > /proc/sys/vm/drop_caches" \
              "./target/release/prismancer_memory_bench"

    log "Memory benchmarks completed" "$GREEN"
}

# Crystal coherence benchmarks
benchmark_coherence() {
    log "Running crystal coherence benchmarks..."

    for coherence in $(seq "$MIN_COHERENCE" 0.05 "$TARGET_COHERENCE"); do
        hyperfine --warmup "$WARMUP_ITERATIONS" \
                  --runs "$ITERATIONS" \
                  --export-json "${RESULTS_DIR}/coherence_${coherence}.json" \
                  "./target/release/prismancer_crystal_bench --coherence $coherence"
    done

    log "Coherence benchmarks completed" "$GREEN"
}

# Physics engine benchmarks
benchmark_physics() {
    log "Running physics benchmarks..."

    local scenarios=(
        "basic_collision"
        "quantum_interaction"
        "wave_propagation"
        "crystal_lattice"
    )

    for scenario in "${scenarios[@]}"; do
        hyperfine --warmup "$WARMUP_ITERATIONS" \
                  --runs "$ITERATIONS" \
                  --export-json "${RESULTS_DIR}/physics_${scenario}.json" \
                  "./target/release/prismancer_physics_bench --scenario $scenario"
    done

    log "Physics benchmarks completed" "$GREEN"
}

# Rendering benchmarks
benchmark_rendering() {
    log "Running rendering benchmarks..."

    local batch_sizes=(1000 10000 100000)

    for size in "${batch_sizes[@]}"; do
        hyperfine --warmup "$WARMUP_ITERATIONS" \
                  --runs "$ITERATIONS" \
                  --export-json "${RESULTS_DIR}/render_batch_${size}.json" \
                  "./target/release/prismancer_render_bench --batch-size $size"
    done

    log "Rendering benchmarks completed" "$GREEN"
}

# Cache efficiency benchmarks
benchmark_cache() {
    log "Running cache benchmarks..."

    local cache_sizes=(
        "128M"
        "512M"
        "1G"
    )

    for size in "${cache_sizes[@]}"; do
        hyperfine --warmup "$WARMUP_ITERATIONS" \
                  --runs "$ITERATIONS" \
                  --export-json "${RESULTS_DIR}/cache_${size}.json" \
                  "./target/release/prismancer_cache_bench --size $size"
    done

    log "Cache benchmarks completed" "$GREEN"
}

# Generate report
generate_report() {
    log "Generating benchmark report..."

    {
        echo "# Prismancer Benchmark Report"
        echo "Generated: $(date -u)"
        echo "Git Commit: $(git rev-parse HEAD)"
        echo
        echo "## System Information"
        echo "\`\`\`"
        uname -a
        echo "\`\`\`"
        echo
        echo "## Results Summary"

        # Process results using jq
        for result in "${RESULTS_DIR}"/*.json; do
            echo "### $(basename "$result" .json)"
            jq -r '.results[] | "- Mean: \(.mean) ±\(.stddev) seconds"' "$result"
            echo
        done

    } > "${RESULTS_DIR}/report.md"

    log "Report generated: ${RESULTS_DIR}/report.md" "$GREEN"
}

# Main benchmark flow
main() {
    log "Starting Prismancer benchmark suite..."

    check_dependencies
    prepare_environment

    # Run benchmarks
    benchmark_memory
    benchmark_coherence
    benchmark_physics
    benchmark_rendering
    benchmark_cache

    # Generate report
    generate_report

    log "Benchmark suite completed successfully" "$GREEN"
    log "Results available in: ${RESULTS_DIR}" "$YELLOW"
}

# Execute main function
main "$@"
