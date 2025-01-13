# Scribble Project
Last Updated: 2025-01-13 03:42:37 UTC
Author: isdood
Current User: isdood

## Overview
Scribble is a memory management and alignment toolkit designed for low-level systems programming. It provides safe abstractions for memory operations while maintaining precise control over memory layout and alignment.

## Project Structure

### 1. Unstable Matter (`lib/unstable_matter/`)
Core library providing fundamental types and operations:
- `Vector3D`: 3D vector operations
- `UFO`: Memory tracking and protection
- `SpaceTime`: Memory space-time operations
- `MorphTracker`: File type morphing and tracking
- `Alignment`: Memory alignment utilities (formerly in Scribble)

Key Features:
- No-std compatible
- Atomic operations for thread safety
- Memory protection through UFO tracking
- Precise timestamp tracking for all operations

### 2. Scribble Core (`lib/scribble/`)
High-level memory management toolkit:
- `MemorySpace`: Safe memory region management
- `ScribbleMemory`: Direct memory access with safety checks
- `MemoryAddress`: Type-safe memory address representation
- `Dimensions`: 3D space dimensioning

Key Features:
- Built on top of Unstable Matter
- Memory alignment and padding
- Thread-safe operations
- Dimensional memory layouts

### 3. 3-Stage Bootloader (`lib/bootloader/`)
Bootloader stages providing system initialization:
- `spINIT`: Initial system setup
- `spinUP`: Bringing up essential services
- `spun`: Finalizing the boot process

### 4. Auth System Freezer (`lib/auth/`)
Authentication system for secure access:
- User authentication and session management
- Secure token generation and validation

### 5. splatNstat System (`lib/splatnstat/`)
System for logging and statistics:
- Event logging and tracking
- Real-time statistics and monitoring

## Replacements and Migrations

### Recent Changes
- Alignment module moved from Scribble to Unstable Matter
- UFO tracking integrated into all memory operations
- Timestamp tracking added to all operations

### Replacements
Original Component   | New Component                         | Location
---------------------|----------------------------------------------------------------------------
`align::Align`       | `unstable_matter::align::Align`       | `lib/unstable_matter/src/align.rs`
`ScribbleAlign`      | `unstable_matter::align::AlignMarker` | `lib/unstable_matter/src/align.rs`
Raw pointers         | `MemoryAddress`                       | `lib/scribble/src/lib.rs`
Direct memory access | `ScribbleMemory`                      | `lib/scribble/src/lib.rs`

## Usage

### Memory Operations
```rust
let mem = ScribbleMemory::<u32>::at(0x1000);
unsafe {
    mem.write(42);
    let value = mem.read();
}
