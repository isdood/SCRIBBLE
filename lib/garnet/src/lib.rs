#![cfg_attr(not(feature = "std"), no_std)]

//! Garnet - Terminal Emulator Library for Scribble OS
//!
//! This library provides terminal emulation capabilities for the Scribble operating system,
//! integrating with the crystal computing architecture and dream-space operations.

mod terminal;
mod vga;
mod input;
mod ansi;

pub use terminal::Terminal;
pub use input::InputHandler;
pub use ansi::AnsiParser;

/// Crystal-space terminal configuration
#[derive(Debug, Clone)]
pub struct GarnetConfig {
    /// Terminal width in characters
    pub width: u16,
    /// Terminal height in characters
    pub height: u16,
    /// Crystal resonance frequency for input handling
    pub crystal_freq: u32,
    /// Dream-space buffer size
    pub dream_buffer_size: usize,
}

impl Default for GarnetConfig {
    fn default() -> Self {
        Self {
            width: 80,
            height: 25,
            crystal_freq: 60,
            dream_buffer_size: 4096,
        }
    }
}

/// Result type for Garnet operations
pub type Result<T> = core::result::Result<T, Error>;

/// Error type for Garnet operations
#[derive(Debug)]
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
