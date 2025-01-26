module Sparkle

using .CommandHandler
using .Look

export look, execute_command

include(joinpath(@__DIR__, "core", "CommandHandler.jl"))

# Load all commands
for cmd_file in readdir(joinpath(@__DIR__, "commands"))
    if endswith(cmd_file, ".jl")
        include(joinpath(@__DIR__, "commands", cmd_file))
    end
end

end
