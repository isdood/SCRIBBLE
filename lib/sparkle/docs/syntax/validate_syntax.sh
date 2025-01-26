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
