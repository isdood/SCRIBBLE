using .SeedManager

# Add seed command handling to process_sparkle
function process_sparkle(s)
    buf = LineEdit.buffer(s)
    input = String(take!(copy(buf)))

    if startswith(input, "seed ")
        parts = split(input[6:end])
        cmd = parts[1]
        args = length(parts) > 1 ? parts[2:end] : String[]

        try
            if cmd == "plant" && !isempty(args)
                seed_plant(join(args, " "))
            elseif cmd == "unplant" && !isempty(args)
                seed_unplant(join(args, " "))
            elseif cmd == "garden"
                seed_garden()
            elseif cmd == "sprout"
                seed_sprout()
            else
                println("""
                Seed Package Manager Commands:
                seed plant <package>**<component>   - Install specific component
                seed plant <package>               - Install full package
                seed unplant <package>**<component> - Remove specific component
                seed unplant <package>             - Remove full package
                seed garden                        - List installed packages
                seed sprout                        - Initialize new Spark project
                """)
            end
        catch e
            printstyled("Error: ", bold=true, color=:red)
            println(e)
        end
    else
        # ... rest of the existing command processing ...
    end
end
using .SeedManager

# Add seed command handling to process_sparkle
function process_sparkle(s)
    buf = LineEdit.buffer(s)
    input = String(take!(copy(buf)))

    if startswith(input, "seed ")
        parts = split(input[6:end])
        cmd = parts[1]
        args = length(parts) > 1 ? parts[2:end] : String[]

        try
            if cmd == "plant" && !isempty(args)
                seed_plant(join(args, " "))
            elseif cmd == "unplant" && !isempty(args)
                seed_unplant(join(args, " "))
            elseif cmd == "garden"
                seed_garden()
            elseif cmd == "sprout"
                seed_sprout()
            else
                println("""
                Seed Package Manager Commands:
                seed plant <package>**<component>   - Install specific component
                seed plant <package>               - Install full package
                seed unplant <package>**<component> - Remove specific component
                seed unplant <package>             - Remove full package
                seed garden                        - List installed packages
                seed sprout                        - Initialize new Spark project
                """)
            end
        catch e
            printstyled("Error: ", bold=true, color=:red)
            println(e)
        end
    else
        # ... rest of the existing command processing ...
    end
end
