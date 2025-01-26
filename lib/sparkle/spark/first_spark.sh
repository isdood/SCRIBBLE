#!/bin/bash

# First Spark Project Setup
# Author: isdood
# Created: 2025-01-26 15:52:40 UTC

set -euo pipefail
IFS=$'\n\t'

SPARK_PURPLE='\033[0;35m'
SPARK_ORANGE='\033[0;33m'
NC='\033[0m'

print_spark() {
    echo -e "${SPARK_PURPLE}✨ $1${NC}"
}

print_glitch() {
    echo -e "${SPARK_ORANGE}⚡ $1${NC}"
}

# Create project structure
mkdir -p .spark/{cache,lib,examples}
mkdir -p examples/hello_world

# Create config.spark
cat > config.spark << 'EOL'
@seeds@
name = "scribble"
version = "0.1.0"
author = "isdood"
created = "2025-01-26 15:52:40"

[crystal]
version = "1.9.0"
safety = true
threads = 4

[spark]
format = 2
spells = ["core", "safety", "crystal"]
@seeds@
EOL

# Create launch.spk build file
cat > launch.spk << 'EOL'
@spells@
import core::build
import core::config
import crystal::runtime

spell BuildConfig {
    target = "native"
    optimize = true
    parallel = true
}

spell Runner {
    init() {
        build::set_config(BuildConfig)
        crystal::init_runtime()
    }

    run() {
        build::compile("examples")
    }
}
@spells@
EOL

# Create hello_world.spk example
cat > examples/hello_world/hello_world.spk << 'EOL'
@seeds@
name = "hello_world"
version = "0.1.0"
@seeds@

@spells@
import core::io
import crystal::runtime

spell HelloWorld {
    init() {
        io::println("Hello from Spark!")

        crystal::eval("""
            puts "Hello from Crystal!"
            x = 21
            result = x * 2
            puts "Crystal computed: #{result}"
        """)

        let result = crystal::get_int("result")
        io::println("Spark received: {result}")
    }
}

spell Main {
    run() {
        HelloWorld::init()
    }
}
@spells@
EOL

# Create example config
cat > examples/hello_world/launch.spk << 'EOL'
@seeds@
name = "hello_world_example"
type = "binary"
@seeds@

@spells@
import core::build

spell ExampleConfig {
    src = "hello_world.spk"
    deps = ["core::io", "crystal::runtime"]
}

spell Builder {
    init() {
        build::configure(ExampleConfig)
    }
}
@spells@
EOL

print_spark "Created Spark project structure with proper @seeds@ and @spells@ syntax:"
echo "
├── config.spark           # Project configuration
├── launch.spk            # Main build file
├── .spark/
│   ├── cache/           # Build cache
│   └── lib/             # Dependencies
└── examples/
    └── hello_world/
        ├── hello_world.spk  # Example code
        └── launch.spk       # Example config
"

print_spark "
To build and run:
1. spark build              # Build the project
2. spark run hello_world    # Run the example"

chmod +x launch.spk
