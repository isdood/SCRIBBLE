function forge
    if test (count $argv) -lt 1
        echo "Usage: forge [brew|enchant|test] [project_directory]"
        return 1
    end

    set -l action $argv[1]
    set -l project_dir "."
    test (count $argv) -gt 1; and set project_dir $argv[2]
    set -l launch_file "$project_dir/launch.spk"
    
    if not test -f $launch_file
        echo "‚ùå No launch.spk found in $project_dir"
        return 1
    end
    
    switch $action
        case "brew"
            echo "üî Brewing project..."
            eval "$SPARK_CORE/compiler/forge build $launch_file"
        case "enchant"
            echo "‚ú® Enchanting project..."
            eval "$SPARK_CORE/runtime/spark run $launch_file"
        case "test"
            echo "üé Testing spells..."
            eval "$SPARK_CORE/test/runner $launch_file"
        case '*'
            echo "Unknown forge command: $action"
            return 1
    end
end
