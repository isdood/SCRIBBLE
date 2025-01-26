module History

export add_history, clear_history, get_history
export search_history, save_history, load_history

using Dates

const HISTORY_FILE = joinpath(homedir(), ".spark_history")
const MAX_HISTORY = 1000

mutable struct HistoryEntry
    timestamp::DateTime
    command::String
    status::Symbol  # :success or :error
end

global _history = HistoryEntry[]

"""
Add command to history
"""
function add_history(command::String, status::Symbol=:success)
    entry = HistoryEntry(now(), command, status)
    push!(_history, entry)
    if length(_history) > MAX_HISTORY
        popfirst!(_history)
    end
    entry
end

"""
Clear command history
"""
function clear_history()
    empty!(_history)
    nothing
end

"""
Get command history
"""
function get_history(; limit::Union{Int,Nothing}=nothing)
    isnothing(limit) ? _history : _history[end-min(limit,length(_history))+1:end]
end

"""
Search command history
"""
function search_history(pattern::String)
    filter(e -> occursin(pattern, e.command), _history)
end

"""
Save history to file
"""
function save_history(file::String=HISTORY_FILE)
    open(file, "w") do io
        for entry in _history
            println(io, "$(entry.timestamp)|$(entry.status)|$(entry.command)")
        end
    end
end

"""
Load history from file
"""
function load_history(file::String=HISTORY_FILE)
    clear_history()
    if isfile(file)
        for line in eachline(file)
            parts = split(line, "|", limit=3)
            if length(parts) == 3
                timestamp = DateTime(parts[1])
                status = Symbol(parts[2])
                command = parts[3]
                push!(_history, HistoryEntry(timestamp, command, status))
            end
        end
    end
    nothing
end

# Load history on module initialization
load_history()

end
