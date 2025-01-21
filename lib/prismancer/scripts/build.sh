#!/bin/bash

# build.sh - Prismancer Engine Build Script
# Created: 2025-01-21 18:20:11 UTC
# Author: isdood

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Build configuration
BUILD_TYPE="${1:-release}"  # default to release build
BUILD_DIR="build"
PARALLEL_JOBS=$(nproc)

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
    log "Checking build dependencies..."

    local missing_deps=()

    # Required build tools
    for cmd in cargo zig julia gcc cmake pkg-config; do
        if ! command -v "$cmd" &> /dev/null; then
            missing_deps+=("$cmd")
        fi
    done

    if [ ${#missing_deps[@]} -ne 0 ]; then
        handle_error "Missing dependencies: ${missing_deps[*]}"
    fi

    log "All build dependencies found" "$GREEN"
}

# Prepare build environment
prepare_build() {
    log "Preparing build environment..."

    # Create build directories
    mkdir -p "$BUILD_DIR"/{rust,zig,julia}

    # Clean previous builds if needed
    if [ "$1" = "clean" ]; then
        log "Cleaning previous builds..."
        cargo clean
        rm -rf zig-cache
        rm -rf "$BUILD_DIR"
        mkdir -p "$BUILD_DIR"/{rust,zig,julia}
    fi
}

# Build Rust components
build_rust() {
    log "Building Rust components..."

    local rust_flags=""
    if [ "$BUILD_TYPE" = "release" ]; then
        rust_flags="--release"
    fi

    # Build core engine components
    RUSTFLAGS="-C target-cpu=native" cargo build $rust_flags || \
        handle_error "Rust build failed"

    # Copy artifacts
    cp target/$BUILD_TYPE/libprismancer.* "$BUILD_DIR/rust/" 2>/dev/null || true

    log "Rust components built successfully" "$GREEN"
}

# Build Zig components
build_zig() {
    log "Building Zig components..."

    local zig_flags=""
    if [ "$BUILD_TYPE" = "release" ]; then
        zig_flags="-Drelease-fast"
    fi

    # Build low-level components
    zig build $zig_flags || handle_error "Zig build failed"

    # Copy artifacts
    cp zig-out/lib/* "$BUILD_DIR/zig/" 2>/dev/null || true

    log "Zig components built successfully" "$GREEN"
}

# Build Julia components
build_julia() {
    log "Building Julia components..."

    # Compile Julia physics components
    julia --project=. -e '
        using Pkg;
        Pkg.instantiate();
        include("src/physics/build.jl")
    ' || handle_error "Julia build failed"

    # Copy artifacts
    cp src/physics/*.so "$BUILD_DIR/julia/" 2>/dev/null || true

    log "Julia components built successfully" "$GREEN"
}

# Generate FFI bindings
generate_bindings() {
    log "Generating FFI bindings..."

    # Generate Rust FFI bindings
    cbindgen --config cbindgen.toml --output include/prismancer.h . || \
        handle_error "Failed to generate C bindings"

    log "FFI bindings generated successfully" "$GREEN"
}

# Link everything together
link_components() {
    log "Linking components..."

    # Create the main library
    gcc -shared \
        -o "$BUILD_DIR/libprismancer.so" \
        -Wl,--whole-archive \
        "$BUILD_DIR"/*/lib* \
        -Wl,--no-whole-archive \
        -lvulkan \
        || handle_error "Linking failed"

    log "Components linked successfully" "$GREEN"
}

# Run tests if requested
run_tests() {
    if [ "$1" = "test" ]; then
        log "Running tests..."
        ./scripts/test.sh || handle_error "Tests failed"
        log "All tests passed" "$GREEN"
    fi
}

# Generate build report
generate_report() {
    log "Generating build report..."

    {
        echo "# Prismancer Build Report"
        echo "Generated: $(date -u)"
        echo "Build Type: $BUILD_TYPE"
        echo "Builder: $USER"
        echo
        echo "## Build Information"
        echo "- Rust Version: $(cargo --version)"
        echo "- Zig Version: $(zig version)"
        echo "- Julia Version: $(julia --version)"
        echo
        echo "## Build Artifacts"
        echo "\`\`\`"
        find "$BUILD_DIR" -type f -name "lib*" | sed 's|^|./|'
        echo "\`\`\`"
    } > "$BUILD_DIR/build_report.md"

    log "Build report generated: $BUILD_DIR/build_report.md" "$GREEN"
}

# Main build flow
main() {
    log "Starting Prismancer build process..."

    check_dependencies
    prepare_build "${2:-}"  # Pass clean flag if specified

    # Build components
    build_rust
    build_zig
    build_julia

    # Generate bindings and link
    generate_bindings
    link_components

    # Optional test run
    run_tests "${2:-}"

    # Generate report
    generate_report

    log "Build completed successfully" "$GREEN"
    log "Build artifacts available in: $BUILD_DIR" "$YELLOW"
}

# Execute main function with all arguments
main "$@"
