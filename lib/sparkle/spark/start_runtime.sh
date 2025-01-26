# Core runtime module
cat > src/runtime/SparkRuntime.jl << 'EOL'
module SparkRuntime
    export SparkModule, SparkPath, resolve_path, parse_spark_path, normalize_path, validate_path

    using Base: @kwdef

    # Version and metadata
    const VERSION = v"0.1.0"
    const SPARK_PATH_SEPARATOR = "**"
    const INVALID_PATH_CHARS = ['/', '\\', '*']

    # Custom path type to handle Spark's ** operator
    struct SparkPath
        components::Vector{String}
    end

    # Basic constructors
    SparkPath(path::String) = SparkPath(split(path, SPARK_PATH_SEPARATOR))
    SparkPath(components::Vector{String}) = SparkPath(components)

    # Module representation
    @kwdef mutable struct SparkModule
        name::String
        path::SparkPath
        loaded::Bool = false
        exports::Set{Symbol} = Set{Symbol}()
        dependencies::Vector{SparkPath} = SparkPath[]
    end

    # Module registry
    struct ModuleRegistry
        modules::Dict{String, SparkModule}
        ModuleRegistry() = new(Dict{String, SparkModule}())
    end

    # Global registry instance
    const GLOBAL_REGISTRY = ModuleRegistry()

    # Path utilities
    function normalize_path(path::SparkPath)::String
        @static if Sys.iswindows()
            replace(join(path.components, "\\"), "/" => "\\")
        else
            join(path.components, "/")
        end
    end

    # Fixed validate_path function using proper string operations
    function validate_path(path_str::String)::Bool
        components = split(path_str, SPARK_PATH_SEPARATOR)
        for component in components
            if isempty(component)
                return false
            end
            for char in INVALID_PATH_CHARS
                if occursin(string(char), component)
                    return false
                end
            end
        end
        return true
    end

    # Path resolution
    function resolve_path(path::SparkPath)::String
        joinpath(path.components...)
    end

    # Module loading
    function load_module(path::SparkPath)::SparkModule
        native_path = resolve_path(path)
        @info "Loading module from: $native_path"
        module_name = last(path.components)
        SparkModule(name=module_name, path=path)
    end

    # Path parsing
    function parse_spark_path(path_str::String)::SparkPath
        SparkPath(path_str)
    end

    # Path joining
    function join_spark_paths(base::SparkPath, sub::SparkPath)::SparkPath
        SparkPath([base.components..., sub.components...])
    end

    # Double star operator handling
    function handle_double_star_operator(left::String, right::String)
        SparkPath([left, right])
    end

    # Module import
    function import_module(path_str::String)
        path = parse_spark_path(path_str)
        if !haskey(GLOBAL_REGISTRY.modules, path_str)
            GLOBAL_REGISTRY.modules[path_str] = load_module(path)
        end
        GLOBAL_REGISTRY.modules[path_str]
    end

    Base.joinpath(p::SparkPath, s::String) = join_spark_paths(p, SparkPath(s))
end
EOL
