#!/bin/bash

set -euo pipefail
IFS=$'\n\t'

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" &> /dev/null && pwd)"

# Initialize LD_LIBRARY_PATH if not set
: "${LD_LIBRARY_PATH:=}"
export LD_LIBRARY_PATH="${SCRIPT_DIR}/forge/lib${LD_LIBRARY_PATH:+:$LD_LIBRARY_PATH}"

print_spark() {
    echo -e "\033[0;35m✨ $1\033[0m"
}

print_glitch() {
    echo -e "\033[0;33m⚡ $1\033[0m"
}

cleanup() {
    local pids=$(jobs -p)
    [ -n "$pids" ] && kill $pids 2>/dev/null || true
}

trap cleanup EXIT INT TERM

run_zig_command() {
    print_spark "Running: $*"
    "$@"
}

cd "${SCRIPT_DIR}" || exit 1

# Default to "run" if no command provided
CMD=${1:-run}

case "$CMD" in
    "test")
        print_spark "Running tests..."
        run_zig_command zig test forge/zig/tests.zig
        ;;
    "run"|*)
        print_spark "Running crystal-space bridge..."
        run_zig_command zig build run
        ;;
esac
