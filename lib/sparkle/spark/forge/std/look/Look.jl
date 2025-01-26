module Look

export look, handle_command

using Printf
using Dates

# Main command function
function look(args...)
    handle_command(string("look ", join(args, " ")))
end

# Command handler
function handle_command(input::String)
    parts = split(strip(input))
    isempty(parts) && return false

    if parts[1] != "look"
        return false
    end

    args = length(parts) > 1 ? parts[2:end] : String[]

    # Parse options
    options = Dict{Symbol,Any}(
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
            return true
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
    return true
end

# Rest of the implementation...
include("look_impl.jl")

end # module
