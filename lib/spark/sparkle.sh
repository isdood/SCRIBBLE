#!/usr/bin/env bash

# Sparkle - Spark Runtime Terminal v0.1
# Author: isdood
# Created: 2025-01-25 23:37:31 UTC
# Repository: isdood/scribble

set -e

SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"

# Generate a temporary directory for our sandbox
SANDBOX_DIR=$(mktemp -d)
PKG_DIR="$SANDBOX_DIR/SparkSandbox"
mkdir -p "$PKG_DIR/src"

# Create spark config
cat > "$PKG_DIR/spark.conf" << 'SPARK_EOF'
~weave~
~forge~

@seeds@
REPL = "3fa0cd96-eef1-5676-8a61-b3b8758bbffb"
UnicodePlots = "b8865327-cd53-5732-bb35-84acbb429228"
Statistics = "10745b16-79ce-11e8-11f9-7d13ad32a3b2"
ColorSchemes = "35d6a980-a343-548e-a6ea-1d62b119f2f4"
@seeds@
SPARK_EOF

# Create Project.toml
UUID=$(julia -e 'import UUIDs; println(UUIDs.uuid4())')
cat > "$PKG_DIR/Project.toml" << EOF
name = "SparkSandbox"
uuid = "$UUID"
version = "0.1.0"

[deps]
REPL = "3fa0cd96-eef1-5676-8a61-b3b8758bbffb"
UnicodePlots = "b8865327-cd53-5732-bb35-84acbb429228"
Statistics = "10745b16-79ce-11e8-11f9-7d13ad32a3b2"
ColorSchemes = "35d6a980-a343-548e-a6ea-1d62b119f2f4"
EOF

# Create src/SparkSandbox.jl
cat > "$PKG_DIR/src/SparkSandbox.jl" << 'JULIA_EOF'
module SparkSandbox

using REPL
using REPL.LineEdit
using UnicodePlots
using Statistics
using ColorSchemes

# Export all the functions we want to make available
export crystal, wave, weave, optimize, visualize

# Sparkle types
struct Crystal
    dimensions
    spacing
    data
end

struct Wave
    data
    frequency
end

struct Pattern
    name
    transform
end

# Global state
const patterns = Dict{String,Pattern}()
mutable struct SparkleState
    current_crystal
    current_wave
    patterns
end

const GLOBAL_STATE = SparkleState(nothing, nothing, patterns)

# Register default patterns
patterns["default"] = Pattern("Default", w -> w)
patterns["invert"] = Pattern("Invert", w -> Wave(-w.data, w.frequency))
patterns["double"] = Pattern("Double", w -> Wave(w.data .* 2, w.frequency))
patterns["smooth"] = Pattern("Smooth", w -> begin
    data = copy(w.data)
    for i in 2:length(data)-1
        data[i] = mean(w.data[i-1:i+1])
    end
    Wave(data, w.frequency)
end)

function crystal(dims=(32,32,32), spacing=1.0)
    data = zeros(dims...)
    center = dims .÷ 2
    for i in 1:dims[1], j in 1:dims[2], k in 1:dims[3]
        r = sqrt(((i-center[1])/dims[1])^2 + ((j-center[2])/dims[2])^2 + ((k-center[3])/dims[3])^2)
        data[i,j,k] = exp(-r^2 * 5)
    end

    GLOBAL_STATE.current_crystal = Crystal(dims, spacing, data)
    println("Created crystal structure with dimensions $(dims) and spacing $(spacing)")
    visualize()
    return GLOBAL_STATE.current_crystal
end

function wave(n=100)
    x = range(0, 4π, length=n)
    data = sin.(x) .+ 0.5 .* cos.(2x) .+ 0.2 .* randn(n)
    GLOBAL_STATE.current_wave = Wave(data, 1.0)
    println("Created wave pattern with $(n) points")
    visualize()
    return GLOBAL_STATE.current_wave
end

function weave(pattern="default")
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
    GLOBAL_STATE.current_wave = result
    println("Pattern applied successfully")
    visualize()
    return result
end

function visualize()
    if !isnothing(GLOBAL_STATE.current_crystal)
        crystal = GLOBAL_STATE.current_crystal
        middle_slice = crystal.data[:,:,crystal.dimensions[3]÷2]
        println("\nCrystal Visualization (middle slice):")
        display(heatmap(middle_slice, colormap=:viridis))
    end

    if !isnothing(GLOBAL_STATE.current_wave)
        wave = GLOBAL_STATE.current_wave
        n = length(wave.data)
        println("\nWave Visualization:")
        display(lineplot(1:n, wave.data, title="Wave Pattern", name="amplitude"))
    end
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
        println("  - Mean density: $(mean(GLOBAL_STATE.current_crystal.data))")
        println("  ✓ Crystal optimization complete")
    end

    if !isnothing(GLOBAL_STATE.current_wave)
        println("• Wave optimization:")
        println("  - Points: $(length(GLOBAL_STATE.current_wave.data))")
        println("  - Frequency: $(GLOBAL_STATE.current_wave.frequency)")
        println("  - Amplitude range: [$(minimum(GLOBAL_STATE.current_wave.data)), $(maximum(GLOBAL_STATE.current_wave.data))]")
        println("  ✓ Wave optimization complete")
    end

    visualize()
    return (crystal=GLOBAL_STATE.current_crystal, wave=GLOBAL_STATE.current_wave)
end

# REPL mode handling
function process_sparkle(s)
    buf = LineEdit.buffer(s)
    input = String(take!(copy(buf)))

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
        visualize                      - Show current structures
        exit/quit                      - Exit Sparkle mode
        """)
    elseif input == "exit" || input == "quit"
        println("Exiting Sparkle mode...")
        LineEdit.transition(s, Base.active_repl.interface.modes[1])
    else
        try
            expr = Meta.parse(input)
            if expr isa Symbol
                expr = Expr(:call, expr)
            end
            result = Base.eval(SparkSandbox, expr)
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

# Initialize REPL mode
function init_sparkle(repl)
    terminal = repl.t

    sparkle = LineEdit.Prompt("sparkle> ";
        prompt_prefix = "\e[35m",
        prompt_suffix = "\e[0m",
        on_enter = REPL.return_callback)

    sparkle.on_done = (s, buf, ok) -> begin
        if !ok
            LineEdit.transition(s, repl.interface.modes[1])
            return nothing
        end
        REPL.reset(repl)
        process_sparkle(s)
        REPL.prepare_next(repl)
        return nothing
    end

    push!(repl.interface.modes, sparkle)
    main_mode = repl.interface.modes[1]

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

end # module SparkSandbox
JULIA_EOF

# Create startup script
cat > "$SANDBOX_DIR/init.jl" << 'INIT_EOF'
push!(LOAD_PATH, dirname(pwd()))
using SparkSandbox

atreplinit() do repl
    @async begin
        sleep(0.1)
        try
            SparkSandbox.init_sparkle(repl)
            println("\n✨ Welcome to Sparkle - Spark Runtime Terminal ✨")
            println("Press '*' to enter Sparkle mode, type '?' for help\n")
        catch e
            @warn "Failed to initialize Sparkle mode" exception=e
        end
    end
end
INIT_EOF

# Install required packages
cd "$PKG_DIR" && julia --project=. -e '
    using Pkg
    Pkg.instantiate()
    Pkg.add(["UnicodePlots", "Statistics", "ColorSchemes"])
'

# Show banner
cat << 'BANNER'
    ✨ 𝕊𝕡𝕒𝕣𝕜𝕝𝕖 ✨
    Spark Runtime Terminal
    Version 0.1-alpha
BANNER

# Start Julia REPL with proper environment
cd "$SANDBOX_DIR" && julia --project="$PKG_DIR" -i init.jl

# Cleanup
rm -rf "$SANDBOX_DIR"
