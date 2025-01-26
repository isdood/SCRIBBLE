"""
Look module - Directory listing utility
"""
module Look

export look

using Printf
using Dates

function look(args...)
    # Convert all arguments to strings
    str_args = String[string(arg) for arg in args]

    # Parse options
    dir = "."
    all = false
    long = false
    human = true
    reverse = false
    sort_by = :name

    for arg in str_args
        if arg == "--help"
            show_help()
            return
        elseif arg == "-a" || arg == "--all"
            all = true
        elseif arg == "-l" || arg == "--long"
            long = true
        elseif arg == "-b" || arg == "--bytes"
            human = false
        elseif arg == "-r" || arg == "--reverse"
            reverse = true
        elseif startswith(arg, "--sort=")
            try
                sort_by = Symbol(split(arg, "=")[2])
            catch
                sort_by = :name
            end
        elseif !startswith(arg, "-")
            dir = arg
        end
    end

    # List directory contents
    try
        entries = readdir(dir)

        if !all
            entries = filter(x -> !startswith(x, '.'), entries)
        end

        if sort_by == :size
            sort!(entries, by=x->stat(joinpath(dir, x)).size, rev=!reverse)
        elseif sort_by == :modified
            sort!(entries, by=x->stat(joinpath(dir, x)).mtime, rev=!reverse)
        else
            sort!(entries, rev=reverse)
        end

        if long
            for entry in entries
                path = joinpath(dir, entry)
                info = stat(path)
                size_str = human ? format_size(info.size) : string(info.size)
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
                    color = get_entry_color(joinpath(dir, entry))
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

# Helper functions
function format_size(size::Integer)
    for (unit, div) in [("T", 1024^4), ("G", 1024^3), ("M", 1024^2), ("K", 1024)]
        if size >= div
            return @sprintf("%.1f%s", size/div, unit)
        end
    end
    return string(size)
end

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

end # module Look
