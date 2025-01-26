module SparkSandbox

using REPL
using REPL.LineEdit
using Statistics
using Dates
using TOML
using UnicodePlots
using ColorSchemes
using LinearAlgebra

# Get the absolute path to .sparkle directory
const SPARKLE_ROOT = dirname(dirname(@__FILE__))

# Include core types and modules
include("Types.jl")
include("Crystal.jl")
include("SeedManager.jl")
include("REPL.jl")

# Function to load a component
function load_component(package::String, component::String)
    println("📦 Loading component $component from package $package...")

    # Use absolute path to .sparkle/std
    path = joinpath(SPARKLE_ROOT, "std", component, "init.jl")
    println("📂 Looking for component at: $path")

    if !isfile(path)
        println("❌ Component file not found: $path")
        return false
    end

    # Include the module
    try
        include(path)
        println("✨ Successfully loaded $component")

        # If it's a Look module, make functions available globally
        if component == "look"
            @eval import .Look: look
            @eval export look
        end

        # If it's a Prism module, make functions available globally
        if component == "prism"
            @eval import .Prism: init_prism, mount_prism, unmount_prism
            @eval export init_prism, mount_prism, unmount_prism
        end

        return true
    catch e
        println("❌ Error loading component $component: ", e)
        return false
    end
end

# Load all components from config.spark
function load_components()
    println("\n🔄 Loading components...")

    if !isfile("config.spark")
        println("❌ No config.spark found")
        return
    end

    config = TOML.parsefile("config.spark")
    if !haskey(config, "packages")
        println("❌ No packages section in config.spark")
        return
    end

    for (pkg, components) in config["packages"]
        for comp in components
            load_component(pkg, comp)
        end
    end

    println("✨ Component loading complete\n")
end

# Initialize
function __init__()
    @async begin
        sleep(0.1)  # Wait for REPL to initialize
        load_components()
    end
end

# Re-export all public functions
export crystal, wave, weave, optimize, visualize
export seed_plant, seed_unplant, seed_garden, seed_sprout
export init_sparkle

# Export module loading
export load_component, load_components

end # module
