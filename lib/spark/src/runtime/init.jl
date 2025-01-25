# Spark Runtime Initialization
using Pkg

# Activate the project environment
Pkg.activate(".")

# Ensure required packages are installed
required_packages = [
    "DataStructures",
    "Logging",
    "Test"
]

for pkg in required_packages
    if !haskey(Pkg.project().dependencies, pkg)
        Pkg.add(pkg)
    end
end

# Load runtime modules
include("SparkRuntime.jl")
using .SparkRuntime

# Initialize runtime and run basic validation
print_purple = s -> println("\e[35m$s\e[0m")
print_purple("🔮 Spark Runtime v$(SparkRuntime.VERSION) initializing...")
test_path = SparkPath("std**math**add")
print_purple("✨ Runtime initialized with native ** operator support")
println("Test path components: ", test_path.components)
