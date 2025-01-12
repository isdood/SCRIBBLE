# Three-Stage Boot Process
Last Updated: 2025-01-12 22:21:28 UTC
Maintainer: isdood

## Stages

1. **spinit** (Stage 1)
   - Initial boot sequence
   - Hardware initialization
   - Location: `boot/spinit/`

2. **spinup** (Stage 2)
   - Memory setup
   - UFO initialization
   - Location: `boot/spinup/`

3. **spun** (Stage 3)
   - Final kernel stage
   - System management
   - Location: `boot/spun/`

## Directory Structure

```bash
boot/
├── spinit/     # Stage 1
├── spinup/     # Stage 2
└── spun/       # Stage 3
