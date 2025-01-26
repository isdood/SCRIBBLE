//! Addition Enchantments

use crate::types::*;

/// Specialized addition for each magical type
pub mod enchant {
    use super::*;

    #[inline]
    pub fn whispers(a: Whisper, b: Whisper) -> Whisper {
        a + b
    }

    #[inline]
    pub fn murmurs(a: Murmur, b: Murmur) -> Murmur {
        a + b
    }

    #[inline]
    pub fn voices(a: Voice, b: Voice) -> Voice {
        a + b
    }

    #[inline]
    pub fn shouts(a: Shout, b: Shout) -> Shout {
        a + b
    }

    #[inline]
    pub fn echoes(a: Echo, b: Echo) -> Echo {
        a + b
    }

    #[inline]
    pub fn thunders(a: Thunder, b: Thunder) -> Thunder {
        a + b
    }
}
