# Test Sparkle command loading
using Dates  # Add this import

println("🔍 Sparkle Test - $(Dates.now())")
println("Working directory: $(pwd())")
println("Loading paths:")
for path in LOAD_PATH
    println("  - $path")
end

# Examine Sparkle structure
sparkle_dir = joinpath(pwd(), ".sparkle")
println("\nSparkle directory structure:")
if isdir(sparkle_dir)
    for (root, dirs, files) in walkdir(sparkle_dir)
        rel_path = relpath(root, sparkle_dir)
        if rel_path == "."
            println("📁 .sparkle/")
        else
            println("📁 $(rel_path)/")
        end
        for file in files
            println("  └─ $file")
        end
    end
else
    println("❌ Sparkle directory not found!")
end

# Minimal command test
module TestLook
export look, handle_command

function look(args...)
    println("Look command called with args: ", args)
end

function handle_command(input::String)
    parts = split(strip(input))
    isempty(parts) && return false

    if parts[1] == "look"
        look(parts[2:end]...)
        return true
    end
    return false
end
end

# Load test command
using .TestLook

# Print module info
println("\nLoaded modules:")
for (key, mod) in Base.loaded_modules
    println("  $key => $mod")
end

# Test command handling
println("\nTesting command handling:")
handle_command("look")
handle_command("look -l")
handle_command("look --help")
