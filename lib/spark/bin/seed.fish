function seed
    if test (count $argv) -lt 2
        echo "Usage: seed [plant|unplant] package**name"
        return 1
    end

    set -l action $argv[1]
    set -l package $argv[2]
    set -l package_clean (string replace --all '**' '/' $package)
    set -l package_path "$HOME/.spark/garden/$package_clean"
    
    switch $action
        case "plant"
            echo "ðŸŒ Planting $package..."
            if string match -q "std*" $package
                set -l std_name (string replace "std**" "" $package)
                set -l std_path "$SPARK_ROOT/std/$std_name"
                mkdir -p (dirname $package_path)
                ln -sf $std_path $package_path
            else
                mkdir -p $package_path
            end
        case "unplant"
            echo "ðŸ¥ Unplanting $package..."
            rm -rf $package_path
        case '*'
            echo "ðŸŒ Unknown magical command: $action"
            return 1
    end
end
