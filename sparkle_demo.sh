#!/bin/bash

# Set strict error handling
set -euo pipefail
IFS=$'\n\t'

echo "📝 Creating seed package manager..."

mkdir -p bin

# Create the main seed script
cat > "bin/seed" << 'SEED_SCRIPT'
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
                local module_path="$module_name"
                local module_status="✨ Active"

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
    echo -e "${PURPLE}🌱 Planting $package...${NC}"
    mkdir -p "$package"
    echo -e "${GREEN}✨ Package $package has taken root!${NC}"
}

function unplant_package() {
    local package=$1
    echo -e "${PURPLE}🍂 Gently removing $package...${NC}"
    rm -rf "$package"
    echo -e "${GREEN}✨ Package $package has been returned to stardust${NC}"
}

case "$1" in
    "plant")
        if [ -z "${2:-}" ]; then
            echo -e "${BLUE}Usage: ./bin/seed plant std**math${NC}"
            exit 1
        fi
        plant_package "$2"
        ;;
    "unplant")
        if [ -z "${2:-}" ]; then
            echo -e "${BLUE}Usage: ./bin/seed unplant std**math${NC}"
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
        echo "  ./bin/seed plant std**math     - Plant a new package"
        echo "  ./bin/seed unplant std**math   - Remove a package"
        echo "  ./bin/seed status              - Show package status"
        ;;
esac
SEED_SCRIPT

chmod +x bin/seed

# Create example configuration
echo "📝 Creating example configuration..."
cat > "config.sparkle" << 'CONFIG'
# Sparkle Garden Configuration
garden:
  version: "1.0.0"
  created: "2025-01-25 02:44:36"
  tender: "isdood"

modules:
  - name: "std**math"
    version: "1.0.0"
    description: "Mathematical operations with moonlit precision"
    path: "std**math"
CONFIG

echo -e "\n✨ Demo script created successfully!"
echo -e "Try running:"
echo -e "  ./bin/seed status"
echo -e "  ./bin/seed plant std**math"
echo -e "  ./bin/seed unplant std**math"
