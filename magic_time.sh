cat > "config.sparkle" << 'SPARKLECONFIG'
# Sparkle Garden Configuration
garden:
  version: "1.0.0"
  created: "2025-01-25 01:52:58"
  tender: "isdood"

modules:
  - name: "std**math"
    version: "1.0.0"
    description: "Mathematical operations with moonlit precision"
    path: "std/math"

packages:
  - name: "std**math"
    version: "1.0.0"
    config: "std/math/config.spark"
    whispers:
      - "The mathematical moonlight dances"
      - "Numbers weave through starlit paths"
SPARKLECONFIG

cat > "bin/seed" << 'SEEDMANAGER'
#!/bin/bash

PURPLE='\033[0;35m'
GREEN='\033[0;32m'
BLUE='\033[0;34m'
NC='\033[0m'

function show_garden_status() {
    echo -e "${PURPLE}🌱 Garden Status${NC}"
    echo -e "└─ ${GREEN}Blooming Packages${NC}"

    if [ -f "config.sparkle" ]; then
        local in_modules=false
        while IFS= read -r line; do
            if [[ $line == *"modules:"* ]]; then
                in_modules=true
                continue
            elif [[ $line == *"packages:"* ]]; then
                in_modules=false
                continue
            fi

            if [ "$in_modules" = true ] && [[ $line =~ ^\ \ -\ name:\ \"([^\"]+)\" ]]; then
                local module_name="${BASH_REMATCH[1]}"
                local module_path=$(echo "$module_name" | sed 's/\*\*/\//g')
                local module_status="✨ Active"

                # Check if module is actually installed
                if [ -d "$module_path" ]; then
                    echo "   └─ 🌺 ${module_name} (${module_status})"
                fi
            fi
        done < "config.sparkle"
    else
        echo "   └─ No packages planted yet"
    fi
}

function plant_package() {
    local package=$1
    local package_path=$(echo "$package" | sed 's/\*\*/\//g')
    echo -e "${PURPLE}🌱 Planting $package...${NC}"

    # Create config.sparkle if it doesn't exist
    if [ ! -f "config.sparkle" ]; then
        cat > "config.sparkle" << CONF
# Sparkle Garden Configuration
garden:
  version: "1.0.0"
  created: "2025-01-25 01:52:58"
  tender: "isdood"

modules:
  - name: "std**math"
    version: "1.0.0"
    description: "Mathematical operations with moonlit precision"
    path: "std/math"

packages:
CONF
    fi

    # Create package directory structure if it doesn't exist
    mkdir -p "$package_path"

    # Create or update package's config.spark
    cat > "$package_path/config.spark" << SPARKCONFIG
# Package: $package
pattern:
  weave: 500  # Thread weaving intensity
  bio: false  # Bio-computational mode

garden:
  planted: "2025-01-25 01:52:58"
  tender: "isdood"
  sparkles: ["add", "sub", "mul", "div"]

whispers:
  - "Numbers dance in moonlit arrays"
  - "Calculations flow like starlight"
SPARKCONFIG

    # Add to modules if not already present
    if ! grep -q "name: \"$package\"" "config.sparkle"; then
        sed -i "/modules:/a \ \ - name: \"$package\"\n    version: \"1.0.0\"\n    description: \"Module $package\"\n    path: \"$package_path\"" "config.sparkle"
    fi

    echo -e "${GREEN}✨ Package $package has taken root!${NC}"
}

function unplant_package() {
    local package=$1
    local package_path=$(echo "$package" | sed 's/\*\*/\//g')
    echo -e "${PURPLE}🍂 Gently removing $package...${NC}"

    if [ -f "config.sparkle" ]; then
        # Remove from modules section
        sed -i "/name: \"$package\"/,+3d" "config.sparkle"

        # Remove package directory
        rm -rf "$package_path"

        echo -e "${GREEN}✨ Package $package has been returned to stardust${NC}"
    else
        echo -e "${BLUE}No garden found. Create one with 'seed plant'${NC}"
    fi
}

case "$1" in
    "plant")
        if [ -z "$2" ]; then
            echo -e "${BLUE}Usage: seed plant <package-name>${NC}"
            exit 1
        fi
        plant_package "$2"
        ;;
    "unplant")
        if [ -z "$2" ]; then
            echo -e "${BLUE}Usage: seed unplant <package-name>${NC}"
            exit 1
        fi
        unplant_package "$2"
        ;;
    "status")
        show_garden_status
        ;;
    *)
        echo -e "${PURPLE}🌱 Seed Package Manager${NC}"
        echo "Usage:"
        echo "  seed plant <package-name>   - Plant a new package in your garden"
        echo "  seed unplant <package-name> - Gently remove a package"
        echo "  seed status                 - Show your blooming packages"
        ;;
esac
SEEDMANAGER

chmod +x bin/seed

# Update any existing package to use new naming convention
mv std/math std__math 2>/dev/null || true
