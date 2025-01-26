#!/usr/bin/env bash

# Sudo Forge RunSTD7 - Sparkle Standard Library Implementation Part 7
# Author: isdood
# Created: 2025-01-26 13:06:29 UTC
# Repository: isdood/scribble

# Check if script is run as root
if [ "$EUID" -ne 0 ]; then
    echo "🔒 This script requires root permissions to set up the Sparkle environment"
    echo "Running with sudo..."
    sudo "$0" "$@"
    exit $?
fi

set -e

echo "⚒️ Forge RunSTD7 - Final Standard Library Components"
echo "Created: 2025-01-26 13:06:29 UTC"
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
mkdir -p "$SPARKLE_DIR/std/shimmer"
mkdir -p "$SPARKLE_DIR/std/signal"
mkdir -p "$SPARKLE_DIR/std/spell"
mkdir -p "$SPARKLE_DIR/std/thunder"

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

# Create std/shimmer implementation
echo "📝 Creating std/shimmer/init.jl..."
cat > "$SPARKLE_DIR/std/shimmer/init.jl" << 'EOT'
module Shimmer

export animate, sparkle, fade, pulse
export ShimmerEffect, apply_effect, chain_effects

using UnicodePlots
using ColorSchemes

"""
Animation effect types
"""
@enum ShimmerEffect begin
    SPARKLE
    FADE
    PULSE
    RAINBOW
    GLITTER
    WAVE
end

"""
Animate text with effect
"""
function animate(text::String, effect::ShimmerEffect; duration::Float64=1.0)
    frames = Int(duration * 30)  # 30 fps
    for i in 1:frames
        # Clear previous frame
        print("\033[2K\033[1G")

        # Apply effect
        if effect == SPARKLE
            print(_sparkle_frame(text, i))
        elseif effect == FADE
            print(_fade_frame(text, i, frames))
        elseif effect == PULSE
            print(_pulse_frame(text, i))
        elseif effect == RAINBOW
            print(_rainbow_frame(text, i))
        elseif effect == GLITTER
            print(_glitter_frame(text, i))
        elseif effect == WAVE
            print(_wave_frame(text, i))
        end

        flush(stdout)
        sleep(1/30)
    end
    println()
end

"""
Create sparkle effect
"""
function sparkle(text::String; duration::Float64=1.0)
    animate(text, SPARKLE, duration=duration)
end

"""
Create fade effect
"""
function fade(text::String; duration::Float64=1.0)
    animate(text, FADE, duration=duration)
end

"""
Create pulse effect
"""
function pulse(text::String; duration::Float64=1.0)
    animate(text, PULSE, duration=duration)
end

# Internal frame generation functions
function _sparkle_frame(text::String, frame::Int)
    chars = collect(text)
    spark_pos = rand(1:length(chars))
    chars[spark_pos] = '✨'
    join(chars)
end

function _fade_frame(text::String, frame::Int, total_frames::Int)
    opacity = abs(sin(π * frame / total_frames))
    "\033[38;5;$(Int(round(255 * opacity)))m$text\033[0m"
end

function _pulse_frame(text::String, frame::Int)
    scale = 1.0 + 0.2 * sin(2π * frame / 30)
    "\033[${scale}m$text\033[0m"
end

function _rainbow_frame(text::String, frame::Int)
    colors = ColorSchemes.rainbow
    join(["\033[38;5;$(colors[(i+frame)%length(colors)])m$(c)" for (i,c) in enumerate(text)])
end

function _glitter_frame(text::String, frame::Int)
    chars = collect(text)
    for i in 1:length(chars)
        if rand() < 0.1
            chars[i] = '.'
        end
    end
    join(chars)
end

function _wave_frame(text::String, frame::Int)
    chars = collect(text)
    for (i, c) in enumerate(chars)
        offset = sin(2π * (i + frame) / 20)
        chars[i] = ' ' ^ Int(round(2 + 2 * offset)) * c
    end
    join(chars)
end

end
EOT

# Create std/signal implementation
echo "📝 Creating std/signal/init.jl..."
cat > "$SPARKLE_DIR/std/signal/init.jl" << 'EOT'
module Signal

export on, off, emit, once
export SignalHandler, connect, disconnect
export @signal, @handler

using Base.Threads

"""
Signal handler type
"""
mutable struct SignalHandler
    name::Symbol
    handlers::Vector{Function}
    once::Vector{Bool}
    mutex::ReentrantLock
end

const SIGNAL_HANDLERS = Dict{Symbol, SignalHandler}()
const HANDLER_MUTEX = ReentrantLock()

"""
Create or get signal handler
"""
function get_handler(name::Symbol)
    lock(HANDLER_MUTEX) do
        if !haskey(SIGNAL_HANDLERS, name)
            SIGNAL_HANDLERS[name] = SignalHandler(name, Function[], Bool[], ReentrantLock())
        end
        SIGNAL_HANDLERS[name]
    end
end

"""
Register event handler
"""
function on(signal::Symbol, handler::Function; once::Bool=false)
    h = get_handler(signal)
    lock(h.mutex) do
        push!(h.handlers, handler)
        push!(h.once, once)
    end
    nothing
end

"""
Remove event handler
"""
function off(signal::Symbol, handler::Function)
    h = get_handler(signal)
    lock(h.mutex) do
        idx = findfirst(==(handler), h.handlers)
        if !isnothing(idx)
            deleteat!(h.handlers, idx)
            deleteat!(h.once, idx)
        end
    end
    nothing
end

"""
Emit signal
"""
function emit(signal::Symbol, args...)
    h = get_handler(signal)
    to_remove = Int[]

    lock(h.mutex) do
        for (i, (handler, once)) in enumerate(zip(h.handlers, h.once))
            @async handler(args...)
            if once
                push!(to_remove, i)
            end
        end

        # Remove one-time handlers
        deleteat!(h.handlers, to_remove)
        deleteat!(h.once, to_remove)
    end
    nothing
end

"""
Register one-time event handler
"""
function once(signal::Symbol, handler::Function)
    on(signal, handler, once=true)
end

"""
Connect multiple signals
"""
function connect(connections::Dict{Symbol,Function})
    for (signal, handler) in connections
        on(signal, handler)
    end
end

"""
Disconnect multiple signals
"""
function disconnect(connections::Dict{Symbol,Function})
    for (signal, handler) in connections
        off(signal, handler)
    end
end

# Macros for signal handling
macro signal(name)
    quote
        const $(esc(name)) = $(QuoteNode(name))
    end
end

macro handler(signal, expr)
    quote
        on($(esc(signal)), $(esc(expr)))
    end
end

end
EOT

# Create std/spell implementation
echo "📝 Creating std/spell/init.jl..."
cat > "$SPARKLE_DIR/std/spell/init.jl" << 'EOT'
module Spell

export cast, bind, unbind, modify
export SpellPattern, compile_spell, compose_spells

"""
Spell pattern for command execution
"""
struct SpellPattern
    pattern::String
    action::Function
    description::String
end

const SPELL_REGISTRY = Dict{String, SpellPattern}()

"""
Create and register a new spell
"""
function cast(pattern::String, action::Function; description::String="")
    spell = SpellPattern(pattern, action, description)
    SPELL_REGISTRY[pattern] = spell
    spell
end

"""
Bind data to spell pattern
"""
function bind(data, pattern::String)
    if !haskey(SPELL_REGISTRY, pattern)
        throw(KeyError("Spell pattern not found: $pattern"))
    end

    spell = SPELL_REGISTRY[pattern]
    spell.action(data)
end

"""
Remove spell binding
"""
function unbind(pattern::String)
    delete!(SPELL_REGISTRY, pattern)
end

"""
Modify existing spell
"""
function modify(pattern::String, new_action::Function)
    if !haskey(SPELL_REGISTRY, pattern)
        throw(KeyError("Spell pattern not found: $pattern"))
    end

    old_spell = SPELL_REGISTRY[pattern]
    SPELL_REGISTRY[pattern] = SpellPattern(
        pattern,
        new_action,
        old_spell.description
    )
end

"""
Compile spell pattern
"""
function compile_spell(pattern::String)
    try
        Regex(pattern)
        true
    catch
        false
    end
end

"""
Compose multiple spells
"""
function compose_spells(spells::Vector{SpellPattern})
    data -> begin
        results = []
        for spell in spells
            try
                push!(results, spell.action(data))
            catch e
                @warn "Spell failed: $(spell.pattern)" exception=e
            end
        end
        results
    end
end

end
EOT

# Create std/thunder implementation
echo "📝 Creating std/thunder/init.jl..."
cat > "$SPARKLE_DIR/std/thunder/init.jl" << 'EOT'
module Thunder

export parallel_map, parallel_filter, parallel_reduce
export @thunder, @storm, distribute_work

using Base.Threads

"""
Parallel map implementation
"""
function parallel_map(f::Function, collection; chunk_size=nothing)
    n = length(collection)
    if isnothing(chunk_size)
        chunk_size = max(1, div(n, nthreads()))
    end

    chunks = [collection[i:min(i+chunk_size-1, n)] for i in 1:chunk_size:n]
    results = Vector{Any}(undef, length(chunks))

    @threads for i in eachindex(chunks)
        results[i] = map(f, chunks[i])
    end

    reduce(vcat, results)
end

"""
Parallel filter implementation
"""
function parallel_filter(pred::Function, collection; chunk_size=nothing)
    n = length(collection)
    if isnothing(chunk_size)
        chunk_size = max(1, div(n, nthreads()))
    end

    chunks = [collection[i:min(i+chunk_size-1, n)] for i in 1:chunk_size:n]
    results = Vector{Any}(undef, length(chunks))

    @threads for i in eachindex(chunks)
        results[i] = filter(pred, chunks[i])
    end

    reduce(vcat, results)
end

"""
Parallel reduce implementation
"""
function parallel_reduce(f::Function, collection; init=nothing)
    if isnothing(init)
        if isempty(collection)
            throw(ArgumentError("empty collection with no init value"))
        end
        init = first(collection)
        collection = collection[2:end]
    end

    n = length(collection)
    chunk_size = max(1, div(n, nthreads()))
    chunks = [collection[i:min(i+chunk_size-1, n)] for i in 1:chunk_size:n]
    results = Vector{Any}(undef, length(chunks))

    @threads for i in eachindex(chunks)
        results[i] = reduce(f, chunks[i], init=init)
    end

    reduce(f, results)
end

"""
Thunder macro for parallel execution
"""
macro thunder(expr)
    quote
        @threads for i in 1:nthreads()
            $(esc(expr))
        end
    end
end

"""
Storm macro for parallel batch processing
"""
macro storm(collection, expr)
    quote
        local data = $(esc(collection))
        local n = length(data)
        local chunk_size = max(1, div(n, nthreads()))
        local chunks = [data[i:min(i+chunk_size-1, n)] for i in 1:chunk_size:n]
        local results = Vector{Any}(undef, length(chunks))

        @threads for i in eachindex(chunks)
            results[i] = map(x -> $(esc(expr)), chunks[i])
        end

        reduce(vcat, results)
    end
end

"""
Distribute work across threads
"""
function distribute_work(tasks::Vector{<:Function})
    results = Vector{Any}(undef, length(tasks))

    @threads for i in eachindex(tasks)
        results[i] = tasks[i]()
    end

    results
end

end
EOT

#!/usr/bin/env bash

# Sudo Forge RunSTD7 - Sparkle Standard Library Implementation Part 7
# Author: isdood
# Created: 2025-01-26 13:06:29 UTC
# Repository: isdood/scribble

# Check if script is run as root
if [ "$EUID" -ne 0 ]; then
    echo "🔒 This script requires root permissions to set up the Sparkle environment"
    echo "Running with sudo..."
    sudo "$0" "$@"
    exit $?
fi

set -e

echo "⚒️ Forge RunSTD7 - Final Standard Library Components"
echo "Created: 2025-01-26 13:06:29 UTC"
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
mkdir -p "$SPARKLE_DIR/std/shimmer"
mkdir -p "$SPARKLE_DIR/std/signal"
mkdir -p "$SPARKLE_DIR/std/spell"
mkdir -p "$SPARKLE_DIR/std/thunder"

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

# Create std/shimmer implementation
echo "📝 Creating std/shimmer/init.jl..."
cat > "$SPARKLE_DIR/std/shimmer/init.jl" << 'EOT'
module Shimmer

export animate, sparkle, fade, pulse
export ShimmerEffect, apply_effect, chain_effects

using UnicodePlots
using ColorSchemes

"""
Animation effect types
"""
@enum ShimmerEffect begin
    SPARKLE
    FADE
    PULSE
    RAINBOW
    GLITTER
    WAVE
end

"""
Animate text with effect
"""
function animate(text::String, effect::ShimmerEffect; duration::Float64=1.0)
    frames = Int(duration * 30)  # 30 fps
    for i in 1:frames
        # Clear previous frame
        print("\033[2K\033[1G")

        # Apply effect
        if effect == SPARKLE
            print(_sparkle_frame(text, i))
        elseif effect == FADE
            print(_fade_frame(text, i, frames))
        elseif effect == PULSE
            print(_pulse_frame(text, i))
        elseif effect == RAINBOW
            print(_rainbow_frame(text, i))
        elseif effect == GLITTER
            print(_glitter_frame(text, i))
        elseif effect == WAVE
            print(_wave_frame(text, i))
        end

        flush(stdout)
        sleep(1/30)
    end
    println()
end

"""
Create sparkle effect
"""
function sparkle(text::String; duration::Float64=1.0)
    animate(text, SPARKLE, duration=duration)
end

"""
Create fade effect
"""
function fade(text::String; duration::Float64=1.0)
    animate(text, FADE, duration=duration)
end

"""
Create pulse effect
"""
function pulse(text::String; duration::Float64=1.0)
    animate(text, PULSE, duration=duration)
end

# Internal frame generation functions
function _sparkle_frame(text::String, frame::Int)
    chars = collect(text)
    spark_pos = rand(1:length(chars))
    chars[spark_pos] = '✨'
    join(chars)
end

function _fade_frame(text::String, frame::Int, total_frames::Int)
    opacity = abs(sin(π * frame / total_frames))
    "\033[38;5;$(Int(round(255 * opacity)))m$text\033[0m"
end

function _pulse_frame(text::String, frame::Int)
    scale = 1.0 + 0.2 * sin(2π * frame / 30)
    "\033[${scale}m$text\033[0m"
end

function _rainbow_frame(text::String, frame::Int)
    colors = ColorSchemes.rainbow
    join(["\033[38;5;$(colors[(i+frame)%length(colors)])m$(c)" for (i,c) in enumerate(text)])
end

function _glitter_frame(text::String, frame::Int)
    chars = collect(text)
    for i in 1:length(chars)
        if rand() < 0.1
            chars[i] = '.'
        end
    end
    join(chars)
end

function _wave_frame(text::String, frame::Int)
    chars = collect(text)
    for (i, c) in enumerate(chars)
        offset = sin(2π * (i + frame) / 20)
        chars[i] = ' ' ^ Int(round(2 + 2 * offset)) * c
    end
    join(chars)
end

end
EOT

# Create std/signal implementation
echo "📝 Creating std/signal/init.jl..."
cat > "$SPARKLE_DIR/std/signal/init.jl" << 'EOT'
module Signal

export on, off, emit, once
export SignalHandler, connect, disconnect
export @signal, @handler

using Base.Threads

"""
Signal handler type
"""
mutable struct SignalHandler
    name::Symbol
    handlers::Vector{Function}
    once::Vector{Bool}
    mutex::ReentrantLock
end

const SIGNAL_HANDLERS = Dict{Symbol, SignalHandler}()
const HANDLER_MUTEX = ReentrantLock()

"""
Create or get signal handler
"""
function get_handler(name::Symbol)
    lock(HANDLER_MUTEX) do
        if !haskey(SIGNAL_HANDLERS, name)
            SIGNAL_HANDLERS[name] = SignalHandler(name, Function[], Bool[], ReentrantLock())
        end
        SIGNAL_HANDLERS[name]
    end
end

"""
Register event handler
"""
function on(signal::Symbol, handler::Function; once::Bool=false)
    h = get_handler(signal)
    lock(h.mutex) do
        push!(h.handlers, handler)
        push!(h.once, once)
    end
    nothing
end

"""
Remove event handler
"""
function off(signal::Symbol, handler::Function)
    h = get_handler(signal)
    lock(h.mutex) do
        idx = findfirst(==(handler), h.handlers)
        if !isnothing(idx)
            deleteat!(h.handlers, idx)
            deleteat!(h.once, idx)
        end
    end
    nothing
end

"""
Emit signal
"""
function emit(signal::Symbol, args...)
    h = get_handler(signal)
    to_remove = Int[]

    lock(h.mutex) do
        for (i, (handler, once)) in enumerate(zip(h.handlers, h.once))
            @async handler(args...)
            if once
                push!(to_remove, i)
            end
        end

        # Remove one-time handlers
        deleteat!(h.handlers, to_remove)
        deleteat!(h.once, to_remove)
    end
    nothing
end

"""
Register one-time event handler
"""
function once(signal::Symbol, handler::Function)
    on(signal, handler, once=true)
end

"""
Connect multiple signals
"""
function connect(connections::Dict{Symbol,Function})
    for (signal, handler) in connections
        on(signal, handler)
    end
end

"""
Disconnect multiple signals
"""
function disconnect(connections::Dict{Symbol,Function})
    for (signal, handler) in connections
        off(signal, handler)
    end
end

# Macros for signal handling
macro signal(name)
    quote
        const $(esc(name)) = $(QuoteNode(name))
    end
end

macro handler(signal, expr)
    quote
        on($(esc(signal)), $(esc(expr)))
    end
end

end
EOT

# Create std/spell implementation
echo "📝 Creating std/spell/init.jl..."
cat > "$SPARKLE_DIR/std/spell/init.jl" << 'EOT'
module Spell

export cast, bind, unbind, modify
export SpellPattern, compile_spell, compose_spells

"""
Spell pattern for command execution
"""
struct SpellPattern
    pattern::String
    action::Function
    description::String
end

const SPELL_REGISTRY = Dict{String, SpellPattern}()

"""
Create and register a new spell
"""
function cast(pattern::String, action::Function; description::String="")
    spell = SpellPattern(pattern, action, description)
    SPELL_REGISTRY[pattern] = spell
    spell
end

"""
Bind data to spell pattern
"""
function bind(data, pattern::String)
    if !haskey(SPELL_REGISTRY, pattern)
        throw(KeyError("Spell pattern not found: $pattern"))
    end

    spell = SPELL_REGISTRY[pattern]
    spell.action(data)
end

"""
Remove spell binding
"""
function unbind(pattern::String)
    delete!(SPELL_REGISTRY, pattern)
end

"""
Modify existing spell
"""
function modify(pattern::String, new_action::Function)
    if !haskey(SPELL_REGISTRY, pattern)
        throw(KeyError("Spell pattern not found: $pattern"))
    end

    old_spell = SPELL_REGISTRY[pattern]
    SPELL_REGISTRY[pattern] = SpellPattern(
        pattern,
        new_action,
        old_spell.description
    )
end

"""
Compile spell pattern
"""
function compile_spell(pattern::String)
    try
        Regex(pattern)
        true
    catch
        false
    end
end

"""
Compose multiple spells
"""
function compose_spells(spells::Vector{SpellPattern})
    data -> begin
        results = []
        for spell in spells
            try
                push!(results, spell.action(data))
            catch e
                @warn "Spell failed: $(spell.pattern)" exception=e
            end
        end
        results
    end
end

end
EOT

# Create std/thunder implementation
echo "📝 Creating std/thunder/init.jl..."
cat > "$SPARKLE_DIR/std/thunder/init.jl" << 'EOT'
module Thunder

export parallel_map, parallel_filter, parallel_reduce
export @thunder, @storm, distribute_work

using Base.Threads

"""
Parallel map implementation
"""
function parallel_map(f::Function, collection; chunk_size=nothing)
    n = length(collection)
    if isnothing(chunk_size)
        chunk_size = max(1, div(n, nthreads()))
    end

    chunks = [collection[i:min(i+chunk_size-1, n)] for i in 1:chunk_size:n]
    results = Vector{Any}(undef, length(chunks))

    @threads for i in eachindex(chunks)
        results[i] = map(f, chunks[i])
    end

    reduce(vcat, results)
end

"""
Parallel filter implementation
"""
function parallel_filter(pred::Function, collection; chunk_size=nothing)
    n = length(collection)
    if isnothing(chunk_size)
        chunk_size = max(1, div(n, nthreads()))
    end

    chunks = [collection[i:min(i+chunk_size-1, n)] for i in 1:chunk_size:n]
    results = Vector{Any}(undef, length(chunks))

    @threads for i in eachindex(chunks)
        results[i] = filter(pred, chunks[i])
    end

    reduce(vcat, results)
end

"""
Parallel reduce implementation
"""
function parallel_reduce(f::Function, collection; init=nothing)
    if isnothing(init)
        if isempty(collection)
            throw(ArgumentError("empty collection with no init value"))
        end
        init = first(collection)
        collection = collection[2:end]
    end

    n = length(collection)
    chunk_size = max(1, div(n, nthreads()))
    chunks = [collection[i:min(i+chunk_size-1, n)] for i in 1:chunk_size:n]
    results = Vector{Any}(undef, length(chunks))

    @threads for i in eachindex(chunks)
        results[i] = reduce(f, chunks[i], init=init)
    end

    reduce(f, results)
end

"""
Thunder macro for parallel execution
"""
macro thunder(expr)
    quote
        @threads for i in 1:nthreads()
            $(esc(expr))
        end
    end
end

"""
Storm macro for parallel batch processing
"""
macro storm(collection, expr)
    quote
        local data = $(esc(collection))
        local n = length(data)
        local chunk_size = max(1, div(n, nthreads()))
        local chunks = [data[i:min(i+chunk_size-1, n)] for i in 1:chunk_size:n]
        local results = Vector{Any}(undef, length(chunks))

        @threads for i in eachindex(chunks)
            results[i] = map(x -> $(esc(expr)), chunks[i])
        end

        reduce(vcat, results)
    end
end

"""
Distribute work across threads
"""
function distribute_work(tasks::Vector{<:Function})
    results = Vector{Any}(undef, length(tasks))

    @threads for i in eachindex(tasks)
        results[i] = tasks[i]()
    end

    results
end

end
EOT

# Set permissions
echo "🔒 Setting permissions..."
chown -R "$REAL_USER:$REAL_USER" "$SPARKLE_DIR"
find "$SPARKLE_DIR" -type d -exec chmod 755 {} \;
find "$SPARKLE_DIR" -type f -exec chmod 644 {} \;
