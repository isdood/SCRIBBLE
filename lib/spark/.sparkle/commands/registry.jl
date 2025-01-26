# Command Registry
const COMMANDS = Dict{String, Function}()
const COMMAND_HELP = Dict{String, Function}()

function register_command(name::String, fn::Function, help_fn::Function)
    global COMMANDS[name] = fn
    global COMMAND_HELP[name] = help_fn
end

function get_command(name::String)
    get(COMMANDS, name, nothing)
end

function get_command_help(name::String)
    get(COMMAND_HELP, name, nothing)
end

function list_commands()
    sort!(collect(keys(COMMANDS)))
end
