//! Spark's Magical Type System

/// Whisper - A tiny number (8-bit integer)
pub type Whisper = i8;

/// Murmur - A small number (16-bit integer)
pub type Murmur = i16;

/// Voice - A regular number (32-bit integer)
pub type Voice = i32;

/// Shout - A big number (64-bit integer)
pub type Shout = i64;

/// Echo - A floating number (32-bit float)
pub type Echo = f32;

/// Thunder - A precise floating number (64-bit float)
pub type Thunder = f64;

/// Scroll - A string of characters
pub type Scroll = String;

/// Rune - A single character
pub type Rune = char;

/// Essence - A basic true/false value
pub type Essence = bool;

/// Void - Represents nothing (unit type)
pub type Void = ();

/// Size of magical containers
pub type Magnitude = usize;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MagicKind {
    Whisper,
    Murmur,
    Voice,
    Shout,
    Echo,
    Thunder,
    Scroll,
    Rune,
    Essence,
    Void,
}

impl std::fmt::Display for MagicKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MagicKind::Whisper => write!(f, "Whisper"),
            MagicKind::Murmur => write!(f, "Murmur"),
            MagicKind::Voice => write!(f, "Voice"),
            MagicKind::Shout => write!(f, "Shout"),
            MagicKind::Echo => write!(f, "Echo"),
            MagicKind::Thunder => write!(f, "Thunder"),
            MagicKind::Scroll => write!(f, "Scroll"),
            MagicKind::Rune => write!(f, "Rune"),
            MagicKind::Essence => write!(f, "Essence"),
            MagicKind::Void => write!(f, "Void"),
        }
    }
}
