module SparkSandbox

export look

include(joinpath(@__DIR__, "core", "CommandHandler.jl"))
include(joinpath(@__DIR__, "std", "look", "init.jl"))

# Make Look module available
using .Look
using .CommandHandler

# Load all commands
for cmd_file in readdir(joinpath(@__DIR__, "commands"))
    if endswith(cmd_file, ".jl")
        include(joinpath(@__DIR__, "commands", cmd_file))
    end
end

end
