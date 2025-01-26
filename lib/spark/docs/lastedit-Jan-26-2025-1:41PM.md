# Spark Toolchain for Scribble

> Part of the Scribble Framework: A Crystal-based high-performance computing framework written in Rust & Zig.

## Overview

Spark is a specialized toolchain for managing and executing Crystal spells within the Scribble framework. It provides safety checks, runtime verification, and standardized execution patterns for Crystal-based computations.

## Project Structure

```
scribble/lib/spark/
├── config.spark           # Project configuration
├── launch.spk            # Main build file
├── examples/
│   └── hello_world/
│       └── hello_world.spk  # Example spell
├── install_spark.sh      # Initial installation script
├── fix_spark.sh         # Maintenance/repair script
└── spark                # Main command-line tool
```

## File Format Specifications

### config.spark
```
~forge~ = calm

@seeds@
name = "project_name"
version = "0.1.0"
author = "author_name"
created = "timestamp"

[crystal]
version = "1.9.0"
safety = true
threads = 4

[spark]
format = 2
spells = ["core", "safety", "crystal"]
@seeds@
```

### Spell Files (.spk)
Spell files use a three-level bracket hierarchy:
- { } for spell definitions
- [ ] for blocks and arrays
- ( ) for function calls and parameters

Example:
```
~forge~ = calm

@seeds@
name = "spell_name"
version = "0.1.0"
@seeds@

@spells@
import core::io
import crystal::runtime

spell SpellName {
    init() [
        io::println["Message"]
        
        crystal::eval("""
            Crystal code here
        """)
    ]
    
    cast() [
        self::init[]
        return 0
    ]
}
@spells@
```

## Command-Line Interface

The `spark` command provides several operations:

```bash
spark launch            # Cast main spell
spark launch <spell>    # Cast specific spell (e.g., hello_world)
spark spells           # List available spells
spark verify           # Verify forge safety levels
spark help             # Show this help
```

### Safety Verification

Spark performs multiple safety checks:
1. Forge safety level verification (`~forge~ = calm`)
2. Bracket hierarchy analysis
3. Configuration validation
4. Runtime safety checks

## Installation

1. Run the installation script:
```bash
./install_spark.sh
```

2. Verify the installation:
```bash
spark verify
```

3. Test with hello_world:
```bash
spark launch hello_world
```

## Development Status (as of 2025-01-26 18:41:08)

Current features:
- [x] Forge safety level verification
- [x] Three-level bracket hierarchy support
- [x] Crystal runtime integration
- [x] Colorized output
- [x] Configuration validation
- [x] Example spell (hello_world)
- [x] Installation and repair scripts

## Output Format

The toolchain uses color-coded output:
- 💜 Purple (✨): Spark system messages
- 🔶 Orange (⚡): Warning/error messages
- 🔷 Blue (🔨): Forge/compilation messages

Example output:
```
🔨 forge: Validating project configuration...
🔨 forge: Analyzing forge safety level in config.spark
🔨 forge: Found safety level: calm in config.spark
✨ Launching Spark spell for hello_world
🔨 forge: === Begin Forge Safety Analysis ===
✨ Hello, World!
🔨 forge: Compiling with safety level: calm
```

## Maintenance

If you encounter issues with the toolchain, use the fix script:
```bash
./fix_spark.sh
```

This script will:
1. Restore proper validation chains
2. Update output formatting
3. Fix spell execution flow
4. Repair bracket analysis

## Authors

- **isdood** - *Initial work* - [GitHub](https://github.com/isdood)

## License

This project is part of the Scribble framework. See the LICENSE file for details.

---
Last updated: 2025-01-26 18:41:08 UTC by isdood
