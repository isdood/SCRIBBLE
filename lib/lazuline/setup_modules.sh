#!/usr/bin/env bash

# Lazuline Setup Script
# Created: 2025-01-21 18:29:56
# Author: isdood

# Create directory structure
mkdir -p src/core src/interface src/bridge

# Create lib.rs
cat > src/lib.rs << 'END'
mod core;
mod interface;
mod bridge;

pub use crate::core::*;
pub use crate::interface::*;

pub fn init() -> Result<Lazuline, Error> {
    Lazuline::new()
}

pub mod prelude {
    pub use crate::core::{Error, Lazuline};
    pub use crate::init;
}
END

# Create core/mod.rs
cat > src/core/mod.rs << 'END'
use std::error::Error as StdError;

#[derive(Debug)]
pub struct Lazuline {
    initialized: bool,
}

impl Lazuline {
    pub fn new() -> Result<Self, Box<dyn StdError>> {
        Ok(Self {
            initialized: true,
        })
    }
}

pub type Error = Box<dyn StdError>;
END

# Create interface/mod.rs
cat > src/interface/mod.rs << 'END'
pub trait LanguageInterface {
    fn initialize(&mut self) -> crate::core::Error;
}
END

# Create bridge/mod.rs
cat > src/bridge/mod.rs << 'END'
use crate::interface::LanguageInterface;
END

echo "✨ Lazuline module structure created!"
