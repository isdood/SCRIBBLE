module Itex

export interactive_replace, prompt_edit, suggest_completion
export highlight_diff, apply_patch, revert_changes

using REPL
using Crayons

"""
Interactive text replacement with preview
"""
function interactive_replace(text::String, pattern::String)
    matches = findall(pattern, text)
    if isempty(matches)
        return text
    end

    result = text
    for match in matches
        before = result[1:prevind(result, first(match))]
        highlighted = crayon"red"(result[match])
        after = result[nextind(result, last(match)):end]

        println("\nFound: ", before, highlighted, after)
        print("Replace with (empty to skip): ")
        replacement = readline()

        if !isempty(replacement)
            result = before * replacement * after
        end
    end
    result
end

"""
Prompt for interactive text editing
"""
function prompt_edit(text::String)
    temp_file = tempname()
    write(temp_file, text)
    run(`$(ENV["EDITOR"]) $temp_file`)
    result = read(temp_file, String)
    rm(temp_file)
    result
end

"""
Suggest completions for text
"""
function suggest_completion(text::String, dictionary::Vector{String})
    matches = filter(w -> startswith(w, text), dictionary)
    isempty(matches) ? nothing : matches
end

"""
Highlight differences between two texts
"""
function highlight_diff(old::String, new::String)
    old_lines = split(old, '\n')
    new_lines = split(new, '\n')

    for (i, (old_line, new_line)) in enumerate(zip(old_lines, new_lines))
        if old_line != new_line
            println("Line $i:")
            println("- ", crayon"red"(old_line))
            println("+ ", crayon"green"(new_line))
        end
    end
end

"""
Apply patch to text
"""
function apply_patch(text::String, patch::String)
    # Simple patch format: @@ -start,length +start,length @@
    # Followed by context lines
    # TODO: Implement proper patch parsing
    text
end

"""
Revert last change
"""
function revert_changes(text::String, history::Vector{String})
    isempty(history) ? text : history[end]
end

end
