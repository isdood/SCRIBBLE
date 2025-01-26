#!/bin/bash
# ✨ forge_syntax.sh - Magical Documentation Generator for Spark Syntax ✨
# Author: isdood
# Created: 2025-01-26 12:15:50 UTC
# Description: Generates comprehensive documentation for Spark syntax

# Set magical colors for enchanted output
STAR="✨"
SCROLL="📜"
SPARKLE="\033[38;5;135m"
MOONLIGHT="\033[38;5;147m"
STARDUST="\033[0m"
ERROR="\033[38;5;196m"
SUCCESS="\033[38;5;82m"

# Make sure we're in the project root
if [[ ! -d ".git" ]]; then
    echo -e "${ERROR}${STAR} Please run this script from the project root ${STAR}${STARDUST}"
    exit 1
fi

# Create our magical directories
echo -e "${SPARKLE}${STAR} Creating magical documentation realm...${STARDUST}"
mkdir -p docs/syntax/{core,examples,patterns}

# Generate enhanced syntax checker script
cat << 'EOF' > "docs/syntax/validate_syntax.sh"
#!/bin/bash

check_syntax() {
    local file=$1
    local errors=0
    local warnings=0

    echo "🔍 Checking $file..."

    # Required feature tags
    if ! grep -q "~forge~" "$file"; then
        echo "❌ Missing ~forge~ tag"
        ((errors++))
    fi

    if ! grep -q "~weave~" "$file"; then
        echo "❌ Missing ~weave~ tag"
        ((errors++))
    fi

    # Check for magical blocks
    if ! grep -q "@seeds@" "$file"; then
        echo "❌ Missing @seeds@ block"
        ((errors++))
    fi

    if ! grep -q "@spells@" "$file"; then
        echo "❌ Missing @spells@ block"
        ((errors++))
    fi

    # Check for proper method invocation
    if grep -q "::" "$file"; then
        echo "❌ Found :: instead of **"
        ((errors++))
    fi

    # Check for proper comments
    if grep -q "^//" "$file"; then
        echo "⚠️ Found // instead of >>> (warning)"
        ((warnings++))
    fi

    # Check for proper bracket usage
    if grep -q "{" "$file" || grep -q "}" "$file"; then
        echo "❌ Found curly braces instead of square brackets"
        ((errors++))
    fi

    if [ $errors -eq 0 ]; then
        if [ $warnings -eq 0 ]; then
            echo -e "\033[38;5;82m✨ Syntax is magical!\033[0m"
        else
            echo -e "\033[38;5;226m✨ Syntax is mostly magical! ($warnings warnings)\033[0m"
        fi
        return 0
    else
        echo -e "\033[38;5;196m🌙 Found $errors syntax mishaps and $warnings warnings\033[0m"
        return 1
    fi
}

# Header
echo "✨ Spark Syntax Validator ✨"
echo "Running validation on $(date +"%Y-%m-%d %H:%M:%S UTC")"
echo "----------------------------------------"

# Find all .spk files and check them
find . -name "*.spk" -type f | while read -r file; do
    check_syntax "$file"
    echo "----------------------------------------"
done

# Summary
echo "Validation complete!"
echo "To fix template issues, run: ./docs/syntax/fix_template.sh"
EOF

# Generate documentation with correct syntax
cat << 'EOF' > "docs/syntax/core/MAGICAL_SYNTAX.spk"
~forge~ = calm  # Safety level
~weave~ = 500  # Performance tuning

@seeds@
use sparkle.core.[Documentation, Enchantment]
use sparkle.format.[Markdown, Spellbook]
@seeds@

@spells@
>>> Core Syntax Documentation

>>> 1. Required Feature Tags
fn demonstrate_features[] -> Example [
    ~forge~ = calm    # Safety levels: calm, balanced, wild
    ~weave~ = 500    # Performance tuning (0-1000)
]

>>> 2. Function Visibility
fn private_spell[] -> Example [
    >>> This is a private spell
]

pfn public_spell[] -> Example [
    >>> This is a public spell (p replaces pub)
]

>>> 3. Magical Blocks
fn demonstrate_blocks[] -> Example [
    @seeds@
    use realm.module.[Component]
    @seeds@

    @spells@
    fn inner_spell[] -> Result [
        >>> Implementation
    ]
    @spells@
]

>>> 4. Method Invocation
fn show_method_calls[] -> Example [
    object
        **first_enchantment()
        **second_enchantment()
        **final_seal()
]
@spells@
EOF

# Template fixer script for ~weave~ tag
cat << 'EOF' > "docs/syntax/fix_template.sh"
#!/bin/bash

fix_template() {
    local file=$1
    local temp_file="${file}.tmp"

    # Check if file already has ~weave~
    if ! grep -q "~weave~" "$file"; then
        # Add ~weave~ after ~forge~ if it exists
        if grep -q "~forge~" "$file"; then
            sed '/~forge~/a ~weave~ = 500  # Performance tuning' "$file" > "$temp_file"
            mv "$temp_file" "$file"
            echo "✨ Added ~weave~ to $file"
        else
            # Add both ~forge~ and ~weave~ at the start
            echo -e "~forge~ = calm  # Safety level\n~weave~ = 500  # Performance tuning\n$(cat "$file")" > "$temp_file"
            mv "$temp_file" "$file"
            echo "✨ Added ~forge~ and ~weave~ to $file"
        fi
    fi
}

# Find and fix all .spk files
find . -name "*.spk" -type f | while read -r file; do
    fix_template "$file"
done
EOF

# Make scripts executable
chmod +x docs/syntax/validate_syntax.sh
chmod +x docs/syntax/fix_template.sh

echo -e "${SPARKLE}${STAR} Documentation enchantment complete! ${STAR}${STARDUST}"
echo -e "${MOONLIGHT}Your magical documentation realm awaits in ./docs/syntax/${STARDUST}"
echo -e "${MOONLIGHT}Run ./docs/syntax/validate_syntax.sh to verify your spells${STARDUST}"
echo -e "${MOONLIGHT}Run ./docs/syntax/fix_template.sh to fix template issues${STARDUST}"
