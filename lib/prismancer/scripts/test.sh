#!/bin/bash

# test.sh - Prismancer Engine Test Suite
# Created: 2025-01-21 18:35:56 UTC
# Author: isdood

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Test configuration
TEST_DIR="tests"
RESULTS_DIR="tests/results/$(date -u +"%Y-%m-%d_%H-%M-%S")"
PARALLEL_JOBS=$(nproc)
LOG_FILE="${RESULTS_DIR}/test.log"

# Logging function
log() {
    echo -e "${2:-$BLUE}[$(date -u +"%Y-%m-%d %H:%M:%S UTC")] $1${NC}" | tee -a "$LOG_FILE"
}

# Error handling
handle_error() {
    log "Error: $1" "$RED"
    exit 1
}

# Initialize test environment
init_test_env() {
    log "Initializing test environment..."

    # Create results directory
    mkdir -p "$RESULTS_DIR"
    touch "$LOG_FILE"

    # Record system information
    {
        echo "Prismancer Test Run"
        echo "==================="
        echo "Date: $(date -u)"
        echo "User: $USER"
        echo "System: $(uname -a)"
        echo "Rust: $(cargo --version)"
        echo "Zig: $(zig version)"
        echo "Julia: $(julia --version)"
        echo "==================="
        echo
    } >> "$LOG_FILE"
}

# Run Rust tests
run_rust_tests() {
    log "Running Rust component tests..."

    local modules=(
        "core"
        "render"
        "physics"
        "systems"
        "parallel"
    )

    for module in "${modules[@]}"; do
        log "Testing module: $module"
        RUST_BACKTRACE=1 cargo test --package "prismancer-${module}" -- --nocapture \
            >> "$LOG_FILE" 2>&1 || handle_error "Rust tests failed for module: $module"
    done

    # Run doc tests
    cargo test --doc >> "$LOG_FILE" 2>&1 || handle_error "Documentation tests failed"

    log "Rust tests completed successfully" "$GREEN"
}

# Run Zig tests
run_zig_tests() {
    log "Running Zig component tests..."

    local components=(
        "memory"
        "geometry"
        "cache"
        "vulkan"
    )

    for component in "${components[@]}"; do
        log "Testing component: $component"
        zig test "src/low_level/${component}.zig" \
            >> "$LOG_FILE" 2>&1 || handle_error "Zig tests failed for component: $component"
    done

    log "Zig tests completed successfully" "$GREEN"
}

# Run Julia tests
run_julia_tests() {
    log "Running Julia component tests..."

    julia --project=. -e '
        using Pkg;
        Pkg.test();
    ' >> "$LOG_FILE" 2>&1 || handle_error "Julia tests failed"

    log "Julia tests completed successfully" "$GREEN"
}

# Run integration tests
run_integration_tests() {
    log "Running integration tests..."

    local test_cases=(
        "crystal_physics"
        "quantum_render"
        "memory_coherence"
        "full_pipeline"
    )

    for test_case in "${test_cases[@]}"; do
        log "Running integration test: $test_case"
        "./tests/integration/${test_case}" \
            >> "$LOG_FILE" 2>&1 || handle_error "Integration test failed: $test_case"
    done

    log "Integration tests completed successfully" "$GREEN"
}

# Run performance tests
run_performance_tests() {
    if [ "${RUN_PERF_TESTS}" = "true" ]; then
        log "Running performance tests..."

        cargo bench >> "$LOG_FILE" 2>&1 || handle_error "Performance tests failed"

        log "Performance tests completed successfully" "$GREEN"
    fi
}

# Check code formatting
check_formatting() {
    log "Checking code formatting..."

    # Check Rust formatting
    cargo fmt -- --check >> "$LOG_FILE" 2>&1 || handle_error "Rust formatting check failed"

    # Check Zig formatting
    find . -name "*.zig" -exec zig fmt --check {} \; \
        >> "$LOG_FILE" 2>&1 || handle_error "Zig formatting check failed"

    log "Code formatting check passed" "$GREEN"
}

# Run static analysis
run_static_analysis() {
    log "Running static analysis..."

    # Rust clippy
    cargo clippy --all-targets --all-features -- -D warnings \
        >> "$LOG_FILE" 2>&1 || handle_error "Rust static analysis failed"

    log "Static analysis passed" "$GREEN"
}

# Generate test report
generate_report() {
    log "Generating test report..."

    {
        echo "# Prismancer Test Report"
        echo "Generated: $(date -u)"
        echo "User: $USER"
        echo
        echo "## Test Summary"
        echo "- Rust Tests: $(grep -c 'test result: ok' "$LOG_FILE") passed"
        echo "- Zig Tests: $(grep -c 'All tests passed' "$LOG_FILE") components tested"
        echo "- Julia Tests: $(grep -c 'Test Summary' "$LOG_FILE") test sets completed"
        echo "- Integration Tests: $(grep -c 'integration test' "$LOG_FILE") scenarios verified"
        echo
        echo "## Performance Metrics"
        if [ "${RUN_PERF_TESTS}" = "true" ]; then
            grep -A 5 "Benchmark Results" "$LOG_FILE" || echo "No performance data available"
        else
            echo "Performance tests were not run"
        fi

    } > "${RESULTS_DIR}/report.md"

    log "Test report generated: ${RESULTS_DIR}/report.md" "$GREEN"
}

# Main test flow
main() {
    log "Starting Prismancer test suite..."

    # Parse command line arguments
    RUN_PERF_TESTS=false
    while getopts "p" opt; do
        case $opt in
            p) RUN_PERF_TESTS=true ;;
            *) echo "Usage: $0 [-p]" >&2
               exit 1 ;;
        esac
    done

    init_test_env

    # Run all tests
    check_formatting
    run_static_analysis
    run_rust_tests
    run_zig_tests
    run_julia_tests
    run_integration_tests
    run_performance_tests

    generate_report

    log "Test suite completed successfully" "$GREEN"
    log "Test results available in: $RESULTS_DIR" "$YELLOW"
}

# Execute main function
main "$@"
