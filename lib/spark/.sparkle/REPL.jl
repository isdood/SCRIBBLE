# REPL mode implementation
using REPL
using REPL.LineEdit

export init_sparkle, process_sparkle

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
        return
    end

    if input == "?" || input == "help"
        println("""
        Sparkle Commands:
        ?/help                          - Show this help
        crystal([dims], [spacing])      - Create a new crystal structure
                                         dims: Tuple of 3 integers (default: (32,32,32))
                                         spacing: Float64 (default: 1.0)
        wave([n])                       - Create a new wave pattern
                                         n: Integer number of points (default: 100)
        weave([pattern])               - Apply weave pattern to current wave
                                         pattern: String (default: "default")
                                         Available patterns: $(join(keys(patterns), ", "))
        optimize                       - Optimize current structure
        visualize                      - Show current structures
        exit/quit                      - Exit Sparkle mode

        Seed Package Manager:
        seed ?                         - Show seed package manager help
        """)
    elseif input == "exit" || input == "quit"
        println("Exiting Sparkle mode...")
        LineEdit.transition(s, Base.active_repl.interface.modes[1])
    else
        try
            expr = Meta.parse(input)
            if expr isa Symbol
                expr = Expr(:call, expr)
            end
            result = Base.eval(Main, expr)
            if result !== nothing
                if !(result isa Union{Crystal,Wave}) # Only print if not already handled
                    println(result)
                end
            end
        catch e
            printstyled("Error: ", bold=true, color=:red)
            println(e)
        end
    end
end

function init_sparkle(repl)
    terminal = repl.t

    sparkle = LineEdit.Prompt("sparkle> ";
        prompt_prefix = "\e[35m",
        prompt_suffix = "\e[0m",
        on_enter = REPL.return_callback)

    sparkle.on_done = (s, buf, ok) -> begin
        if !ok
            LineEdit.transition(s, repl.interface.modes[1])
            return nothing
        end
        REPL.reset(repl)
        process_sparkle(s)
        REPL.prepare_next(repl)
        return nothing
    end

    push!(repl.interface.modes, sparkle)
    main_mode = repl.interface.modes[1]

    main_mode.keymap_dict = LineEdit.keymap_merge(
        main_mode.keymap_dict,
        Dict{Any,Any}(
            '*' => function (s,args...)
                buf = LineEdit.buffer(s)
                if position(buf) == 0
                    if !haskey(s.mode_state, sparkle)
                        s.mode_state[sparkle] = LineEdit.init_state(terminal, sparkle)
                    end
                    LineEdit.transition(s, sparkle)
                else
                    LineEdit.edit_insert(s, '*')
                end
            end
        )
    )
end
