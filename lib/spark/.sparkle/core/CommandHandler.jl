module CommandHandler

export register_command, execute_command, get_command_help, list_commands

const COMMANDS = Dict{String, Function}()
const COMMAND_HELP = Dict{String, Function}()

function register_command(name::String, fn::Function, help_fn::Function)
    global COMMANDS[name] = fn
    global COMMAND_HELP[name] = help_fn
end

function execute_command(cmd::String, args::Vector{String})
    if haskey(COMMANDS, cmd)
        try
            COMMANDS[cmd](args...)
        catch e
            println("Error executing command '$cmd': ", e)
            println(stacktrace(catch_backtrace()))
        end
    else
        println("Unknown command: $cmd")
        println("Type '?' for help")
    end
end

function get_command_help(name::String)
    get(COMMAND_HELP, name, nothing)
end

function list_commands()
    sort!(collect(keys(COMMANDS)))
end

end
