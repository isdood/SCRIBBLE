pub mod harmony;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_harmony_core_optimize() {
        let mut core = harmony::HarmonyCore::new();
        let initial_level = core.get_resonance_level();

        // Run multiple iterations to ensure stability
        for _ in 0..100 {
            core.optimize();
        }

        let final_level = core.get_resonance_level();
        assert!(final_level > initial_level,
            "Resonance level should increase from {} to {}",
            initial_level, final_level);

        // Test value bounds
        assert!(final_level.is_finite(), "Resonance level should remain finite");
        assert!(final_level > 0.0, "Resonance level should remain positive");
    }
}
