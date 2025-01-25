#!/usr/bin/env bash

# Sparkle - Spark Runtime Terminal v0.1
# Author: isdood
# Created: 2025-01-25 23:05:31 UTC
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

using REPL
using REPL.LineEdit

# Process sparkle commands
function process_sparkle(s::LineEdit.MIState)
    buf = LineEdit.buffer(s)
    input = String(take!(copy(buf))::Vector{UInt8})
    if input == "?" || input == "help"
        println("""
        Sparkle Commands:
        ?/help      - Show this help
        crystal     - Create a new crystal structure
        wave       - Create a new wave pattern
        weave      - Apply weave pattern
        optimize   - Optimize current structure
        exit/quit  - Exit Sparkle
        """)
    elseif input == "exit" || input == "quit"
        println("Exiting Sparkle mode...")
        LineEdit.transition(s, Base.active_repl.interface.modes[1])
    else
        try
            expr = Meta.parse(input)
            result = Core.eval(Main, expr)
            println(result)
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
