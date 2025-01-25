module ModuleLoader
    using ..SparkRuntime
    using ..PathUtils

    export load_spark_module

    function load_spark_module(path::SparkPath)
        native_path = normalize_path(path)

        if !isfile(native_path)
            throw(ErrorException("Module not found: $native_path"))
        end

        SparkModule(
            name = last(path.components),
            path = path,
            loaded = true
        )
    end
end
