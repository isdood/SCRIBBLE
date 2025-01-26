module Echo

export echo, echo_nl, echo_err, echo_fmt
export echo_debug, echo_info, echo_warn

using Dates

"""
Echo a message with optional formatting
"""
function echo(msg...; color=:normal)
    printstyled(join(msg, " "), color=color)
end

"""
Echo a message with newline
"""
function echo_nl(msg...; color=:normal)
    printstyled(join(msg, " "), "\n", color=color)
end

"""
Echo to stderr
"""
function echo_err(msg...)
    printstyled(stderr, join(msg, " "), "\n", color=:red)
end

"""
Echo with printf-style formatting
"""
function echo_fmt(fmt::String, args...; color=:normal)
    printstyled(@sprintf(fmt, args...), color=color)
end

"""
Echo debug message with timestamp
"""
function echo_debug(msg...)
    printstyled("[DEBUG ", now(), "] ", color=:cyan)
    println(join(msg, " "))
end

"""
Echo info message with timestamp
"""
function echo_info(msg...)
    printstyled("[INFO ", now(), "] ", color=:blue)
    println(join(msg, " "))
end

"""
Echo warning message with timestamp
"""
function echo_warn(msg...)
    printstyled("[WARN ", now(), "] ", color=:yellow)
    println(join(msg, " "))
end

end
