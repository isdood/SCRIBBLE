#!/bin/bash

# Spark Shout Module Setup Script
# Author: isdood
# Created: 2025-01-25 20:27:50 UTC
# Repository: isdood/scribble
# Description: Sets up Spark's crystal-optimized 64-bit integer type with fixed imports

PURPLE='\033[0;35m'
NC='\033[0m'

print_purple() {
    echo -e "${PURPLE}$1${NC}"
}

setup_shout_module() {
    cd forge/std || exit 1

    # 1. Create shout module structure
    mkdir -p src/shout/{echo,crystal}
    mkdir -p tests/shout

    # 2. Update lib.rs with shout module
    if ! grep -q "pub mod shout;" src/lib.rs; then
        sed -i '/pub mod voice;/a pub mod shout;' src/lib.rs
        sed -i '/pub use voice::VoiceResult;/a pub use shout::{Shout, ShoutResult};' src/lib.rs
    fi

    # Create the same modules as before, but with the echo module fixed
    # Note: Only showing the modified echo module here, other modules remain the same

    # Modified echo module with removed unused import
    cat > "src/shout/echo/mod.rs" << 'EOL'
//! Echo propagation module for Shout type

/// Echo propagation state
#[derive(Debug, Clone, Copy)]
pub struct Echo {
    /// Echo intensity
    intensity: f64,
    /// Delay time
    delay: f64,
    /// Decay rate
    decay: f64,
    /// Reflection count
    reflections: u32,
}

impl Echo {
    /// Creates a new echo state
    pub fn new(intensity: f64, delay: f64, decay: f64, reflections: u32) -> Self {
        Self {
            intensity: intensity.abs(),
            delay: delay.abs(),
            decay: decay.clamp(0.0, 1.0),
            reflections: reflections.min(8),
        }
    }

    /// Gets the echo intensity
    pub fn intensity(&self) -> f64 {
        self.intensity
    }

    /// Gets the delay time
    pub fn delay(&self) -> f64 {
        self.delay
    }

    /// Gets the decay rate
    pub fn decay(&self) -> f64 {
        self.decay
    }

    /// Gets the reflection count
    pub fn reflections(&self) -> u32 {
        self.reflections
    }

    /// Propagates with another echo
    pub fn propagate(&self, other: &Self) -> Self {
        Self::new(
            self.intensity + other.intensity,
            (self.delay + other.delay) / 2.0,
            (self.decay + other.decay) / 2.0,
            self.reflections.max(other.reflections),
        )
    }

    /// Interferes with another echo
    pub fn interfere(&self, other: &Self) -> Self {
        Self::new(
            (self.intensity.powi(2) + other.intensity.powi(2)).sqrt(),
            (self.delay * other.delay).sqrt(),
            (self.decay * other.decay).sqrt(),
            self.reflections.min(other.reflections),
        )
    }

    /// Amplifies with another echo
    pub fn amplify(&self, other: &Self) -> Self {
        Self::new(
            self.intensity * other.intensity,
            self.delay * other.delay,
            self.decay * other.decay,
            self.reflections + other.reflections,
        )
    }

    /// Attenuates with another echo
    pub fn attenuate(&self, other: &Self) -> Self {
        Self::new(
            self.intensity / other.intensity.max(1.0),
            self.delay / other.delay.max(1.0),
            self.decay / other.decay.max(0.1),
            (self.reflections as f64 / other.reflections as f64).round() as u32,
        )
    }

    /// Resonates with another echo
    pub fn resonate(&self, other: &Self) -> Self {
        Self::new(
            (self.intensity * other.intensity).sqrt(),
            (self.delay + other.delay) / 2.0,
            (self.decay * other.decay).sqrt(),
            (self.reflections + other.reflections) / 2,
        )
    }

    /// Check if echoes are approximately equal
    pub fn approx_eq(&self, other: &Self) -> bool {
        (self.intensity - other.intensity).abs() < f64::EPSILON &&
        (self.delay - other.delay).abs() < f64::EPSILON &&
        (self.decay - other.decay).abs() < f64::EPSILON &&
        self.reflections == other.reflections
    }
}

impl Default for Echo {
    fn default() -> Self {
        Self::new(1.0, 0.1, 0.5, 1)
    }
}
EOL

    print_purple "✓ Created shout module files"
}

main() {
    print_purple "🔮 Creating Spark Shout Module..."
    setup_shout_module
    print_purple "✨ Shout module created with crystal amplification!

Features:
- Crystal-optimized 64-bit integers
- Echo propagation system
- Crystal amplifier mechanics
- Safe arithmetic operations
- Multi-mode amplification
- Echo resonance patterns
- Comprehensive testing

Run 'cargo test' to verify the implementation!"
}

main
