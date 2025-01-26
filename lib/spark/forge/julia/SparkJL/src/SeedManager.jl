module SeedManager

export seed_plant, seed_unplant, seed_garden, seed_sprout

@seeds@
using TOML
using JSON
@seeds@

@spell@
function seed_plant(package_spec)
    parts = split(package_spec, "**")
    if length(parts) == 1
        # Installing full package
        println("🌱 Planting full package: $(parts[1])")
        _install_full_package(parts[1])
    elseif length(parts) == 2
        # Installing specific component
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
            "author" => "$(ENV["USER"])",
            "created" => "$(now())"
        ),
        "packages" => Dict(),
        "dependencies" => Dict()
    )

    open("config.spark", "w") do io
        TOML.print(io, config)
    end
    println("🌱 Initialized new Spark project")
end

# Internal helper functions
function _install_full_package(package)
    config = _load_config()
    if !haskey(config, "packages")
        config["packages"] = Dict()
    end

    if !haskey(config["packages"], package)
        config["packages"][package] = String[]
    end

    _save_config(config)
    println("✨ Successfully planted $package")
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
@spell@

end # module SeedManager
