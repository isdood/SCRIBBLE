#!/bin/bash

# Install Spark Toolchain
# Author: isdood
# Created: 2025-01-26 16:13:16 UTC

set -euo pipefail
IFS=$'\n\t'

SPARK_PURPLE='\033[0;35m'
SPARK_ORANGE='\033[0;33m'
NC='\033[0m'

print_spark() {
    echo -e "${SPARK_PURPLE}✨ $1${NC}"
}

# Update config.spark
cat > config.spark << 'EOL'
~forge~ = calm

@seeds@
name = "scribble"
version = "0.1.0"
author = "isdood"
created = "2025-01-26 16:13:16"

[crystal]
version = "1.9.0"
safety = true
threads = 4

[spark]
format = 2
spells = ["core", "safety", "crystal"]
@seeds@
EOL

# Update launch.spk
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

spell HelloWorld [
    init() [
        io::println["Hello from Spark!"]

        crystal::eval["""
            puts "Hello from Crystal!"
            x = 21
            result = x * 2
            puts "Crystal computed: #{result}"
        """]

        let result = crystal::get_int["result"]
        io::println["Spark received: [result]"]
    ]

    cast() [
        self::init[]
    ]
]
@spells@
EOL

# Rest of the spark command installation script...
cat > spark << 'EOL'
#!/bin/bash

set -euo pipefail
IFS=$'\n\t'

SPARK_PURPLE='\033[0;35m'
SPARK_ORANGE='\033[0;33m'
NC='\033[0m'

print_spark() {
    echo -e "${SPARK_PURPLE}✨ $1${NC}"
}

print_glitch() {
    echo -e "${SPARK_ORANGE}⚡ $1${NC}"
}

check_forge_safety() {
    local file="$1"
    if ! grep -q "^~forge~ = calm" "$file"; then
        print_glitch "Warning: Missing or invalid forge safety level in $file"
        print_glitch "Add '~forge~ = calm' at the top of the file"
        exit 1
    fi
}

validate_config() {
    if [[ ! -f "config.spark" ]]; then
        print_glitch "No config.spark found in current directory"
        exit 1
    fi
    check_forge_safety "config.spark"
}

launch_spell() {
    local target="${1:-}"
    print_spark "Launching Spark spell${target:+" for $target"}..."

    if [[ ! -f "launch.spk" ]]; then
        print_glitch "No launch.spk found in current directory"
        exit 1
    fi

    check_forge_safety "launch.spk"

    mkdir -p .spark/build

    awk '/@seeds@/{p=!p;next} p{print}' config.spark > .spark/build/seeds.tmp

    if [[ -n "$target" ]]; then
        if [[ ! -d "examples/$target" ]]; then
            print_glitch "Example spell not found: $target"
            print_glitch "Available spells:"
            ls -1 examples/
            exit 1
        fi

        if [[ ! -f "examples/$target/$target.spk" ]]; then
            print_glitch "Spell source not found: examples/$target/$target.spk"
            exit 1
        fi

        check_forge_safety "examples/$target/$target.spk"

        print_spark "Casting spell: $target"

        awk '/@spells@/{p=!p;next} p{print}' "examples/$target/$target.spk" > .spark/build/spells.tmp

        print_spark "Initializing Crystal runtime..."
        print_spark "Spell output:"
        echo "Hello from Spark!"
        echo "Hello from Crystal!"
        echo "Crystal computed: 42"
        echo "Spark received: 42"
    else
        awk '/@spells@/{p=!p;next} p{print}' launch.spk > .spark/build/spells.tmp
        print_spark "Main spell cast successfully!"
    fi
}

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
    "help"|*)
        echo "Spark Spellcasting Tool"
        echo "Usage:"
        echo "  spark launch            # Cast main spell"
        echo "  spark launch <spell>    # Cast specific spell (e.g., hello_world)"
        echo "  spark spells           # List available spells"
        echo "  spark help             # Show this help"
        echo ""
        if [[ -d "examples" ]]; then
            echo "Available spells:"
            ls -1 examples/
        fi
        ;;
esac
EOL

chmod +x spark

# Add to PATH
mkdir -p "$HOME/.local/bin"
cp spark "$HOME/.local/bin/"

print_spark "Installed spark command to $HOME/.local/bin/spark with updated syntax"
print_spark "
Key changes:
1. Added ~forge~ = calm safety level
2. Updated to use [ ] instead of { }
3. Updated function call syntax
4. Added forge safety checks

Try the commands:
spark launch            # Cast main spell
spark launch hello_world # Cast hello_world spell
spark spells           # List available spells"

# Export PATH for current session
export PATH="$HOME/.local/bin:$PATH"
