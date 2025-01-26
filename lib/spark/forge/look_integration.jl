include(joinpath(@__DIR__, "..", "..", "forge", "std", "look", "init.jl"))
using .Look

function handle_command(input::String)
    parts = split(strip(input))
    isempty(parts) && return

    cmd = parts[1]
    args = length(parts) > 1 ? parts[2:end] : String[]

    if cmd == "look"
        Look.handle_look_command(args)
    end
end
