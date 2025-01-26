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
