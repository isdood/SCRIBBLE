#[cfg(test)]
mod tests {
    use harmony_core::*;

    #[test]
    fn test_crystal_resonance() {
        let mut core = HarmonyCore::new();
        let pattern = vec![65, 84, 67, 71]; // ATCG
        let result = core.weave_pattern(&pattern).unwrap();
        
        assert!(result.coherence > 0.9, "Crystal coherence too low");
        assert!(result.frequency >= 440.0, "Base frequency too low");
    }

    #[test]
    fn test_pattern_blending() {
        let mut core = HarmonyCore::new();
        let pattern1 = vec![65, 84]; // AT
        let pattern2 = vec![67, 71]; // CG
        
        let result1 = core.weave_pattern(&pattern1).unwrap();
        let result2 = core.weave_pattern(&pattern2).unwrap();
        
        let blended = core.blender.blend(result1, result2);
        assert!(blended.coherence > 0.95, "Blending reduced coherence too much");
    }
}
