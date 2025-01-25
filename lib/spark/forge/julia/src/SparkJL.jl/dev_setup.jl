using Pkg

# Activate the package directory
Pkg.activate(".")

# Add dependencies one by one with specific versions
deps = [
    ("CUDA", "5.0"),
    ("DataStructures", "0.18"),
    ("SIMD", "3.4")
]

for (dep, ver) in deps
    try
        Pkg.add(PackageSpec(name=dep, version=ver))
        @info "Added $dep version $ver"
    catch e
        @error "Failed to add dependency $dep" version=ver exception=e
    end
end

# Develop the package
try
    Pkg.develop(PackageSpec(path="."))
    @info "Package development setup complete"
catch e
    @error "Failed to develop package" exception=e
    exit(1)
end
