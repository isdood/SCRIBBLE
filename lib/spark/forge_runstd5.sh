#!/usr/bin/env bash

# Sudo Forge RunSTD5 - Sparkle Standard Library Implementation Part 5
# Author: isdood
# Created: 2025-01-26 12:56:26 UTC
# Repository: isdood/scribble

# Check if script is run as root
if [ "$EUID" -ne 0 ]; then
    echo "🔒 This script requires root permissions to set up the Sparkle environment"
    echo "Running with sudo..."
    sudo "$0" "$@"
    exit $?
fi

set -e

echo "⚒️ Forge RunSTD5 - Additional Standard Library Components"
echo "Created: 2025-01-26 12:56:26 UTC"
echo "Author: isdood"

# Get the real user who ran sudo
REAL_USER="${SUDO_USER:-$USER}"
SCRIPT_DIR="$(dirname "$(readlink -f "$0")")"
SPARKLE_DIR="$SCRIPT_DIR/.sparkle"

echo "🔑 Running as root, will set permissions for user: $REAL_USER"

# Verify Sparkle installation
if [ ! -d "$SPARKLE_DIR" ]; then
    echo "❌ Error: Sparkle not installed. Please run mega_fix.sh first."
    exit 1
fi

# Create new std directories
echo "📚 Creating new std directories..."
mkdir -p "$SPARKLE_DIR/std/itex"
mkdir -p "$SPARKLE_DIR/std/lend"
mkdir -p "$SPARKLE_DIR/std/mark"
mkdir -p "$SPARKLE_DIR/std/murmur"

# Update SeedManager.jl to include new components
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
        "itex",      # Interactive text manipulation\
        "lend",      # Resource lending utilities\
        "mark",      # Markup processing\
        "murmur"     # Murmur hash implementation\
    ]\
)' "$SPARKLE_DIR/src/SeedManager.jl"

# Create std/itex implementation
echo "📝 Creating std/itex/init.jl..."
cat > "$SPARKLE_DIR/std/itex/init.jl" << 'EOT'
module Itex

export interactive_replace, prompt_edit, suggest_completion
export highlight_diff, apply_patch, revert_changes

using REPL
using Crayons

"""
Interactive text replacement with preview
"""
function interactive_replace(text::String, pattern::String)
    matches = findall(pattern, text)
    if isempty(matches)
        return text
    end

    result = text
    for match in matches
        before = result[1:prevind(result, first(match))]
        highlighted = crayon"red"(result[match])
        after = result[nextind(result, last(match)):end]

        println("\nFound: ", before, highlighted, after)
        print("Replace with (empty to skip): ")
        replacement = readline()

        if !isempty(replacement)
            result = before * replacement * after
        end
    end
    result
end

"""
Prompt for interactive text editing
"""
function prompt_edit(text::String)
    temp_file = tempname()
    write(temp_file, text)
    run(`$(ENV["EDITOR"]) $temp_file`)
    result = read(temp_file, String)
    rm(temp_file)
    result
end

"""
Suggest completions for text
"""
function suggest_completion(text::String, dictionary::Vector{String})
    matches = filter(w -> startswith(w, text), dictionary)
    isempty(matches) ? nothing : matches
end

"""
Highlight differences between two texts
"""
function highlight_diff(old::String, new::String)
    old_lines = split(old, '\n')
    new_lines = split(new, '\n')

    for (i, (old_line, new_line)) in enumerate(zip(old_lines, new_lines))
        if old_line != new_line
            println("Line $i:")
            println("- ", crayon"red"(old_line))
            println("+ ", crayon"green"(new_line))
        end
    end
end

"""
Apply patch to text
"""
function apply_patch(text::String, patch::String)
    # Simple patch format: @@ -start,length +start,length @@
    # Followed by context lines
    # TODO: Implement proper patch parsing
    text
end

"""
Revert last change
"""
function revert_changes(text::String, history::Vector{String})
    isempty(history) ? text : history[end]
end

end
EOT

# Create std/lend implementation
echo "📝 Creating std/lend/init.jl..."
cat > "$SPARKLE_DIR/std/lend/init.jl" << 'EOT'
module Lend

export borrow, return_borrowed, with_borrowed
export track_usage, show_borrowed, clear_borrowed

using Base.Threads

mutable struct BorrowedResource{T}
    value::T
    borrowed::Bool
    owner::String
    timestamp::Float64
end

const BORROWED_RESOURCES = Dict{String, BorrowedResource}()
const BORROW_LOCK = ReentrantLock()

"""
Borrow a resource
"""
function borrow(resource::T, name::String) where T
    lock(BORROW_LOCK) do
        if haskey(BORROWED_RESOURCES, name)
            throw(ErrorException("Resource $name is already borrowed"))
        end
        BORROWED_RESOURCES[name] = BorrowedResource(resource, true, string(current_task()), time())
        resource
    end
end

"""
Return a borrowed resource
"""
function return_borrowed(name::String)
    lock(BORROW_LOCK) do
        if !haskey(BORROWED_RESOURCES, name)
            throw(ErrorException("Resource $name was not borrowed"))
        end
        resource = BORROWED_RESOURCES[name]
        delete!(BORROWED_RESOURCES, name)
        resource.value
    end
end

"""
Use resource within a scope
"""
function with_borrowed(f::Function, resource::T, name::String) where T
    borrowed = borrow(resource, name)
    try
        f(borrowed)
    finally
        return_borrowed(name)
    end
end

"""
Track resource usage
"""
function track_usage(name::String)
    lock(BORROW_LOCK) do
        if haskey(BORROWED_RESOURCES, name)
            resource = BORROWED_RESOURCES[name]
            (
                borrowed=resource.borrowed,
                owner=resource.owner,
                duration=time() - resource.timestamp
            )
        else
            nothing
        end
    end
end

"""
Show all borrowed resources
"""
function show_borrowed()
    lock(BORROW_LOCK) do
        Dict(name => (
            owner=resource.owner,
            duration=time() - resource.timestamp
        ) for (name, resource) in BORROWED_RESOURCES)
    end
end

"""
Clear all borrowed resources
"""
function clear_borrowed()
    lock(BORROW_LOCK) do
        empty!(BORROWED_RESOURCES)
    end
end

end
EOT

# Create std/mark implementation
echo "📝 Creating std/mark/init.jl..."
cat > "$SPARKLE_DIR/std/mark/init.jl" << 'EOT'
module Mark

export parse_markdown, to_html, to_latex
export extract_links, extract_headers, create_toc

"""
Parse markdown text into internal structure
"""
function parse_markdown(text::String)
    # Simple markdown parser
    # TODO: Implement proper markdown parsing
    lines = split(text, '\n')
    result = []

    for line in lines
        if startswith(line, "#")
            level = length(match(r"^#+", line).match)
            push!(result, (:header, level, strip(line[level+1:end])))
        elseif startswith(line, ">")
            push!(result, (:quote, strip(line[2:end])))
        elseif startswith(line, "- ")
            push!(result, (:list_item, strip(line[2:end])))
        elseif !isempty(line)
            push!(result, (:text, line))
        end
    end

    result
end

"""
Convert markdown to HTML
"""
function to_html(markdown::Vector{Tuple})
    result = []
    for (type, args...) in markdown
        if type == :header
            level, text = args
            push!(result, "<h$level>$text</h$level>")
        elseif type == :quote
            push!(result, "<blockquote>$(args[1])</blockquote>")
        elseif type == :list_item
            push!(result, "<li>$(args[1])</li>")
        elseif type == :text
            push!(result, "<p>$(args[1])</p>")
        end
    end
    join(result, "\n")
end

"""
Convert markdown to LaTeX
"""
function to_latex(markdown::Vector{Tuple})
    result = []
    for (type, args...) in markdown
        if type == :header
            level, text = args
            section = level == 1 ? "section" :
                     level == 2 ? "subsection" : "subsubsection"
            push!(result, "\\$section{$text}")
        elseif type == :quote
            push!(result, "\\begin{quote}\n$(args[1])\n\\end{quote}")
        elseif type == :list_item
            push!(result, "\\item $(args[1])")
        elseif type == :text
            push!(result, args[1])
        end
    end
    join(result, "\n")
end

"""
Extract all links from markdown text
"""
function extract_links(text::String)
    matches = eachmatch(r"\[([^\]]+)\]\(([^\)]+)\)", text)
    [(m.captures[1], m.captures[2]) for m in matches]
end

"""
Extract headers from markdown text
"""
function extract_headers(text::String)
    lines = split(text, '\n')
    filter(line -> startswith(line, '#'), lines)
end

"""
Create table of contents from markdown
"""
function create_toc(text::String)
    headers = extract_headers(text)
    result = []
    for header in headers
        level = length(match(r"^#+", header).match)
        title = strip(header[level+1:end])
        push!(result, "  "^(level-1) * "- " * title)
    end
    join(result, "\n")
end

end
EOT

# Create std/murmur implementation
echo "📝 Creating std/murmur/init.jl..."
cat > "$SPARKLE_DIR/std/murmur/init.jl" << 'EOT'
module Murmur

export murmur32, murmur64, murmur128
export hash_file, hash_stream

"""
32-bit Murmur3 hash
"""
function murmur32(data::Vector{UInt8}, seed::UInt32=0x00000000)
    len = length(data)
    nblocks = div(len, 4)
    h1 = seed

    # Constants
    c1 = 0xcc9e2d51
    c2 = 0x1b873593

    # Body
    for block in 1:nblocks
        k1 = UInt32(data[block*4-3]) |
             UInt32(data[block*4-2]) << 8 |
             UInt32(data[block*4-1]) << 16 |
             UInt32(data[block*4]) << 24

        k1 *= c1
        k1 = (k1 << 15) | (k1 >> 17)
        k1 *= c2

        h1 ⊻= k1
        h1 = (h1 << 13) | (h1 >> 19)
        h1 = h1 * 5 + 0xe6546b64
    end

    # Tail
    tail_start = nblocks * 4 + 1
    k1 = 0x00000000
    if tail_start <= len
        for i in 0:min(len-tail_start, 3)
            k1 ⊻= UInt32(data[tail_start+i]) << (i * 8)
        end
        k1 *= c1
        k1 = (k1 << 15) | (k1 >> 17)
        k1 *= c2
        h1 ⊻= k1
    end

    # Finalization
    h1 ⊻= UInt32(len)
    h1 ⊻= h1 >> 16
    h1 *= 0x85ebca6b
    h1 ⊻= h1 >> 13
    h1 *= 0xc2b2ae35
    h1 ⊻= h1 >> 16

    h1
end

"""
64-bit Murmur2 hash
"""
function murmur64(data::Vector{UInt8}, seed::UInt64=0x0000000000000000)
    len = length(data)
    nblocks = div(len, 8)
    h = seed ⊻ (len * 0xc6a4a7935bd1e995)

    # Constants
    m = 0xc6a4a7935bd1e995
    r = 47

    # Body
    for block in 1:nblocks
        k = UInt64(data[block*8-7]) |
            UInt64(data[block*8-6]) << 8 |
            UInt64(data[block*8-5]) << 16 |
            UInt64(data[block*8-4]) << 24 |
            UInt64(data[block*8-3]) << 32 |
            UInt64(data[block*8-2]) << 40 |
            UInt64(data[block*8-1]) << 48 |
            UInt64(data[block*8]) << 56

        k *= m
        k ⊻= k >> r
        k *= m

        h ⊻= k
        h *= m
    end

    # Tail
    tail_start = nblocks * 8 + 1
    if tail_start <= len
        for i in 0:min(len-tail_start, 7)
            h ⊻= UInt64(data[tail_start+i]) << (i * 8)
        end
        h *= m
    end

    # Finalization
    h ⊻= h >> r
    h *= m
    h ⊻= h >> r

    h
end

"""
128-bit Murmur3 hash
"""
function murmur128(data::Vector{UInt8}, seed::UInt64=0x0000000000000000)
    len = length(data)
    nblocks = div(len, 16)

    # Constants
    c1 = 0x87c37b91114253d5
    c2 = 0x4cf5ad432745937f

    # Initialize hash values
    h1 = seed
    h2 = seed

    # Body
    for block in 1:nblocks
        # Get 128 bits
        k1 = UInt64(data[block*16-15]) |
            UInt64(data[block*16-14]) << 8 |
            UInt64(data[block*16-13]) << 16 |
            UInt64(data[block*16-12]) << 24 |
            UInt64(data[block*16-11]) << 32 |
            UInt64(data[block*16-10]) << 40 |
            UInt64(data[block*16-9]) << 48 |
            UInt64(data[block*16-8]) << 56

        k2 = UInt64(data[block*16-7]) |
            UInt64(data[block*16-6]) << 8 |
            UInt64(data[block*16-5]) << 16 |
            UInt64(data[block*16-4]) << 24 |
            UInt64(data[block*16-3]) << 32 |
            UInt64(data[block*16-2]) << 40 |
            UInt64(data[block*16-1]) << 48 |
            UInt64(data[block*16]) << 56

        # Mix k1
        k1 *= c1
        k1 = (k1 << 31) | (k1 >> 33)
        k1 *= c2
        h1 ⊻= k1

        h1 = (h1 << 27) | (h1 >> 37)
        h1 += h2
        h1 = h1 * 5 + 0x52dce729

        # Mix k2
        k2 *= c2
        k2 = (k2 << 33) | (k2 >> 31)
        k2 *= c1
        h2 ⊻= k2

        h2 = (h2 << 31) | (h2 >> 33)
        h2 += h1
        h2 = h2 * 5 + 0x38495ab5
    end

    # Tail
    tail_start = nblocks * 16 + 1
    if tail_start <= len
        k1 = 0x0000000000000000
        k2 = 0x0000000000000000

        if len - tail_start >= 15
            k2 ⊻= UInt64(data[tail_start+14]) << 48
        end
        if len - tail_start >= 14
            k2 ⊻= UInt64(data[tail_start+13]) << 40
        end
        if len - tail_start >= 13
            k2 ⊻= UInt64(data[tail_start+12]) << 32
        end
        if len - tail_start >= 12
            k2 ⊻= UInt64(data[tail_start+11]) << 24
        end
        if len - tail_start >= 11
            k2 ⊻= UInt64(data[tail_start+10]) << 16
        end
        if len - tail_start >= 10
            k2 ⊻= UInt64(data[tail_start+9]) << 8
        end
        if len - tail_start >= 9
            k2 ⊻= UInt64(data[tail_start+8])
            k2 *= c2
            k2 = (k2 << 33) | (k2 >> 31)
            k2 *= c1
            h2 ⊻= k2
        end

        if len - tail_start >= 8
            k1 ⊻= UInt64(data[tail_start+7]) << 56
        end
        if len - tail_start >= 7
            k1 ⊻= UInt64(data[tail_start+6]) << 48
        end
        if len - tail_start >= 6
            k1 ⊻= UInt64(data[tail_start+5]) << 40
        end
        if len - tail_start >= 5
            k1 ⊻= UInt64(data[tail_start+4]) << 32
        end
        if len - tail_start >= 4
            k1 ⊻= UInt64(data[tail_start+3]) << 24
        end
        if len - tail_start >= 3
            k1 ⊻= UInt64(data[tail_start+2]) << 16
        end
        if len - tail_start >= 2
            k1 ⊻= UInt64(data[tail_start+1]) << 8
        end
        if len - tail_start >= 1
            k1 ⊻= UInt64(data[tail_start])
            k1 *= c1
            k1 = (k1 << 31) | (k1 >> 33)
            k1 *= c2
            h1 ⊻= k1
        end
    end

    # Finalization
    h1 ⊻= UInt64(len)
    h2 ⊻= UInt64(len)

    h1 += h2
    h2 += h1

    # Final mix functions
    h1 ⊻= h1 >> 33
    h1 *= 0xff51afd7ed558ccd
    h1 ⊻= h1 >> 33
    h1 *= 0xc4ceb9fe1a85ec53
    h1 ⊻= h1 >> 33

    h2 ⊻= h2 >> 33
    h2 *= 0xff51afd7ed558ccd
    h2 ⊻= h2 >> 33
    h2 *= 0xc4ceb9fe1a85ec53
    h2 ⊻= h2 >> 33

    return (h1, h2)
end

"""
Hash a file using Murmur3
"""
function hash_file(filename::String; bits::Integer=32)
    open(filename, "r") do file
        data = read(file)
        if bits == 32
            murmur32(data)
        elseif bits == 64
            murmur64(data)
        elseif bits == 128
            murmur128(data)
        else
            throw(ArgumentError("bits must be 32, 64, or 128"))
        end
    end
end

"""
Hash a stream using Murmur3
"""
function hash_stream(io::IO; bits::Integer=32)
    data = read(io)
    if bits == 32
        murmur32(data)
    elseif bits == 64
        murmur64(data)
    elseif bits == 128
        murmur128(data)
    else
        throw(ArgumentError("bits must be 32, 64, or 128"))
    end
end

end
EOT

# Set permissions
echo "🔒 Setting permissions..."
chown -R "$REAL_USER:$REAL_USER" "$SPARKLE_DIR"
find "$SPARKLE_DIR" -type d -exec chmod 755 {} \;
find "$SPARKLE_DIR" -type f -exec chmod 644 {} \;

echo "✨ Additional standard library components have been forged!"
echo "Available components:"
echo "  - std**itex    (Interactive text manipulation)"
echo "  - std**lend    (Resource lending utilities)"
echo "  - std**mark    (Markup processing)"
echo "  - std**murmur  (Murmur hash implementation)"
echo "Try 'seed plant std**mark' in Sparkle."
