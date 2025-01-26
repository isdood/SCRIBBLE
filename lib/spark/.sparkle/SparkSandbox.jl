module SparkSandbox

using REPL
using REPL.LineEdit
using Statistics
using Dates
using TOML
using UnicodePlots
using ColorSchemes

# Types and state
include("Types.jl")
# Core functionality
include("Crystal.jl")
include("SeedManager.jl")
# REPL interface
include("REPL.jl")

# Export interface
export crystal, wave, weave, optimize, visualize
export seed_plant, seed_unplant, seed_garden, seed_sprout
export init_sparkle

end # module
