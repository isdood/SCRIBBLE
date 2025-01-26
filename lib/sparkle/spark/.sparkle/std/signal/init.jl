module Signal

export on, off, emit, once
export SignalHandler, connect, disconnect
export @signal, @handler

using Base.Threads

"""
Signal handler type
"""
mutable struct SignalHandler
    name::Symbol
    handlers::Vector{Function}
    once::Vector{Bool}
    mutex::ReentrantLock
end

const SIGNAL_HANDLERS = Dict{Symbol, SignalHandler}()
const HANDLER_MUTEX = ReentrantLock()

"""
Create or get signal handler
"""
function get_handler(name::Symbol)
    lock(HANDLER_MUTEX) do
        if !haskey(SIGNAL_HANDLERS, name)
            SIGNAL_HANDLERS[name] = SignalHandler(name, Function[], Bool[], ReentrantLock())
        end
        SIGNAL_HANDLERS[name]
    end
end

"""
Register event handler
"""
function on(signal::Symbol, handler::Function; once::Bool=false)
    h = get_handler(signal)
    lock(h.mutex) do
        push!(h.handlers, handler)
        push!(h.once, once)
    end
    nothing
end

"""
Remove event handler
"""
function off(signal::Symbol, handler::Function)
    h = get_handler(signal)
    lock(h.mutex) do
        idx = findfirst(==(handler), h.handlers)
        if !isnothing(idx)
            deleteat!(h.handlers, idx)
            deleteat!(h.once, idx)
        end
    end
    nothing
end

"""
Emit signal
"""
function emit(signal::Symbol, args...)
    h = get_handler(signal)
    to_remove = Int[]

    lock(h.mutex) do
        for (i, (handler, once)) in enumerate(zip(h.handlers, h.once))
            @async handler(args...)
            if once
                push!(to_remove, i)
            end
        end

        # Remove one-time handlers
        deleteat!(h.handlers, to_remove)
        deleteat!(h.once, to_remove)
    end
    nothing
end

"""
Register one-time event handler
"""
function once(signal::Symbol, handler::Function)
    on(signal, handler, once=true)
end

"""
Connect multiple signals
"""
function connect(connections::Dict{Symbol,Function})
    for (signal, handler) in connections
        on(signal, handler)
    end
end

"""
Disconnect multiple signals
"""
function disconnect(connections::Dict{Symbol,Function})
    for (signal, handler) in connections
        off(signal, handler)
    end
end

# Macros for signal handling
macro signal(name)
    quote
        const $(esc(name)) = $(QuoteNode(name))
    end
end

macro handler(signal, expr)
    quote
        on($(esc(signal)), $(esc(expr)))
    end
end

end
