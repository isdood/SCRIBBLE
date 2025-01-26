# Sparkle Mode Handler
using .SparkSandbox

function handle_sparkle_command(input::String)
    parts = split(strip(input))
    if isempty(parts)
        return
    end

    cmd = parts[1]
    args = length(parts) > 1 ? parts[2:end] : String[]

    if cmd == "?"
        show_help()
    else
        execute_command(cmd, args)
    end
end

function show_help()
    println("Available commands:")
    for cmd in list_commands()
        help_fn = get_command_help(cmd)
        if help_fn !== nothing
            help_text = help_fn()
            println("\n", help_text)
        end
    end
end
