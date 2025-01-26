module Look

export look

"""
List directory contents
"""
function look(args...)
    entries = readdir(".")
    if isempty(entries)
        println("(empty directory)")
    else
        for entry in sort(entries)
            if isdir(entry)
                printstyled(entry, "/\n", color=:blue)
            else
                println(entry)
            end
        end
    end
end

end # module Look
