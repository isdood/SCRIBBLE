#!/usr/bin/env fish

# Spark Seed Manager Fish Shell Wrapper
# Handles ** package syntax in fish shell

set SEED_BIN (dirname (status -f))/zig-out/bin/seed

function escape_package
    echo $argv[1] | string replace -a '**' '@@'
end

if test "$argv[1]" = "plant"; or test "$argv[1]" = "unplant"
    set command $argv[1]
    set package (escape_package "$argv[2]")
    set package (string replace -a '@@' '**' "$package")
    eval "$SEED_BIN" "$command" "$package"
else
    eval "$SEED_BIN" $argv
end
