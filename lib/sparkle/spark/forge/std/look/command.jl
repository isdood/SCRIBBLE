using .Look: look

"""
Handle look command from Sparkle
"""
function handle_look_command(input::String)
    parts = split(strip(input))
    cmd = parts[1]
    args = length(parts) > 1 ? parts[2:end] : String[]

    if cmd == "look"
        Look.look(args)
    end
end
