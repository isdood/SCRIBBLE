#!/bin/bash
# ✨ update_syntax.sh - Magical Syntax Management API ✨
# Author: isdood
# Created: 2025-01-26 12:19:15 UTC
# Description: Central API for managing Spark syntax rules and documentation

# Set magical colors and symbols
STAR="✨"
SCROLL="📜"
SPARKLE="\033[38;5;135m"
MOONLIGHT="\033[38;5;147m"
STARDUST="\033[0m"
ERROR="\033[38;5;196m"
SUCCESS="\033[38;5;82m"
WARNING="\033[38;5;226m"

# Configuration
SYNTAX_DIR="docs/syntax"
RULES_FILE="$SYNTAX_DIR/rules.json"
VERSION="1.0.0"

# Ensure we're in the project root
if [[ ! -d ".git" ]]; then
    echo -e "${ERROR}${STAR} Please run this script from the project root ${STAR}${STARDUST}"
    exit 1
fi

# Create syntax directory if it doesn't exist
mkdir -p "$SYNTAX_DIR"

# Initialize rules if they don't exist
if [[ ! -f "$RULES_FILE" ]]; then
    cat << EOF > "$RULES_FILE"
{
    "version": "$VERSION",
    "last_updated": "$(date -u +"%Y-%m-%d %H:%M:%S")",
    "updated_by": "isdood",
    "feature_tags": {
        "forge": {
            "required": true,
            "values": ["calm", "balanced", "wild"],
            "default": "calm"
        },
        "weave": {
            "required": true,
            "range": [0, 1000],
            "default": 500
        }
    },
    "blocks": {
        "seeds": {
            "required": true,
            "description": "Import declarations"
        },
        "spells": {
            "required": true,
            "description": "Function definitions"
        }
    },
    "syntax_rules": {
        "method_chain": "**",
        "brackets": "[]",
        "comments": ">>>",
        "visibility_prefix": "p"
    }
}
EOF
fi

# Function to display help
show_help() {
    cat << EOF
✨ Spark Syntax Management API ✨

Usage: ./update_syntax.sh [command] [options]

Commands:
    help                    Show this help message
    version                 Show version information
    validate               Check syntax in all .spk files
    fix                    Fix common syntax issues
    update [component]     Update syntax rules
    check [file]          Check specific file
    rules                 Show current syntax rules
    export               Export rules to documentation
    stats                Show syntax statistics
    watch                Watch for syntax changes

Options:
    --quiet, -q           Suppress output
    --force, -f           Force operation
    --verbose, -v         Show detailed output
    --json               Output in JSON format

Examples:
    ./update_syntax.sh validate
    ./update_syntax.sh fix
    ./update_syntax.sh update feature_tags
    ./update_syntax.sh check path/to/file.spk
EOF
}

# Function to validate syntax
validate_syntax() {
    local quiet=$1

    [[ "$quiet" != "true" ]] && echo "🔍 Validating Spark syntax..."

    # Call existing validate_syntax.sh
    bash "$SYNTAX_DIR/validate_syntax.sh"
}

# Function to fix syntax
fix_syntax() {
    local force=$1

    echo "🔧 Fixing syntax issues..."

    # Call existing fix_template.sh
    bash "$SYNTAX_DIR/fix_template.sh"
}

# Function to update rules
update_rules() {
    local component=$1
    local rules

    rules=$(cat "$RULES_FILE")

    case "$component" in
        "feature_tags")
            echo "Updating feature tags..."
            # Logic to update feature tags
            ;;
        "blocks")
            echo "Updating block definitions..."
            # Logic to update blocks
            ;;
        "syntax")
            echo "Updating syntax rules..."
            # Logic to update syntax
            ;;
        *)
            echo "Invalid component. Use: feature_tags, blocks, or syntax"
            exit 1
            ;;
    esac

    # Update timestamp
    rules=$(echo "$rules" | jq ".last_updated = \"$(date -u +"%Y-%m-%d %H:%M:%S")\"")
    echo "$rules" > "$RULES_FILE"
}

# Function to show statistics
show_stats() {
    echo "📊 Spark Syntax Statistics"
    echo "-------------------------"

    # Count .spk files
    local spk_files=$(find . -name "*.spk" | wc -l)
    echo "Total .spk files: $spk_files"

    # Count syntax issues
    local issues=$(./docs/syntax/validate_syntax.sh 2>&1 | grep -c "❌")
    echo "Syntax issues found: $issues"

    # Show rules version
    local version=$(jq -r .version "$RULES_FILE")
    echo "Rules version: $version"

    # Last update
    local last_updated=$(jq -r .last_updated "$RULES_FILE")
    echo "Last updated: $last_updated"
}

# Function to watch for changes
watch_syntax() {
    echo "👀 Watching for syntax changes..."

    while true; do
        inotifywait -e modify,create,delete -r . --format '%w%f' | while read file; do
            if [[ "$file" =~ \.spk$ ]]; then
                echo "🔄 Change detected in $file"
                ./docs/syntax/validate_syntax.sh "$file"
            fi
        done
    done
}

# Function to export rules
export_rules() {
    local format=$1

    case "$format" in
        "json")
            cat "$RULES_FILE"
            ;;
        "markdown")
            # Convert rules to markdown
            echo "# Spark Syntax Rules"
            echo
            jq -r . "$RULES_FILE" | sed 's/^/    /'
            ;;
        *)
            echo "Invalid format. Use: json or markdown"
            exit 1
            ;;
    esac
}

# Main command processing
case "$1" in
    "help")
        show_help
        ;;
    "version")
        echo "Spark Syntax Manager v$VERSION"
        ;;
    "validate")
        validate_syntax "${2:-false}"
        ;;
    "fix")
        fix_syntax "${2:-false}"
        ;;
    "update")
        update_rules "$2"
        ;;
    "rules")
        cat "$RULES_FILE" | jq .
        ;;
    "export")
        export_rules "${2:-json}"
        ;;
    "stats")
        show_stats
        ;;
    "watch")
        watch_syntax
        ;;
    *)
        echo "Unknown command. Use './update_syntax.sh help' for usage information."
        exit 1
        ;;
esac

exit 0
