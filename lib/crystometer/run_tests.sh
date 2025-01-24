#!/bin/bash

echo "🔮 Running Crystometer test suite..."

echo "🦀 Running Rust tests..."
cargo test

echo "⚡ Running Zig tests..."
zig build test

echo "💎 Running Julia tests..."
julia --project=julia -e 'using Pkg; Pkg.test()'

echo "✨ Test suite complete!"
