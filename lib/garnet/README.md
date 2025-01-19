# 🌟 Garnet
## Crystal-Integrated Terminal Emulation System

```ascii
┌──────────────────────┐
│    Dream Buffer      │
├──────────────────────┤
│ Crystal Resonance    │
├──────────────────────┤
│ Terminal Emulation   │
└──────────────────────┘
```

Garnet is a sophisticated terminal emulation library designed specifically for the Scribble operating system. It integrates seamlessly with crystal computing architecture and dream-space operations to provide a unique and efficient terminal experience.

## ✨ Features

### Crystal Integration
- Crystal resonance-based input handling
- Dream-space buffer management
- No-std compatible
- VGA buffer integration

### Terminal Capabilities
- ANSI escape sequence support
- Configurable dimensions
- Crystal-space color management
- Dream buffer integration

## 🚀 Quick Start

```rust
use garnet::{Terminal, GarnetConfig, Result};

fn main() -> Result<()> {
    // Create terminal configuration
    let config = GarnetConfig {
        width: 80,
        height: 25,
        crystal_freq: 60,
        dream_buffer_size: 4096,
    };

    // Initialize terminal
    let mut terminal = Terminal::new(config)?;
    
    // Write to terminal
    terminal.write_char('H')?;
    terminal.write_char('i')?;
    
    Ok(())
}
```

## 📊 Configuration

```rust
pub struct GarnetConfig {
    /// Terminal width in characters
    width: u16,
    /// Terminal height in characters
    height: u16,
    /// Crystal resonance frequency (Hz)
    crystal_freq: u32,
    /// Dream-space buffer size
    dream_buffer_size: usize,
}
```

### Default Configuration
```rust
GarnetConfig {
    width: 80,
    height: 25,
    crystal_freq: 60,
    dream_buffer_size: 4096,
}
```

## 🎯 Core Modules

### 1. Terminal (`terminal.rs`)
- Character writing
- Cursor management
- Screen buffering
- Scrolling operations

### 2. VGA (`vga.rs`)
- Color management
- Buffer operations
- Character attributes

### 3. Input (`input.rs`)
- Crystal resonance input handling
- Dream-space event processing
- Input buffer management

### 4. ANSI (`ansi.rs`)
- ANSI escape sequence parsing
- Terminal control sequences
- Color code handling

## 💫 Terminal Operations

### Character Operations
```rust
// Write a character
terminal.write_char('A')?;

// Handle special characters
terminal.write_char('\n')?; // New line
terminal.write_char('\r')?; // Carriage return
terminal.write_char('\t')?; // Tab
```

### Screen Management
```rust
// Clear screen
terminal.clear();

// Scroll operations
terminal.scroll_up();
```

## ⚡ Performance Characteristics

- Crystal Resonance: 60Hz default
- Buffer Access: O(1)
- Screen Scrolling: O(n) where n is line size
- Dream Buffer: 4KB default

## 🛠️ Requirements

### System Requirements
- Rust (no_std compatible)
- Crystal computing architecture
- VGA-compatible display
- Dream-space support

### Optional Features
```toml
[features]
std = []  # Enable standard library support
```

## 🔬 Error Handling

```rust
pub enum Error {
    /// Invalid terminal dimensions
    InvalidDimensions,
    /// Buffer overflow
    BufferOverflow,
    /// Crystal resonance error
    CrystalResonanceError,
    /// Input handling error
    InputError,
    /// VGA buffer error
    VgaError,
}
```

## 📈 Buffer Layout

### Screen Buffer
```
[char][attr][char][attr]...
  │     │     │     │
  │     │     │     └─ Character 2 attributes
  │     │     └─────── Character 2
  │     └─────────────Character 1 attributes
  └─────────────────── Character 1
```

### Attributes
```
7 6 5 4 3 2 1 0
│ │ │ │ │ │ │ │
│ │ │ └─┴─┴─┴─┘
│ │ │     │
│ └─┴─────┴───── Background color (4 bits)
└───────────────Foreground color (4 bits)
```

## 🤝 Contributing

1. Maintain crystal resonance requirements
2. Test with various terminal dimensions
3. Verify dream-space compatibility
4. Update documentation
5. Add tests for new features

## 📊 Current Status
- Version: 0.1.0
- Last Updated: 2025-01-19 23:57:42 UTC
- Implementation: Rust (no_std)
- Author: Caleb J.D. Terkovics (isdood)

## 📜 License
MIT - See LICENSE for details

---

*"The terminal is not just a window to computation, but a crystal lens into dream-space."* - isdood
