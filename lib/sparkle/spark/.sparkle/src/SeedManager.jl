# Seed package manager functions
export seed_plant, seed_unplant, seed_garden, seed_sprout

using TOML, Dates

const STD_PACKAGES = Dict{String,Vector{String}}(
    "std" => ["look", "prism"]  # Simplified for testing
)

"""
Create the initial config.spark file
"""
function seed_sprout()
    config = Dict(
        "project" => Dict(
            "name" => "SparkSandbox",
            "version" => "0.1.0",
            "author" => "isdood",
            "created" => Dates.format(now(), "yyyy-mm-dd HH:MM:SS")
        ),
        "packages" => Dict{String,Vector{String}}()
    )

    open("config.spark", "w") do io
        TOML.print(io, config)
    end
    println("🌱 Initialized new Spark project")
end

"""
Install a package or component
"""
function seed_plant(package_spec)
    parts = split(package_spec, "**")

    if length(parts) == 2
        package, component = parts
        if !haskey(STD_PACKAGES, package)
            error("Package $package not found")
        end
        if !(component in STD_PACKAGES[package])
            error("Component $component not found in package $package")
        end

        println("🌱 Planting component $component from package $package")

        # Load or create config
        if !isfile("config.spark")
            seed_sprout()
        end

        config = TOML.parsefile("config.spark")
        if !haskey(config, "packages")
            config["packages"] = Dict{String,Vector{String}}()
        end
        if !haskey(config["packages"], package)
            config["packages"][package] = String[]
        end

        # Add component if not already present
        if !(component in config["packages"][package])
            push!(config["packages"][package], component)
        end

        # Save config
        open("config.spark", "w") do io
            TOML.print(io, config)
        end

        # Create component directory and module
        std_dir = joinpath("std")
        comp_dir = joinpath(std_dir, component)
        mkpath(comp_dir)

        # Write module file
        open(joinpath(comp_dir, "init.jl"), "w") do f
            if component == "look"
                write(f, """
                module Look
                export look
                function look(args...)
                    entries = readdir(".")
                    if isempty(entries)
                        println("(empty directory)")
                    else
                        for entry in sort(entries)
                            if isdir(entry)
                                printstyled(entry, "/\\n", color=:blue)
                            else
                                println(entry)
                            end
                        end
                    end
                end
                end # module Look
                """)
            end
        end

        println("✨ Successfully planted $package**$component")

        # Reload components
        @eval Main.SparkSandbox begin
            load_components()
        end
    else
        error("Invalid package specification. Use format: package**component")
    end
end

"""
List installed packages
"""
function seed_garden()
    if !isfile("config.spark")
        println("No packages installed (no config.spark found)")
        return
    end

    config = TOML.parsefile("config.spark")
    println("\n🌿 Installed Packages:")

    if haskey(config, "packages")
        for (pkg, components) in config["packages"]
            println("📦 $pkg")
            for comp in components
                println("  └─ $comp")
            end
        end
    else
        println("No packages installed")
    end
    println()
end

"""
Remove a package or component
"""
function seed_unplant(package_spec)
    if !isfile("config.spark")
        error("No config.spark found")
    end

    parts = split(package_spec, "**")
    config = TOML.parsefile("config.spark")

    if length(parts) == 2
        package, component = parts
        if haskey(config["packages"], package)
            filter!(c -> c != component, config["packages"][package])
            if isempty(config["packages"][package])
                delete!(config["packages"], package)
            end
            open("config.spark", "w") do io
                TOML.print(io, config)
            end
            println("✨ Successfully unplanted $package**$component")
        end
    else
        error("Invalid package specification. Use format: package**component")
    end
end
