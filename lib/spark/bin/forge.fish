#!/usr/bin/env fish

# Spark's signature purple
set -g spark_purple '\033[0;35m'
set -g spark_reset '\033[0m'

function __spark_echo
    echo -e "$spark_purple✨ $argv$spark_reset"
end

function forge
    if test (count $argv) -lt 1
        echo "Usage: forge [brew|enchant|test] [project_directory]"
        return 1
    end

    set -l action $argv[1]
    set -l project_dir "."
    test (count $argv) -gt 1; and set project_dir $argv[2]
    set -l launch_file "$project_dir/launch.spk"
    
    switch $action
        case "brew"
            __spark_echo "Brewing project..."
            # Add compilation with feature handling
        case "enchant"
            __spark_echo "Enchanting project..."
            # Add execution with feature handling
        case "test"
            __spark_echo "Testing spells..."
            # Add testing with feature handling
        case '*'
            __spark_echo "Unknown forge command: $action"
            return 1
    end
end

complete -c forge -f -n "__fish_use_subcommand" -a "brew" -d "Build the project"
complete -c forge -f -n "__fish_use_subcommand" -a "enchant" -d "Run the project"
complete -c forge -f -n "__fish_use_subcommand" -a "test" -d "Run tests"
