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
