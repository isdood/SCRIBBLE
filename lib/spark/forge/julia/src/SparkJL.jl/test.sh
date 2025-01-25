#!/bin/bash
SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
julia --project="$SCRIPT_DIR" -e 'using Pkg; Pkg.test()'
