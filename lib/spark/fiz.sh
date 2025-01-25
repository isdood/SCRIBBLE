#!/bin/bash

# Fiz - Spark Julia Runtime Terminal Emulator v0.1
# Author: isdood
# Created: 2025-01-25 22:25:26 UTC
# Repository: isdood/scribble

# Colors and formatting
BOLD='\033[1m'
BLUE='\033[0;34m'
GREEN='\033[0;32m'
YELLOW='\033[0;33m'
RED='\033[0;31m'
PURPLE='\033[0;35m'
NC='\033[0m'

# Get script directory for package location
SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
PKG_DIR="$SCRIPT_DIR/forge/julia/SparkJL"

print_colored() {
    local color=$1
    local text=$2
    echo -e "${color}${text}${NC}"
}

show_banner() {
    cat << "EOF"
    ⚡ 𝔽 𝕀 ℤ ⚡
    Spark Julia Runtime Terminal
    Version 0.1-alpha
EOF
}

check_requirements() {
    if ! command -v julia >/dev/null 2>&1; then
        print_colored $RED "Error: Julia is not installed"
        exit 1
    fi

    if [ ! -d "$PKG_DIR" ]; then
        print_colored $RED "Error: SparkJL package not found at $PKG_DIR"
        exit 1
    fi
}

create_sandbox_environment() {
    # Create temporary directory for sandbox environment
    SANDBOX_DIR=$(mktemp -d)
    print_colored $BLUE "Creating sandbox environment in $SANDBOX_DIR"

    # Create Project.toml for sandbox
    cat > "$SANDBOX_DIR/Project.toml" << EOL
name = "SparkSandbox"
uuid = "$(julia -e 'import UUIDs; println(UUIDs.uuid4())')"
version = "0.1.0"

[deps]
SparkJL = "d7891abc-4510-51ab-9240-a78b42f11234"
REPL = "3fa0cd96-eef1-5676-8a61-b3b8758bbffb"
EOL

    # Create sandbox initialization script
    cat > "$SANDBOX_DIR/init.jl" << 'EOL'
using SparkJL
using REPL

# Custom REPL mode for Fiz
struct FizMode <: REPL.AbstractREPL end

function create_fiz_repl()
    repl = Base.active_repl

    # Create Fiz REPL mode
    fiz_mode = LineEdit.Prompt(
        "fiz> ";
        prompt_prefix = "\e[34m",
        prompt_suffix = "\e[0m",
        on_enter = process_fiz_command
    )

    # Add help command
    LineEdit.keymap_dict(fiz_mode)['?'] = function (s,o...)
        println("""
        Fiz Commands:
        ?          - Show this help
        .crystal   - Create a new crystal structure
        .wave     - Create a new wave pattern
        .weave    - Apply weave pattern
        .optimize - Optimize current structure
        .exit     - Exit Fiz
        """)
        LineEdit.refresh_line(s)
    end

    # Push the mode to REPL
    main_mode = repl.interface.modes[1]
    push!(repl.interface.modes, fiz_mode)

    # Set keymap
    fiz_mode.keymap_dict = LineEdit.keymap([
        LineEdit.default_keymap,
        LineEdit.escape_defaults,
    ])
end

# Initialize environment
function init_fiz()
    println("⚡ Welcome to Fiz - Spark Julia Runtime Terminal ⚡")
    println("Type '?' for help\n")
    create_fiz_repl()
end

init_fiz()
EOL
}

start_fiz() {
    print_colored $GREEN "Starting Fiz terminal..."
    julia --project="$SANDBOX_DIR" -i "$SANDBOX_DIR/init.jl"
}

cleanup() {
    if [ -d "$SANDBOX_DIR" ]; then
        rm -rf "$SANDBOX_DIR"
    fi
}

main() {
    show_banner
    check_requirements
    create_sandbox_environment
    trap cleanup EXIT
    start_fiz
}

main
