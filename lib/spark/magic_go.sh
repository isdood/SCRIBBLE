#!/usr/bin/env bash
# magic_go.sh - Spark Language Implementation
# Author: isdood
# Created: 2025-01-25 13:28:54

set -euo pipefail

# Project-relative paths
SPARK_ROOT="$(pwd)"
SPARK_BIN="$SPARK_ROOT/bin"
SPARK_CORE="$SPARK_ROOT/core"
SPARK_TOOLS="$SPARK_ROOT/tools"
SPARK_STD="$SPARK_ROOT/std"

# User-specific paths
USER_SPARK_HOME="$HOME/.spark"
USER_SPARK_GARDEN="$USER_SPARK_HOME/garden"
USER_SPARK_CACHE="$USER_SPARK_HOME/cache"

# Colors for whimsical output
MAGIC_PURPLE='\033[0;35m'
MAGIC_GREEN='\033[0;32m'
NC='\033[0m'

echo_magic() {
    echo -e "${MAGIC_PURPLE}âœ¨ $1${NC}"
}

setup_project_structure() {
    echo_magic "Creating Spark project structure..."
    
    # Create project directories
    mkdir -p "$SPARK_BIN" "$SPARK_CORE" "$SPARK_TOOLS" "$SPARK_STD"
    
    # Create standard library modules
    mkdir -p "$SPARK_STD/math" "$SPARK_STD/string" "$SPARK_STD/io"
    
    # Create user directories
    mkdir -p "$USER_SPARK_GARDEN" "$USER_SPARK_CACHE"
}

create_seed_fish() {
    cat > "$SPARK_BIN/seed.fish" << 'EOF'
function seed
    if test (count $argv) -lt 2
        echo "Usage: seed [plant|unplant] package**name"
        return 1
    end

    set -l action $argv[1]
    set -l package $argv[2]
    set -l package_clean (string replace --all '**' '/' $package)
    set -l package_path "$HOME/.spark/garden/$package_clean"
    
    switch $action
        case "plant"
            echo "ðŸŒ Planting $package..."
            if string match -q "std*" $package
                set -l std_name (string replace "std**" "" $package)
                set -l std_path "$SPARK_ROOT/std/$std_name"
                mkdir -p (dirname $package_path)
                ln -sf $std_path $package_path
            else
                mkdir -p $package_path
            end
        case "unplant"
            echo "ðŸ¥ Unplanting $package..."
            rm -rf $package_path
        case '*'
            echo "ðŸŒ Unknown magical command: $action"
            return 1
    end
end
EOF
}

create_forge_fish() {
    cat > "$SPARK_BIN/forge.fish" << 'EOF'
function forge
    if test (count $argv) -lt 1
        echo "Usage: forge [brew|enchant|test] [project_directory]"
        return 1
    end

    set -l action $argv[1]
    set -l project_dir "."
    test (count $argv) -gt 1; and set project_dir $argv[2]
    set -l launch_file "$project_dir/launch.spk"
    
    if not test -f $launch_file
        echo "âŒ No launch.spk found in $project_dir"
        return 1
    end
    
    switch $action
        case "brew"
            echo "ðŸ” Brewing project..."
            eval "$SPARK_CORE/compiler/forge build $launch_file"
        case "enchant"
            echo "âœ¨ Enchanting project..."
            eval "$SPARK_CORE/runtime/spark run $launch_file"
        case "test"
            echo "ðŸŽ Testing spells..."
            eval "$SPARK_CORE/test/runner $launch_file"
        case '*'
            echo "Unknown forge command: $action"
            return 1
    end
end
EOF
}

create_example_launch_spk() {
    mkdir -p "$SPARK_ROOT/examples/basic"
    cat > "$SPARK_ROOT/examples/basic/launch.spk" << 'EOF'
@launch@
name: "spark_example"
version: "0.1.0"

@ingredients@
std**math
std**string
@ingredients@

@brew@
target: "sparkle"
safety: "careful"
optimization: "quick"
@brew@
EOF
}

setup_fish_integration() {
    mkdir -p "$HOME/.config/fish/conf.d"
    
    cat > "$HOME/.config/fish/conf.d/spark.fish" << EOF
# Spark Fish Shell Integration
set -gx SPARK_ROOT "$SPARK_ROOT"
set -gx SPARK_BIN "$SPARK_BIN"

# Source the Spark fish functions
source "$SPARK_BIN/seed.fish"
source "$SPARK_BIN/forge.fish"

# Define universal variables
set -U spark_installed true

# Spark aliases
alias spk 'forge enchant'
alias spkb 'forge brew'
alias spkt 'forge test'
EOF
}

# Main implementation
main() {
    echo_magic "ðŸŒ Setting up Spark in $SPARK_ROOT"
    
    setup_project_structure
    create_seed_fish
    create_forge_fish
    create_example_launch_spk
    setup_fish_integration
    
    echo_magic "Setup complete! ðŸŽ"
    echo_magic "Restart your shell or run: source ~/.config/fish/conf.d/spark.fish"
    echo_magic "Try: seed plant std**math"
}

main "$@"
