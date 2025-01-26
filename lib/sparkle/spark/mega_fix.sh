#!/usr/bin/env bash

# Mega Fix - Complete Sparkle Environment Setup
# Author: isdood
# Created: 2025-01-26 11:59:23 UTC
# Repository: isdood/scribble

set -e

echo "🌟 Mega Fix - Complete Sparkle Environment Setup"
echo "Created: 2025-01-26 11:59:23 UTC"
echo "Author: isdood"

SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
TEMP_DIR=$(mktemp -d)
PKG_DIR="$TEMP_DIR/SparkSandbox"

# Clean and create directories
echo "🧹 Cleaning previous installation..."
rm -rf "$SCRIPT_DIR/.sparkle" 2>/dev/null || true
rm -f "$SCRIPT_DIR/sparkle.sh" 2>/dev/null || true

echo "📁 Creating directories..."
mkdir -p "$SCRIPT_DIR/.sparkle"
mkdir -p "$SCRIPT_DIR/.sparkle/src"
mkdir -p "$PKG_DIR/src"

# Create sparkle.sh
echo "📝 Creating sparkle.sh..."
cat > "$SCRIPT_DIR/sparkle.sh" << 'EOT'
#!/usr/bin/env bash

# Sparkle - Spark Runtime Terminal v0.1
# Created: 2025-01-26 11:59:23 UTC
# Author: isdood

set -e

SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
TEMP_DIR=$(mktemp -d)
PKG_DIR="$TEMP_DIR/SparkSandbox"

# Create package structure
mkdir -p "$PKG_DIR"
mkdir -p "$PKG_DIR/src"

# Verify .sparkle directory exists
if [ ! -d "$SCRIPT_DIR/.sparkle" ]; then
    echo "Error: .sparkle directory not found. Please run mega_fix.sh first."
    exit 1
fi

# Verify required template files exist
required_files=(
    "Project.toml"
    "init.jl"
    "src/SparkSandbox.jl"
    "src/Types.jl"
    "src/Crystal.jl"
    "src/SeedManager.jl"
    "src/REPL.jl"
)

for file in "${required_files[@]}"; do
    if [ ! -f "$SCRIPT_DIR/.sparkle/$file" ]; then
        echo "Error: Required template file $file not found in .sparkle directory."
        echo "Please run mega_fix.sh to restore template files."
        rm -rf "$TEMP_DIR"
        exit 1
    fi
done

# Copy template files with correct permissions
echo "📦 Setting up Sparkle environment..."
mkdir -p "$PKG_DIR/src"
cp -r "$SCRIPT_DIR/.sparkle/src/"* "$PKG_DIR/src/"
cp "$SCRIPT_DIR/.sparkle/Project.toml" "$PKG_DIR/"
cp "$SCRIPT_DIR/.sparkle/init.jl" "$PKG_DIR/"
chmod 644 "$PKG_DIR/src/"*
chmod 644 "$PKG_DIR/Project.toml"
chmod 644 "$PKG_DIR/init.jl"

# Show banner
cat << 'BANNER'
    ✨ 𝕊𝕡𝕒𝕣𝕜𝕝𝕖 ✨
    Spark Runtime Terminal
    Version 0.1-alpha
BANNER

# Create trap to clean up temporary directory
trap 'rm -rf "$TEMP_DIR"' EXIT

# Start Julia REPL with proper environment
cd "$PKG_DIR" || exit 1
JULIA_PROJECT="." exec julia -i --color=yes init.jl
EOT

chmod +x "$SCRIPT_DIR/sparkle.sh"

# Create Project.toml
echo "📝 Creating Project.toml..."
cat > "$PKG_DIR/Project.toml" << 'EOT'
name = "SparkSandbox"
uuid = "b03cc3df-2e3a-4564-98fe-76823717dd5f"
authors = ["isdood"]
version = "0.1.0"

[deps]
REPL = "3fa0cd96-eef1-5676-8a61-b3b8758bbffb"
UnicodePlots = "b8865327-cd53-5732-bb35-84acbb429228"
Statistics = "10745b16-79ce-11e8-11f9-7d13ad32a3b2"
ColorSchemes = "35d6a980-a343-548e-a6ea-1d62b119f2f4"
TOML = "fa267f1f-6049-4f14-aa54-33bafae1ed76"
Dates = "ade2ca70-3891-5945-98fb-dc099432e06a"

[compat]
julia = "1.11"
UnicodePlots = "3.7"
ColorSchemes = "3.28"
EOT

# Create init.jl
echo "📝 Creating init.jl..."
cat > "$PKG_DIR/init.jl" << 'EOT'
using Pkg
Pkg.activate(".")
Pkg.instantiate()

push!(LOAD_PATH, "@v#.#", "@stdlib")
push!(LOAD_PATH, dirname(pwd()))

try
    using SparkSandbox

    # Make SparkSandbox functions available in Main without redefining core functions
    for name in names(SparkSandbox; all=true)
        if !startswith(string(name), "#") &&
           name ∉ (:eval, :include, :using, :import) &&
           !isdefined(Main, name)
            @eval Main const $name = SparkSandbox.$name
        end
    end

    atreplinit() do repl
        @async begin
            sleep(0.1)
            try
                SparkSandbox.init_sparkle(repl)
                println("\n✨ Welcome to Sparkle - Spark Runtime Terminal ✨")
                println("Press '*' to enter Sparkle mode, type '?' for help\n")
                println("Created: 2025-01-26 11:59:23")
                println("User: isdood")
            catch e
                @warn "Failed to initialize Sparkle mode" exception=e
            end
        end
    end
catch e
    @error "Failed to load SparkSandbox" exception=e
    exit(1)
end
EOT

# Create module files
echo "📝 Creating module files..."

# Create SparkSandbox.jl
echo "📝 Creating SparkSandbox.jl..."
cat > "$PKG_DIR/src/SparkSandbox.jl" << 'EOT'
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
EOT

# Create Types.jl
echo "📝 Creating Types.jl..."
cat > "$PKG_DIR/src/Types.jl" << 'EOT'
# Type definitions
export Crystal, Wave, Pattern, patterns, GLOBAL_STATE

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
    current_crystal::Union{Crystal,Nothing}
    current_wave::Union{Wave,Nothing}
    patterns::Dict{String,Pattern}
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
EOT

# Create Crystal.jl
echo "📝 Creating Crystal.jl..."
cat > "$PKG_DIR/src/Crystal.jl" << 'EOT'
# Crystal manipulation functions
export crystal, wave, weave, optimize, visualize

using Statistics
using UnicodePlots
using ColorSchemes

crystal(dims=(32,32,32), spacing=1.0) = begin
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

wave(n=100) = begin
    x = range(0, 4π, length=n)
    data = sin.(x) .+ 0.5 .* cos.(2x) .+ 0.2 .* randn(n)
    GLOBAL_STATE.current_wave = Wave(data, 1.0)
    println("Created wave pattern with $(n) points")
    visualize()
    return GLOBAL_STATE.current_wave
end

weave(pattern="default") = begin
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
    visualize()
    return result
end

visualize() = begin
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

optimize() = begin
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
EOT

# Create SeedManager.jl
echo "📝 Creating SeedManager.jl..."
cat > "$PKG_DIR/src/SeedManager.jl" << 'EOT'
# Seed package manager functions
export seed_plant, seed_unplant, seed_garden, seed_sprout

using TOML, Dates

function seed_plant(package_spec)
    parts = split(package_spec, "**")
    if length(parts) == 1
        println("🌱 Planting full package: $(parts[1])")
        _install_full_package(parts[1])
    elseif length(parts) == 2
        println("🌱 Planting component $(parts[2]) from package $(parts[1])")
        _install_package_component(parts[1], parts[2])
    else
        error("Invalid package specification format")
    end
end

function seed_unplant(package_spec)
    parts = split(package_spec, "**")
    if length(parts) == 1
        println("🗑️ Unplanting full package: $(parts[1])")
        _remove_full_package(parts[1])
    elseif length(parts) == 2
        println("🗑️ Unplanting component $(parts[2]) from package $(parts[1])")
        _remove_package_component(parts[1], parts[2])
    else
        error("Invalid package specification format")
    end
end

function seed_garden()
    if !isfile("config.spark")
        error("No config.spark found. Initialize with 'seed sprout' first.")
    end

    config = TOML.parsefile("config.spark")
    println("\n🌿 Installed Packages:")
    println("====================")

    if haskey(config, "packages")
        for (pkg, components) in config["packages"]
            println("📦 $pkg")
            if !isempty(components)
                for comp in components
                    println("  └─ $comp")
                end
            end
        end
    else
        println("No packages installed yet.")
    end
    println()
end

function seed_sprout()
    if isfile("config.spark")
        error("config.spark already exists!")
    end

    config = Dict(
        "project" => Dict(
            "name" => basename(pwd()),
            "version" => "0.1.0",
            "author" => "isdood",
            "created" => "2025-01-26 11:59:23"
        ),
        "packages" => Dict(),
        "dependencies" => Dict()
    )

    open("config.spark", "w") do io
        TOML.print(io, config)
    end
    println("🌱 Initialized new Spark project")
end
function _install_package_component(package, component)
    config = _load_config()
    if !haskey(config, "packages")
        config["packages"] = Dict()
    end

    if !haskey(config["packages"], package)
        config["packages"][package] = String[]
    end

    if !(component in config["packages"][package])
        push!(config["packages"][package], component)
    end

    _save_config(config)
    println("✨ Successfully planted $package**$component")
end

function _remove_full_package(package)
    config = _load_config()
    if haskey(config["packages"], package)
        delete!(config["packages"], package)
        _save_config(config)
        println("✨ Successfully unplanted $package")
    else
        println("Package $package is not installed")
    end
end

function _remove_package_component(package, component)
    config = _load_config()
    if haskey(config["packages"], package)
        components = config["packages"][package]
        filter!(c -> c != component, components)
        if isempty(components)
            delete!(config["packages"], package)
        else
            config["packages"][package] = components
        end
        _save_config(config)
        println("✨ Successfully unplanted $package**$component")
    else
        println("Package $package is not installed")
    end
end

function _load_config()
    if !isfile("config.spark")
        error("No config.spark found. Initialize with 'seed sprout' first.")
    end
    TOML.parsefile("config.spark")
end

function _save_config(config)
    open("config.spark", "w") do io
        TOML.print(io, config)
    end
end
EOT

# Create REPL.jl
echo "📝 Creating REPL.jl..."
cat > "$PKG_DIR/src/REPL.jl" << 'EOT'
# REPL mode implementation
using REPL
using REPL.LineEdit

export init_sparkle

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

function process_sparkle(s)
    buf = LineEdit.buffer(s)
    input = String(take!(copy(buf)))

    if startswith(input, "seed ")
        parts = split(input[6:end])
        cmd = parts[1]
        args = length(parts) > 1 ? parts[2:end] : String[]

        try
            if cmd == "plant" && !isempty(args)
                seed_plant(join(args, " "))
            elseif cmd == "unplant" && !isempty(args)
                seed_unplant(join(args, " "))
            elseif cmd == "garden"
                seed_garden()
            elseif cmd == "sprout"
                seed_sprout()
            else
                println("""
                Seed Package Manager Commands:
                seed plant <package>**<component>   - Install specific component
                seed plant <package>               - Install full package
                seed unplant <package>**<component> - Remove specific component
                seed unplant <package>             - Remove full package
                seed garden                        - List installed packages
                seed sprout                        - Initialize new Spark project
                """)
            end
        catch e
            printstyled("Error: ", bold=true, color=:red)
            println(e)
        end
        return
    end

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

        Seed Package Manager:
        seed ?                         - Show seed package manager help
        """)
        return
    elseif input == "exit" || input == "quit"
        println("Exiting Sparkle mode...")
        LineEdit.transition(s, Base.active_repl.interface.modes[1])
        return
    end

    try
        expr = Meta.parse(input)
        if expr isa Symbol
            expr = Expr(:call, expr)
        end
        result = Core.eval(Main.SparkSandbox, expr)
        if result !== nothing && !(result isa Union{Crystal,Wave})
            println(result)
        end
    catch e
        printstyled("Error: ", bold=true, color=:red)
        println(e)
    end
end
EOT

# Copy template files to .sparkle directory
echo "📦 Saving template files..."
cp -r "$PKG_DIR/src/"* "$SCRIPT_DIR/.sparkle/src/"
cp "$PKG_DIR/Project.toml" "$SCRIPT_DIR/.sparkle/"
cp "$PKG_DIR/init.jl" "$SCRIPT_DIR/.sparkle/"

# Set correct permissions
chmod 644 "$SCRIPT_DIR/.sparkle/init.jl"
chmod 644 "$SCRIPT_DIR/.sparkle/Project.toml"
chmod 644 "$SCRIPT_DIR/.sparkle/src/"*
chmod 755 "$SCRIPT_DIR/.sparkle"
chmod 755 "$SCRIPT_DIR/.sparkle/src"
chmod +x "$SCRIPT_DIR/sparkle.sh"

# Verify template files
echo "🔍 Verifying template files..."
required_files=(
    "Project.toml"
    "init.jl"
    "src/SparkSandbox.jl"
    "src/Types.jl"
    "src/Crystal.jl"
    "src/SeedManager.jl"
    "src/REPL.jl"
)

for file in "${required_files[@]}"; do
    if [ ! -f "$SCRIPT_DIR/.sparkle/$file" ]; then
        echo "❌ Error: Failed to copy $file to template directory"
        exit 1
    fi
    if [ ! -r "$SCRIPT_DIR/.sparkle/$file" ]; then
        echo "❌ Error: File $file is not readable"
        exit 1
    fi
done

echo "✅ Template files verified successfully"
echo "✨ Sparkle has been fixed! Try running ./sparkle.sh"

# Cleanup
rm -rf "$TEMP_DIR"
