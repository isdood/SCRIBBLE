# Utility functions for Look module

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

function show_long_format(dir::String, entries::Vector{String}, human::Bool)
    for entry in entries
        path = joinpath(dir, entry)
        info = stat(path)
        size_str = format_size(info.size, human)
        mtime_str = Dates.format(unix2datetime(info.mtime), "u-dd HH:MM")
        type_char = isdir(path) ? "d" : islink(path) ? "l" : "-"
        perms = format_permissions(info.mode)
        color = get_entry_color(path)
        println("$type_char$perms $(lpad(size_str, 8)) $mtime_str $color$entry\e[0m")
    end
end

function show_short_format(dir::String, entries::Vector{String})
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
