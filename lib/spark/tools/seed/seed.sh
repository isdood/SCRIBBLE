#!/usr/bin/env bash

# Spark Seed Manager Wrapper
# Handles ** package syntax in both bash and fish shells

SEED_BIN="$(dirname "$0")/zig-out/bin/seed"

# Function to escape ** in package names
escape_package() {
    echo "$1" | sed 's/\*\*/@@/g'
}

if [ "$1" = "plant" ] || [ "$1" = "unplant" ]; then
    command="$1"
    # Replace ** with @@ temporarily to bypass shell globbing
    package=$(escape_package "$2")
    # Restore ** when passing to the binary
    package="${package//@@/**}"
    "$SEED_BIN" "$command" "$package"
else
    "$SEED_BIN" "$@"
fi
