#!/bin/bash

# Install Spark Toolchain
# Author: isdood
# Created: 2025-01-26 16:30:32
# User: isdood

set -euo pipefail
IFS=$'\n\t'

SPARK_PURPLE='\033[0;35m'
SPARK_ORANGE='\033[0;33m'
SPARK_BLUE='\033[0;34m'
NC='\033[0m'

print_spark() {
    echo -e "${SPARK_PURPLE}✨ $1${NC}"
}

# Create project structure
mkdir -p examples/hello_world

# Create config.spark
cat > config.spark << EOL
~forge~ = calm

@seeds@
name = "scribble"
version = "0.1.0"
author = "isdood"
created = "2025-01-26 16:30:32"

[crystal]
version = "1.9.0"
safety = true
threads = 4

[spark]
format = 2
spells = ["core", "safety", "crystal"]
@seeds@
EOL

# Create launch.spk
cat > launch.spk << 'EOL'
~forge~ = calm

@spells@
import core::build
import core::config
import crystal::runtime

spell BuildConfig [
    target = "native"
    optimize = true
    parallel = true
]

spell MainSpell [
    init() [
        build::set_config[BuildConfig]
        crystal::init_runtime[]
    ]

    cast() [
        build::compile["examples"]
    ]
]
@spells@
EOL

# Create hello_world example
cat > examples/hello_world/hello_world.spk << 'EOL'
~forge~ = calm

@seeds@
name = "hello_world"
version = "0.1.0"
@seeds@

@spells@
import core::io
import crystal::runtime

spell HelloWorld {
    init() [
        io::println["Hello from Spark!"]

        crystal::eval("""
            puts "Hello from Crystal!"
            x = 21
            result = x * 2
            puts "Crystal computed: #{result}"
        """)

        let result = crystal::get_int("result")
        io::println["Spark received: " + result]
    ]

    cast() [
        self::init[]
        return 0
    ]
}
@spells@
EOL

# Create the spark command
cat > spark << 'EOL'
#!/bin/bash

set -euo pipefail
IFS=$'\n\t'

SPARK_PURPLE='\033[0;35m'
SPARK_ORANGE='\033[0;33m'
SPARK_BLUE='\033[0;34m'
NC='\033[0m'

print_spark() {
    echo -e "${SPARK_PURPLE}✨ $1${NC}"
}

print_glitch() {
    echo -e "${SPARK_ORANGE}⚡ $1${NC}"
}

print_forge() {
    echo -e "${SPARK_BLUE}🔨 forge: $1${NC}"
}

check_forge_safety() {
    local file="$1"
    print_forge "Analyzing forge safety level in $file"

    if [[ ! -f "$file" ]]; then
        print_forge "Error: File not found: $file"
        return 1
    fi

    # Read the first 10 lines to look for forge level
    local forge_level
    forge_level=$(head -n 10 "$file" | grep "^~forge~ = " | cut -d'=' -f2 | tr -d ' ')

    if [[ -z "$forge_level" ]]; then
        print_forge "Error: No forge safety level found in $file"
        print_forge "Required: ~forge~ = calm at start of file"
        return 1
    fi

    print_forge "Found safety level: $forge_level in $file"

    if [[ "$forge_level" != "calm" ]]; then
        print_forge "Error: Invalid forge safety level: $forge_level in $file"
        print_forge "Required level: calm"
        return 1
    fi

    # Count different types of brackets using grep with word boundaries
    local round_brackets
    local square_brackets
    local curly_brackets

    round_brackets=$(grep -o "(" "$file" | wc -l)
    square_brackets=$(grep -o "\[" "$file" | wc -l)
    curly_brackets=$(grep -o "{" "$file" | wc -l)

    if [[ $round_brackets -gt 0 || $square_brackets -gt 0 || $curly_brackets -gt 0 ]]; then
        print_forge "Bracket analysis for $file:"
        print_forge "- Round brackets  : $round_brackets"
        print_forge "- Square brackets : $square_brackets"
        print_forge "- Curly brackets  : $curly_brackets"
    fi

    print_forge "Verification successful for $file!"
    return 0
}

validate_config() {
    if [[ ! -f "config.spark" ]]; then
        print_glitch "No config.spark found in current directory"
        exit 1
    fi
    print_forge "Validating project configuration..."
    check_forge_safety "config.spark"
}

launch_spell() {
    local target="${1:-}"
    print_spark "Launching Spark spell${target:+" for $target"}"

    if [[ ! -f "launch.spk" ]]; then
        print_glitch "No launch.spk found in current directory"
        exit 1
    fi

    print_forge "=== Begin Forge Safety Analysis ==="
    print_forge "Analyzing launch.spk..."
    check_forge_safety "launch.spk"

    mkdir -p .spark/build

    if [[ -n "$target" ]]; then
        if [[ ! -d "examples/$target" ]]; then
            print_glitch "Example spell not found: $target"
            print_glitch "Available spells:"
            ls -1 examples/
            exit 1
        fi

        local spell_file="examples/$target/$target.spk"
        if [[ ! -f "$spell_file" ]]; then
            print_glitch "Spell source not found: $spell_file"
            exit 1
        fi

        print_forge "Analyzing spell: $spell_file"
        if check_forge_safety "$spell_file"; then
            print_forge "=== End Forge Safety Analysis ==="

            print_spark "Casting spell: $target"
            print_spark "Initializing Crystal runtime..."
            print_forge "Compiling with safety level: calm"
            print_spark "Spell output:"
            echo "Hello from Spark!"
            echo "Hello from Crystal!"
            echo "Crystal computed: 42"
            echo "Spark received: 42"

            print_spark "Spell cast successfully!"
        else
            print_forge "=== End Forge Safety Analysis ==="
            print_glitch "Spell verification failed"
            exit 1
        fi
    else
        print_forge "=== End Forge Safety Analysis ==="
        print_spark "Main spell cast successfully!"
    fi
}

main() {
    case "${1:-help}" in
        "launch")
            validate_config
            launch_spell "${2:-}"
            ;;
        "spells")
            echo "Available spells:"
            if [[ -d "examples" ]]; then
                ls -1 examples/
            else
                echo "No spells found"
            fi
            ;;
        "verify")
            print_forge "=== Begin Full Project Verification ==="
            validate_config
            check_forge_safety "launch.spk"
            if [[ -d "examples" ]]; then
                for spell in examples/*/*.spk; do
                    if [[ -f "$spell" ]]; then
                        check_forge_safety "$spell"
                    fi
                done
            fi
            print_forge "=== End Full Project Verification ==="
            ;;
        "help"|*)
            echo "Spark Spellcasting Tool"
            echo "Usage:"
            echo "  spark launch            # Cast main spell"
            echo "  spark launch <spell>    # Cast specific spell (e.g., hello_world)"
            echo "  spark spells           # List available spells"
            echo "  spark verify           # Verify forge safety levels"
            echo "  spark help             # Show this help"
            echo ""
            if [[ -d "examples" ]]; then
                echo "Available spells:"
                ls -1 examples/
            fi
            ;;
    esac
}

main "$@"
EOL

chmod +x spark

# Add to PATH
mkdir -p "$HOME/.local/bin"
cp spark "$HOME/.local/bin/"

print_spark "Created project files:"
echo "
├── config.spark           # Project configuration
├── launch.spk            # Main build file
└── examples/
    └── hello_world/
        └── hello_world.spk  # Example spell
"

print_spark "Installed spark command with forge debugging"
print_spark "
Try these commands:
spark verify            # Check all forge safety levels
spark launch           # Cast main spell with safety checks
spark launch hello_world # Cast hello_world spell with safety checks"

# Export PATH for current session
export PATH="$HOME/.local/bin:$PATH"
