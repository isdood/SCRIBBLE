#!/usr/bin/env bash

# Sparkle - Spark Runtime Terminal v0.1
# Author: isdood
# Created: 2025-01-25 23:14:50 UTC
# Repository: isdood/scribble

set -e

SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
PKG_DIR="$SCRIPT_DIR/forge/julia/SparkJL"

# Generate a temporary directory for our sandbox
SANDBOX_DIR=$(mktemp -d)

# Create Project.toml
UUID=$(julia -e 'import UUIDs; println(UUIDs.uuid4())')
cat > "$SANDBOX_DIR/Project.toml" << EOF
name = "SparkSandbox"
uuid = "$UUID"
version = "0.1.0"

[deps]
REPL = "3fa0cd96-eef1-5676-8a61-b3b8758bbffb"
EOF

# Create the Julia initialization script
cat > "$SANDBOX_DIR/init.jl" << 'JULIA_EOF'
using REPL
using REPL.LineEdit

# Create Sparkle module to contain our REPL functionality
module Sparkle
export crystal, wave, weave, optimize

using REPL
using REPL.LineEdit

# Sparkle commands implementation
struct Crystal
    dimensions::Tuple{Int,Int,Int}
    spacing::Float64
end

struct Wave
    data::Vector{Float64}
    frequency::Float64
end

struct Pattern
    name::String
    transform::Function
end

# Global state
const patterns = Dict{String,Pattern}()
mutable struct SparkleState
    current_crystal::Union{Crystal,Nothing}
    current_wave::Union{Wave,Nothing}
    patterns::Dict{String,Pattern}
end

const GLOBAL_STATE = SparkleState(nothing, nothing, patterns)

# Command implementations
function crystal(dims::Tuple{Int,Int,Int}=(32,32,32), spacing::Float64=1.0)
    GLOBAL_STATE.current_crystal = Crystal(dims, spacing)
    println("Created crystal structure with dimensions $(dims) and spacing $(spacing)")
    return GLOBAL_STATE.current_crystal
end

function wave(n::Int=100)
    data = randn(n)
    GLOBAL_STATE.current_wave = Wave(data, 1.0)
    println("Created wave pattern with $(n) points")
    return GLOBAL_STATE.current_wave
end

function weave(pattern::String="default")
    if isnothing(GLOBAL_STATE.current_wave)
        println("Error: No wave pattern to weave. Create one first with 'wave'")
        return nothing
    end
    if !haskey(patterns, pattern)
        println("Error: Pattern '$(pattern)' not found")
        return nothing
    end
    println("Applied $(pattern) weave pattern to wave")
    result = patterns[pattern].transform(GLOBAL_STATE.current_wave)
    println("Pattern applied successfully")
    return result
end

function optimize()
    if isnothing(GLOBAL_STATE.current_crystal) && isnothing(GLOBAL_STATE.current_wave)
        println("Error: Nothing to optimize. Create a crystal or wave first")
        return nothing
    end
    println("\nOptimizing current structure...")

    if !isnothing(GLOBAL_STATE.current_crystal)
        println("• Crystal optimization:")
        println("  - Dimensions: $(GLOBAL_STATE.current_crystal.dimensions)")
        println("  - Spacing: $(GLOBAL_STATE.current_crystal.spacing)")
        println("  ✓ Crystal optimization complete")
    end

    if !isnothing(GLOBAL_STATE.current_wave)
        println("• Wave optimization:")
        println("  - Points: $(length(GLOBAL_STATE.current_wave.data))")
        println("  - Frequency: $(GLOBAL_STATE.current_wave.frequency)")
        println("  ✓ Wave optimization complete")
    end

    return (crystal=GLOBAL_STATE.current_crystal, wave=GLOBAL_STATE.current_wave)
end

# Register default patterns
patterns["default"] = Pattern("Default", w -> w)
patterns["invert"] = Pattern("Invert", w -> Wave(-w.data, w.frequency))
patterns["double"] = Pattern("Double", w -> Wave(w.data .* 2, w.frequency))

# Process sparkle commands
function process_sparkle(s::LineEdit.MIState)
    buf = LineEdit.buffer(s)
    input = String(take!(copy(buf))::Vector{UInt8})

    if input == "?" || input == "help"
        println("""
        Sparkle Commands:
        ?/help                          - Show this help
        crystal([dims], [spacing])      - Create a new crystal structure
                                         dims: Tuple of 3 integers (default: (32,32,32))
                                         spacing: Float64 (default: 1.0)
        wave([n])                       - Create a new wave pattern
                                         n: Integer number of points (default: 100)
        weave([pattern])               - Apply weave pattern to current wave
                                         pattern: String (default: "default")
                                         Available patterns: $(join(keys(patterns), ", "))
        optimize                       - Optimize current structure
        exit/quit                      - Exit Sparkle mode
        """)
    elseif input == "exit" || input == "quit"
        println("Exiting Sparkle mode...")
        LineEdit.transition(s, Base.active_repl.interface.modes[1])
    else
        try
            expr = Meta.parse(input)
            # Check if it's just a bare symbol (command name)
            if expr isa Symbol
                # Convert symbol to function call
                expr = Expr(:call, expr)
            end
            # Evaluate in Sparkle module context
            result = Base.eval(Sparkle, expr)
            if result !== nothing
                if result isa NamedTuple  # For optimize results
                    # Results already printed in function
                elseif result isa Union{Crystal,Wave}  # For crystal/wave results
                    # Already printed in the function
                end
            end
        catch e
            printstyled("Error: ", bold=true, color=:red)
            println(e)
        end
    end
    REPL.reset_state(s)
    return nothing
end

function init()
    repl = Base.active_repl
    terminal = repl.t

    # Create the sparkle prompt
    sparkle = LineEdit.Prompt("sparkle> ";
        prompt_prefix = "\e[35m",
        prompt_suffix = "\e[0m",
        on_enter = REPL.return_callback)

    # Set up command processing with proper typing
    sparkle.on_done = (s::LineEdit.MIState, buf::IOBuffer, ok::Bool) -> begin
        if !ok
            LineEdit.transition(s, repl.interface.modes[1])
            return nothing
        end
        REPL.reset(repl)
        process_sparkle(s)
        REPL.prepare_next(repl)
        return nothing
    end

    # Add sparkle mode to REPL
    push!(repl.interface.modes, sparkle)

    # Get main mode
    main_mode = repl.interface.modes[1]

    # Add * key binding to main mode
    main_mode.keymap_dict = LineEdit.keymap_merge(
        main_mode.keymap_dict,
        Dict{Any,Any}(
            '*' => function (s,args...)
                buf = LineEdit.buffer(s)
                if position(buf) == 0
                    if !haskey(s.mode_state, sparkle)
                        s.mode_state[sparkle] = LineEdit.init_state(terminal, sparkle)
                    end
                    LineEdit.transition(s, sparkle)
                else
                    LineEdit.edit_insert(s, '*')
                end
            end
        )
    )
end

end # module Sparkle

# Initialize when REPL is ready
atreplinit() do repl
    @async begin
        sleep(0.1) # Wait for REPL to be fully initialized
        try
            Sparkle.init()
            println("\n✨ Welcome to Sparkle - Spark Runtime Terminal ✨")
            println("Press '*' to enter Sparkle mode, type '?' for help\n")
        catch e
            @warn "Failed to initialize Sparkle mode" exception=e
        end
    end
end
JULIA_EOF

# Show banner
cat << 'BANNER'
    ✨ 𝕊𝕡𝕒𝕣𝕜𝕝𝕖 ✨
    Spark Runtime Terminal
    Version 0.1-alpha
BANNER

# Start Julia REPL
julia --project="$SANDBOX_DIR" -i "$SANDBOX_DIR/init.jl"

# Cleanup
rm -rf "$SANDBOX_DIR"
