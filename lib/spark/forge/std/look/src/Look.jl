module Look

export look, look_impl

using Printf
using Dates

"""
Convert command-line style arguments to options dictionary
"""
function parse_look_args(args::Vector{String})
    options = Dict(
        :dir => ".",
        :all => false,
        :long => false,
        :human => true,
        :reverse => false,
        :sort_by => :name
    )

    for arg in args
        if arg == "--help"
            show_help()
            return nothing
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

    return options
end

"""
Show help message
"""
function show_help()
    println("""
    look [options] [directory]
    List directory contents in a pretty format

    Options:
      -a, --all      Show hidden files
      -l, --long     Use long listing format
      -b, --bytes    Show sizes in bytes
      -r, --reverse  Reverse sort order
      --sort=TYPE    Sort by: name, size, modified
      --help        Show this help message
    """)
end

# Helper functions
include("utils.jl")

"""
Main look implementation
"""
function look_impl(options::Dict{Symbol,Any})
    try
        entries = readdir(options[:dir])

        if !options[:all]
            entries = filter(x -> !startswith(x, '.'), entries)
        end

        if options[:sort_by] == :size
            sort!(entries, by=x->stat(joinpath(options[:dir], x)).size, rev=!options[:reverse])
        elseif options[:sort_by] == :modified
            sort!(entries, by=x->stat(joinpath(options[:dir], x)).mtime, rev=!options[:reverse])
        else
            sort!(entries, rev=options[:reverse])
        end

        if options[:long]
            show_long_format(options[:dir], entries, options[:human])
        else
            show_short_format(options[:dir], entries)
        end
    catch e
        println("Error: ", e)
    end
end

"""
Main entry point for both command-line and function calls
"""
function look(args::Vector{String}=String[])
    options = parse_look_args(args)
    isnothing(options) && return
    look_impl(options)
end

"""
Allow varargs calls like look("a", "b", "c")
"""
function look(args::String...)
    look(collect(args))
end

end # module Look
