module Look

export look

using Printf
using Dates

"""
List directory contents with optional formatting
"""
function look(args::String...)
    parse_and_execute(collect(args))
end

"""
List directory contents (no arguments)
"""
function look()
    parse_and_execute(String[])
end

"""
Parse and execute look command
"""
function parse_and_execute(args::Vector{String})
    # Default options
    options = Dict(
        :dir => ".",
        :all => false,
        :long => false,
        :human => true,
        :reverse => false,
        :sort_by => :name
    )

    # Parse arguments
    for arg in args
        if arg == "--help"
            return show_help()
        elseif arg == "-a" || arg == "--all"
            options[:all] = true
        elseif arg == "-l" || arg == "--long"
            options[:long] = true
        elseif arg == "-b" || arg == "--bytes"
            options[:human] = false
        elseif arg == "-r" || arg == "--reverse"
            options[:reverse] = true
        elseif startswith(arg, "--sort=")
            try
                options[:sort_by] = Symbol(split(arg, "=")[2])
            catch
                options[:sort_by] = :name
            end
        elseif !startswith(arg, "-")
            options[:dir] = arg
        end
    end

    list_directory(options)
end

include("utils.jl")

end # module Look
