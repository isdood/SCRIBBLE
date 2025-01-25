#!/bin/bash

set -e  # Exit on any error

echo "🌟 Starting Sparkle setup..."

echo "📦 Creating Zig string processor..."

# Create Zig string processor
cat > "seed_wrapper.zig" << 'ZIGPROC'
const std = @import("std");

pub fn main() !void {
    var arena = std.heap.ArenaAllocator.init(std.heap.page_allocator);
    defer arena.deinit();
    const allocator = arena.allocator();

    const args = try std.process.argsAlloc(allocator);
    defer std.process.argsFree(allocator, args);

    if (args.len < 2) {
        std.debug.print("Usage: {s} <command>\n", .{args[0]});
        std.process.exit(1);
    }

    // Create new args array
    var new_args = try allocator.alloc([]const u8, args.len);
    defer allocator.free(new_args);

    // First arg is always the original script
    new_args[0] = "./bin/seed.orig";

    // Copy remaining args
    var arg_idx: usize = 1;
    while (arg_idx < args.len) : (arg_idx += 1) {
        new_args[arg_idx] = args[arg_idx];
    }

    var proc = std.process.Child.init(new_args, allocator);
    proc.stdout_behavior = .Pipe;
    proc.stderr_behavior = .Inherit;
    proc.stdin_behavior = .Inherit;

    try proc.spawn();

    const stdout = proc.stdout.?;
    const writer = std.io.getStdOut().writer();

    var buffer: [4096]u8 = undefined;
    var out_buffer: [4096]u8 = undefined;
    var out_index: usize = 0;

    while (true) {
        const bytes_read = try stdout.read(&buffer);
        if (bytes_read == 0) break;

        var byte_idx: usize = 0;
        while (byte_idx < bytes_read) {
            if (byte_idx + 1 < bytes_read and buffer[byte_idx] == '_' and buffer[byte_idx + 1] == '_') {
                out_buffer[out_index] = '*';
                out_buffer[out_index + 1] = '*';
                out_index += 2;
                byte_idx += 2;
            } else {
                out_buffer[out_index] = buffer[byte_idx];
                out_index += 1;
                byte_idx += 1;
            }
        }

        if (out_index > 0) {
            try writer.writeAll(out_buffer[0..out_index]);
            out_index = 0;
        }
    }

    const term = try proc.wait();
    switch (term) {
        .Exited => |code| std.process.exit(code),
        else => std.process.exit(1),
    }
}
ZIGPROC

echo "🔨 Building Zig processor..."

# Build Zig processor
zig build-exe seed_wrapper.zig -O ReleaseSmall

echo "📝 Creating seed script..."

# Create bin directory if it doesn't exist
mkdir -p bin

# Create the main seed script
cat > "bin/seed.orig" << 'SEEDMANAGER'
#!/bin/bash

# Disable globbing
set -f

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
    local package_path=$(echo "$package" | sed 's/\*\*/\//g')

    if [ ! -f "config.sparkle" ]; then
        cat > "config.sparkle" << CONF
# Sparkle Garden Configuration
garden:
  version: "1.0.0"
  created: "2025-01-25 02:30:43"
  tender: "isdood"

modules:
  - name: "std**math"
    version: "1.0.0"
    description: "Mathematical operations with moonlit precision"
    path: "std/math"

packages:
CONF
    fi

    mkdir -p "$package_path"

    cat > "$package_path/config.spark" << SPARKCONFIG
# Package: $package
pattern:
  weave: 500  # Thread weaving intensity
  bio: false  # Bio-computational mode

garden:
  planted: "2025-01-25 02:30:43"
  tender: "isdood"
  sparkles: ["add", "sub", "mul", "div"]

whispers:
  - "Numbers dance in moonlit arrays"
  - "Calculations flow like starlight"
SPARKCONFIG

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
        sed -i "/name: \"$package\"/,+3d" "config.sparkle"
        rm -rf "$package_path"
        echo -e "${GREEN}✨ Package $package has been returned to stardust${NC}"
    else
        echo -e "${BLUE}No garden found. Create one with 'seed plant'${NC}"
    fi
}

case "$1" in
    "plant")
        if [ -z "$2" ]; then
            echo -e "${BLUE}Usage: ./bin/seed plant std**math${NC}"
            exit 1
        fi
        plant_package "$2"
        ;;
    "unplant")
        if [ -z "$2" ]; then
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
        echo "  ./bin/seed plant std**math     - Plant a new package in your garden"
        echo "  ./bin/seed unplant std**math   - Gently remove a package"
        echo "  ./bin/seed status              - Show your blooming packages"
        ;;
esac

# Re-enable globbing
set +f
SEEDMANAGER

echo "🔑 Setting permissions..."

# Make scripts executable
chmod +x bin/seed.orig

echo "📦 Installing Zig wrapper..."

# Move Zig binary to bin/seed
mv seed_wrapper bin/seed
chmod +x bin/seed

echo "🧹 Cleaning up..."

# Clean up build files
rm -f seed_wrapper.zig seed_wrapper.o

echo "✨ Setup complete! Try running: ./bin/seed plant std**math"
