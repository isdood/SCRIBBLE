module PathUtils
    using ..SparkRuntime: SparkPath, SPARK_PATH_SEPARATOR

    export normalize_path, validate_path

    function normalize_path(path::SparkPath)::String
        @static if Sys.iswindows()
            replace(join(path.components, "\\"), "/" => "\\")
        else
            join(path.components, "/")
        end
    end

    function validate_path(path_str::String)::Bool
        components = split(path_str, SPARK_PATH_SEPARATOR)
        all(comp -> !isempty(comp) && !contains(comp, ['/', '\\', '*']), components)
    end
end
