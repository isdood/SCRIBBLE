#!/usr/bin/env bash

# Forge Look - Directory Listing Component
# Author: isdood
# Created: 2025-01-26 13:15:20 UTC
# Repository: isdood/scribble

# Check if script is run as root
if [ "$EUID" -ne 0 ]; then
    echo "🔒 This script requires root permissions to set up the Look component"
    echo "Running with sudo..."
    sudo "$0" "$@"
    exit $?
fi

set -e

echo "⚒️ Forge Look - Directory Listing Component"
echo "Created: 2025-01-26 13:15:20 UTC"
echo "Author: isdood"

# Get the real user who ran sudo
REAL_USER="${SUDO_USER:-$USER}"
SCRIPT_DIR="$(dirname "$(readlink -f "$0")")"
SPARKLE_DIR="$SCRIPT_DIR/.sparkle"
FORGE_DIR="/home/guavabot1/scribble/scribble/lib/spark/forge"

echo "🔑 Running as root, will set permissions for user: $REAL_USER"

# Create directories
echo "📚 Creating directories..."
mkdir -p "$FORGE_DIR/std/look"
mkdir -p "$SPARKLE_DIR/std/look"

# Update SeedManager.jl to include look component
echo "🔧 Updating SeedManager.jl..."
sed -i '/const STD_PACKAGES = Dict(/,/)/c\
const STD_PACKAGES = Dict(\
    "std" => [\
        "math",      # Basic mathematical operations\
        "cubed",     # Cube operations\
        "whisper",   # Text case manipulation\
        "clone",     # Object cloning\
        "inq",       # Inquiry functions\
        "align",     # Text alignment\
        "shout",     # Text case manipulation\
        "any",       # Type-agnostic operations\
        "array",     # Array utilities\
        "ascii",     # ASCII text operations\
        "coll",      # Collection utilities\
        "comp",      # Comparison operations\
        "conv",      # Type conversion utilities\
        "def",       # Default value handling\
        "echo",      # Echo utilities\
        "glitch",    # Glitch art effects\
        "history",   # Command history\
        "look",      # Directory listing\
        "itex",      # Interactive text manipulation\
        "lend",      # Resource lending utilities\
        "mark",      # Markup processing\
        "murmur",    # Murmur hash implementation\
        "potion",    # Data transformation utilities\
        "rune",      # Symbol and pattern matching\
        "scribe",    # Logging and documentation\
        "shard",     # Data partitioning\
        "shimmer",   # Animation and visual effects\
        "signal",    # Event handling and signals\
        "spell",     # Command execution patterns\
        "thunder"    # Parallel processing utilities\
    ]\
)' "$SPARKLE_DIR/src/SeedManager.jl"

# Create std/look implementation
echo "📝 Creating std/look/init.jl..."
cat > "$SPARKLE_DIR/std/look/init.jl" << 'EOT'
module Look

export list_files, list_directory, format_size
export FileInfo, get_file_info, sort_files
export LookOptions, look

using Dates
using Printf

struct FileInfo
    name::String
    size::Int64
    modified::DateTime
    permissions::String
    type::Symbol  # :file, :directory, :symlink
    owner::String
    group::String
end

struct LookOptions
    all::Bool       # Show hidden files
    long::Bool      # Long listing format
    human::Bool     # Human-readable sizes
    sort::Symbol    # :name, :size, :modified
    reverse::Bool   # Reverse sort order
end

const SIZE_UNITS = ["B", "KB", "MB", "GB", "TB"]
const COLORS = Dict(
    :directory => "\033[1;34m",  # Blue
    :symlink => "\033[1;36m",    # Cyan
    :executable => "\033[1;32m",  # Green
    :file => "\033[0m",          # Default
    :reset => "\033[0m"          # Reset
)

"""
Format file size in human-readable format
"""
function format_size(size::Integer; human::Bool=true)
    if !human
        return @sprintf("%d", size)
    end

    unit_index = 1
    size_f = float(size)

    while size_f ≥ 1024 && unit_index < length(SIZE_UNITS)
        size_f /= 1024
        unit_index += 1
    end

    if unit_index == 1
        return @sprintf("%d %s", size, SIZE_UNITS[unit_index])
    else
        return @sprintf("%.1f %s", size_f, SIZE_UNITS[unit_index])
    end
end

"""
Get file information
"""
function get_file_info(path::String)
    stat_info = stat(path)
    name = basename(path)

    type = islink(path) ? :symlink :
           isdir(path) ? :directory :
           :file

    FileInfo(
        name,
        stat_info.size,
        unix2datetime(stat_info.mtime),
        string(filemode(stat_info.mode)),
        type,
        # In a real implementation, these would come from pwd/grp
        "user",
        "group"
    )
end

"""
Sort files according to options
"""
function sort_files(files::Vector{FileInfo}, sort_by::Symbol)
    if sort_by == :name
        sort!(files, by = f -> f.name)
    elseif sort_by == :size
        sort!(files, by = f -> f.size, rev = true)
    elseif sort_by == :modified
        sort!(files, by = f -> f.modified, rev = true)
    end
end

"""
List files in directory with formatting
"""
function list_files(dir::String=".", options::LookOptions=LookOptions(false, false, true, :name, false))
    entries = readdir(dir, join=true)
    files = FileInfo[]

    for entry in entries
        name = basename(entry)
        if !options.all && startswith(name, ".")
            continue
        end
        push!(files, get_file_info(entry))
    end

    sort_files(files, options.sort)
    if options.reverse
        reverse!(files)
    end

    if options.long
        _print_long_format(files, options)
    else
        _print_short_format(files)
    end
end

"""
Print files in long format
"""
function _print_long_format(files::Vector{FileInfo}, options::LookOptions)
    for file in files
        color = get(COLORS, file.type, COLORS[:file])
        size_str = format_size(file.size, human=options.human)
        date_str = Dates.format(file.modified, "u-d HH:MM")
        println("$(file.permissions) $(lpad(size_str, 8)) $date_str $(color)$(file.name)$(COLORS[:reset])")
    end
end

"""
Print files in short format
"""
function _print_short_format(files::Vector{FileInfo})
    max_name_length = maximum(length.(getfield.(files, :name))) + 2
    terminal_width = try
        parse(Int, get(ENV, "COLUMNS", "80"))
    catch
        80
    end

    cols = max(1, div(terminal_width, max_name_length))
    rows = ceil(Int, length(files) / cols)

    for row in 1:rows
        for col in 1:cols
            idx = (col-1) * rows + row
            if idx <= length(files)
                file = files[idx]
                color = get(COLORS, file.type, COLORS[:file])
                print(color, rpad(file.name, max_name_length), COLORS[:reset])
            end
        end
        println()
    end
end

"""
Main look function - lists directory contents
"""
function look(dir::String="."; all::Bool=false, long::Bool=false, human::Bool=true,
             sort_by::Symbol=:name, reverse::Bool=false)
    options = LookOptions(all, long, human, sort_by, reverse)
    list_files(dir, options)
end

end
EOT

# Create Sparkle command integration
echo "📝 Creating look command integration..."
cat > "$SPARKLE_DIR/commands/look.jl" << 'EOT'
# Look command for Sparkle
# Lists files in current directory

function cmd_look(args...)
    using .Look

    all = "--all" in args || "-a" in args
    long = "--long" in args || "-l" in args
    human = !("--bytes" in args || "-b" in args)
    reverse = "--reverse" in args || "-r" in args

    sort_by = :name
    for arg in args
        if startswith(arg, "--sort=")
            sort_type = Symbol(split(arg, "=")[2])
            if sort_type in [:name, :size, :modified]
                sort_by = sort_type
            end
        end
    end

    dir = "."
    for arg in args
        if !startswith(arg, "-")
            dir = arg
            break
        end
    end

    look(dir, all=all, long=long, human=human, sort_by=sort_by, reverse=reverse)
end

cmd_help_look() = """
look [options] [directory]
List directory contents in a pretty format

Options:
  -a, --all      Show hidden files
  -l, --long     Use long listing format
  -b, --bytes    Show sizes in bytes
  -r, --reverse  Reverse sort order
  --sort=TYPE    Sort by: name, size, modified
"""
EOT

# Set permissions
echo "🔒 Setting permissions..."
chown -R "$REAL_USER:$REAL_USER" "$FORGE_DIR/std/look"
chown -R "$REAL_USER:$REAL_USER" "$SPARKLE_DIR/std/look"
chown "$REAL_USER:$REAL_USER" "$SPARKLE_DIR/commands/look.jl"

find "$FORGE_DIR/std/look" -type d -exec chmod 755 {} \;
find "$FORGE_DIR/std/look" -type f -exec chmod 644 {} \;
find "$SPARKLE_DIR/std/look" -type d -exec chmod 755 {} \;
find "$SPARKLE_DIR/std/look" -type f -exec chmod 644 {} \;
chmod 644 "$SPARKLE_DIR/commands/look.jl"

echo "✨ Look component has been forged!"
echo "Available as:"
echo "  - std**look    (Directory listing component)"
echo "  - look         (Sparkle command)"
echo ""
echo "Try 'seed plant std**look' to use in your project"
echo "Or 'look --help' in Sparkle for command usage"

# Verify installation
echo "🔍 Verifying installation..."
if [ -d "$FORGE_DIR/std/look" ] && \
   [ -d "$SPARKLE_DIR/std/look" ] && \
   [ -f "$SPARKLE_DIR/commands/look.jl" ]; then
    echo "✅ Look component installed successfully!"
else
    echo "❌ Some components may be missing. Please check installation."
fi
