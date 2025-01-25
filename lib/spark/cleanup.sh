#!/bin/bash
set -euo pipefail
IFS=$'\n\t'

echo "Cleaning up recursive clone..."

# Store our current absolute path
CURRENT_PATH=$(pwd)

# Go up to the original scribble root
cd ../../..

# Backup any unique sparkle files that might exist
BACKUP_DIR=$(mktemp -d)
echo "Backing up any unique files to: $BACKUP_DIR"

# Move only sparkle-specific files that aren't part of the recursive clone
for file in lib/sparkle/*; do
    if [[ ! -d "$file" || "$(basename "$file")" == "sparkle" ]]; then
        cp -r "$file" "$BACKUP_DIR/"
    fi
done

# Clean the recursive clone
rm -rf lib/sparkle
mkdir -p lib/sparkle

# Restore sparkle-specific files
cp -r "$BACKUP_DIR"/* lib/sparkle/ 2>/dev/null || true

# Create the basic module structure
cat > "lib/sparkle/mod.cr" << 'MOD'
module Scribble
  module Sparkle
    VERSION = "1.0.0"
    CREATED = "2025-01-25 03:19:14"
    TENDER  = "isdood"

    def self.root_path
      File.dirname(__FILE__)
    end

    def self.pattern_path
      File.join(root_path, "patterns")
    end

    def self.config_path
      File.join(root_path, "config.sparkle")
    end
  end
end
MOD

echo "Cleanup complete. Please check lib/sparkle for the correct structure."
echo "Your backup files are in: $BACKUP_DIR"
