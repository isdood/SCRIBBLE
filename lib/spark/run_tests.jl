using Pkg

# Activate the project
Pkg.activate(".")

# Run the tests
include("tests/runtime/runtests.jl")
