#!/usr/bin/env bash
# magic_regex_fix.sh - Fix regex patterns for ** handling
# Author: isdood
# Created: 2025-01-25 14:38:12

set -euo pipefail

SPARK_PURPLE='\033[0;35m'
SPARK_RESET='\033[0m'

echo_magic() {
    echo -e "${SPARK_PURPLE}✨ $1${SPARK_RESET}"
}

SPARK_ROOT="$(pwd)"
SPARK_BIN="$SPARK_ROOT/bin"

fix_seed_fish() {
    cat > "$SPARK_BIN/seed.fish" << EOF
#!/usr/bin/env fish

# Core Spark configuration
set -g SPARK_ROOT "$SPARK_ROOT"
set -g SPARK_PATH_SEP '**'

# Display configuration
set -g spark_purple '\033[0;35m'
set -g spark_reset '\033[0m'

function __spark_echo
    echo -e "\$spark_purple✨ \$argv\$spark_reset"
end

# Core path handling system
function __normalize_path
    # Convert any path to use Spark separators for display
    echo \$argv[1] | string replace -a '/' \$SPARK_PATH_SEP
end

function __to_fs_path
    # Convert any path to filesystem format
    echo \$argv[1] | string replace -a \$SPARK_PATH_SEP '/'
end

function __is_std_package
    # Use exact pattern matching instead of regex
    string match -q "std\$SPARK_PATH_SEP*" (__normalize_path \$argv[1])
end

function __get_std_name
    # Use string replace with escaped pattern
    string replace "std\$SPARK_PATH_SEP" "" (__normalize_path \$argv[1])
end

function __display_path
    __normalize_path \$argv[1]
end

function seed
    if test (count \$argv) -lt 1
        echo "Usage: seed [plant|unplant|garden] [package\$SPARK_PATH_SEP"name"]"
        return 1
    end

    set -l action \$argv[1]
    set -l garden_path "\$HOME/.spark/garden"
    
    switch \$action
        case "garden"
            __spark_echo "Your Magical Garden ✨"
            
            if test -d "\$garden_path"
                for pkg in (find "\$garden_path" -mindepth 1 -maxdepth 3 -type d ! -name ".*" ! -type l 2>/dev/null | sort)
                    set -l rel_path (string replace -r "^\$garden_path/" "" "\$pkg")
                    test -n "\$rel_path"; and __spark_echo (__display_path "\$rel_path")
                end
                
                for pkg in (find "\$garden_path" -mindepth 1 -maxdepth 3 -type l 2>/dev/null | sort)
                    set -l rel_path (string replace -r "^\$garden_path/" "" "\$pkg")
                    test -n "\$rel_path"; and __spark_echo (__display_path "\$rel_path") "(linked)"
                end
            else
                __spark_echo "Your garden is empty! Plant some packages with: seed plant package\$SPARK_PATH_SEP"name""
            end

        case "plant"
            if test (count \$argv) -lt 2
                echo "Usage: seed plant package\$SPARK_PATH_SEP"name""
                return 1
            end
            
            set -l spark_path (__normalize_path \$argv[2])
            set -l fs_path (__to_fs_path \$spark_path)
            set -l target_path "\$garden_path/\$fs_path"
            
            __spark_echo "Planting \$spark_path..."
            
            if __is_std_package \$spark_path
                set -l std_name (__get_std_name \$spark_path)
                set -l std_path "\$SPARK_ROOT/std/\$std_name"
                
                if test -d "\$std_path"
                    mkdir -p (dirname "\$target_path")
                    if ln -sf "\$std_path" "\$target_path"
                        __spark_echo "Standard package \$spark_path planted successfully!"
                    else
                        __spark_echo "Failed to plant standard package \$spark_path"
                        return 1
                    end
                else
                    __spark_echo "Standard package \$spark_path not found in std library!"
                    echo "Available std modules:"
                    for module in (find "\$SPARK_ROOT/std" -mindepth 1 -maxdepth 1 -type d ! -name ".*" 2>/dev/null)
                        echo "  - std\$SPARK_PATH_SEP"(basename \$module)
                    end
                    return 1
                end
            else
                if mkdir -p "\$target_path"
                    __spark_echo "External package \$spark_path planted successfully!"
                else
                    __spark_echo "Failed to plant external package \$spark_path"
                    return 1
                end
            end

        case "unplant"
            if test (count \$argv) -lt 2
                echo "Usage: seed unplant package\$SPARK_PATH_SEP"name""
                return 1
            end
            
            set -l spark_path (__normalize_path \$argv[2])
            set -l fs_path (__to_fs_path \$spark_path)
            set -l target_path "\$garden_path/\$fs_path"
            
            if test -e "\$target_path"; or test -L "\$target_path"
                __spark_echo "Unplanting \$spark_path..."
                if rm -rf "\$target_path"
                    __spark_echo "Package \$spark_path unplanted successfully!"
                else
                    __spark_echo "Failed to unplant \$spark_path"
                    return 1
                end
            else
                __spark_echo "Package \$spark_path not found in garden!"
                return 1
            end

        case '*'
            __spark_echo "Unknown magical command: \$action"
            echo "Available commands: plant, unplant, garden"
            return 1
    end
end

complete -c seed -f -n "__fish_use_subcommand" -a "plant" -d "Plant a new package"
complete -c seed -f -n "__fish_use_subcommand" -a "unplant" -d "Remove a package"
complete -c seed -f -n "__fish_use_subcommand" -a "garden" -d "Display installed packages"
EOF

    chmod +x "$SPARK_BIN/seed.fish"
}

setup_std_lib() {
    echo_magic "Setting up standard library..."
    mkdir -p "$SPARK_ROOT/std/"{math,string,io,crystometer}
    
    cat > "$SPARK_ROOT/std/math/mod.spk" << 'EOF'
~weave~ = 500

@spells@
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

pub fn multiply(a: i32, b: i32) -> i32 {
    a * b
}
@spells@
EOF
    echo_magic "Standard library setup complete!"
}

cleanup_garden() {
    echo_magic "Cleaning up garden..."
    rm -rf "$HOME/.spark/garden"
    mkdir -p "$HOME/.spark/garden"
}

# Apply all fixes
cleanup_garden
setup_std_lib
fix_seed_fish

echo_magic "Regex patterns fixed! ✨"
echo_magic "Try: seed plant std**math"
