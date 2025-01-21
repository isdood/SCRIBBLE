#!/usr/bin/env bash

# Lazuline development environment setup
source /etc/profile.d/rust.sh
export JULIA_NUM_THREADS=auto
export ZIG_CACHE_DIR="$HOME/.cache/zig"
export LAZULINE_CONFIG_DIR="${XDG_CONFIG_HOME:-$HOME/.config}/lazuline"

# Development aliases
alias lz-build='cargo build --release && zig build -Doptimize=ReleaseSafe'
alias lz-test='cargo test && zig build test && julia --project=julia test/runtests.jl'
alias lz-bench='cargo bench && julia --project=julia bench/runbench.jl'
