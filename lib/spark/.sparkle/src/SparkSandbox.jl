module SparkSandbox

using REPL
using REPL.LineEdit
using Statistics
using Dates
using TOML
using UnicodePlots
using ColorSchemes

include("Types.jl")
include("Crystal.jl")
include("SeedManager.jl")
include("REPL.jl")

# Re-export all public functions
export crystal, wave, weave, optimize, visualize
export seed_plant, seed_unplant, seed_garden, seed_sprout
export init_sparkle

end # module
