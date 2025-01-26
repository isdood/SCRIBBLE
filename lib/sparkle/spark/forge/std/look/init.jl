module Look

export look, handle_look_command

using Printf
using Dates

"""
Parse command-line style arguments into look options
"""
function parse_look_args(args::Vector{String})
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

"""
Format file size with units
"""
function format_size(size::Integer, human::Bool)
    if !human
        return string(size)
    end
    for (unit, div) in [("T", 1024^4), ("G", 1024^3), ("M", 1024^2), ("K", 1024)]
        if size >= div
            return @sprintf("%.1f%s", size/div, unit)
        end
    end
    return string(size)
end

"""
Format file permissions
"""
function format_permissions(mode::UInt16)
    chars = ["r", "w", "x"]
    perms = ""
    for i in 8:-3:0
        for j in 2:-1:0
            perms *= (mode & (1 << (i+j))) != 0 ? chars[3-j] : "-"
        end
    end
    perms
end

"""
Get ANSI color code for entry type
"""
function get_entry_color(path::String)
    if isdir(path)
        "\e[1;34m"  # Blue for directories
    elseif islink(path)
        "\e[1;36m"  # Cyan for symlinks
    elseif isexecutable(path)
        "\e[1;32m"  # Green for executables
    else
        "\e[0m"     # Default for regular files
    end
end

"""
List directory contents with given options
"""
function list_directory(options::Dict{Symbol,Any})
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
            for entry in entries
                path = joinpath(options[:dir], entry)
                info = stat(path)
                size_str = format_size(info.size, options[:human])
                mtime_str = Dates.format(unix2datetime(info.mtime), "u-dd HH:MM")
                type_char = isdir(path) ? "d" : islink(path) ? "l" : "-"
                perms = format_permissions(info.mode)
                color = get_entry_color(path)
                println("$type_char$perms $(lpad(size_str, 8)) $mtime_str $color$entry\e[0m")
            end
        else
            if !isempty(entries)
                max_width = maximum(length.(entries)) + 2
                term_width = parse(Int, get(ENV, "COLUMNS", "80"))
                cols = max(1, div(term_width, max_width))

                for i in 1:length(entries)
                    entry = entries[i]
                    color = get_entry_color(joinpath(options[:dir], entry))
                    print(color, rpad(entry, max_width), "\e[0m")
                    if i % cols == 0 || i == length(entries)
                        println()
                    end
                end
            end
        end
    catch e
        println("Error: ", e)
    end
end

"""
Main entry point for look command
"""
function look(args::Vector{String}=String[])
    options = parse_look_args(args)
    if !isnothing(options)
        list_directory(options)
    end
end

"""
Alternative entry point for varargs
"""
function look(args::String...)
    look(collect(args))
end

"""
Handle look command from command line
"""
function handle_look_command(args::Vector{String})
    look(args)
end

end # module Look
