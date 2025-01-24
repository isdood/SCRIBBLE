//! Lazuline - Crystal-based parallel computing framework
//!
//! This library provides tools for crystal-based computation patterns.

pub mod harmony;
pub mod crystal;
pub mod whimsy;

pub use harmony::HarmonyField;
pub use crystal::CrystalBridge;
pub use whimsy::WhimsyEngine;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_harmony_enhancement() {
        let mut field = HarmonyField::new(1.0);
        assert!(field.enhance_harmony().is_ok());
    }

    #[test]
    fn test_crystal_bridge() {
        let bridge = CrystalBridge::new();
        assert!(bridge.get_harmony() > 0.0);
    }

    #[test]
    fn test_whimsy_engine() {
        let engine = WhimsyEngine::new(42);
        assert_eq!(engine.get_level(), 42);
    }
}
