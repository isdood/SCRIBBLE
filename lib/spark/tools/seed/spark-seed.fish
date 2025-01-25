#!/usr/bin/env fish
set -l oldstate $fish_features
set fish_features glob
"$(dirname (status -f))/zig-out/bin/seed" $argv
set fish_features $oldstate
