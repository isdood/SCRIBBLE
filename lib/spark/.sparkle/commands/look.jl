using ..CommandHandler

# Import look module
include(joinpath(@__DIR__, "..", "std", "look", "init.jl"))
using .Look

# Register look command
register_command(
    "look",
    (args...) -> look(args...),
    () -> show_help()
)
