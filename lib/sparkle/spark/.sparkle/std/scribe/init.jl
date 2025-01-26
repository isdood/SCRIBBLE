module Scribe

export log_entry, annotate, transcribe
export create_journal, read_journal, search_journal

using Dates

"""
Journal entry structure
"""
struct JournalEntry
    timestamp::DateTime
    level::Symbol
    message::String
    metadata::Dict{String,Any}
end

"""
Create a new log entry
"""
function log_entry(message::String, level::Symbol=:info; metadata::Dict{String,Any}=Dict())
    entry = JournalEntry(now(), level, message, metadata)
    _write_to_journal(entry)
    entry
end

"""
Add annotation to existing entry
"""
function annotate(entry::JournalEntry, note::String)
    metadata = copy(entry.metadata)
    if haskey(metadata, "annotations")
        push!(metadata["annotations"], note)
    else
        metadata["annotations"] = [note]
    end
    JournalEntry(entry.timestamp, entry.level, entry.message, metadata)
end

"""
Transcribe data to structured format
"""
function transcribe(data; format=:json)
    if format == :json
        JSON.json(data)
    elseif format == :toml
        sprint(io -> TOML.print(io, data))
    else
        string(data)
    end
end

"""
Create new journal file
"""
function create_journal(name::String)
    journal_path = joinpath(homedir(), ".spark_journals", name)
    mkpath(dirname(journal_path))
    touch(journal_path)
    journal_path
end

"""
Read journal entries
"""
function read_journal(name::String; filter_fn::Function=entry->true)
    journal_path = joinpath(homedir(), ".spark_journals", name)
    entries = JournalEntry[]
    if isfile(journal_path)
        for line in eachline(journal_path)
            entry = JSON.parse(line)
            journal_entry = JournalEntry(
                DateTime(entry["timestamp"]),
                Symbol(entry["level"]),
                entry["message"],
                entry["metadata"]
            )
            if filter_fn(journal_entry)
                push!(entries, journal_entry)
            end
        end
    end
    entries
end

"""
Search journal entries
"""
function search_journal(name::String, pattern::String)
    read_journal(name, filter_fn=entry->occursin(pattern, entry.message))
end

"""
Internal: Write entry to journal
"""
function _write_to_journal(entry::JournalEntry)
    journal_dir = joinpath(homedir(), ".spark_journals")
    mkpath(journal_dir)
    journal_path = joinpath(journal_dir, "default.journal")

    entry_data = Dict(
        "timestamp" => string(entry.timestamp),
        "level" => string(entry.level),
        "message" => entry.message,
        "metadata" => entry.metadata
    )

    open(journal_path, "a") do io
        println(io, JSON.json(entry_data))
    end
end

end
