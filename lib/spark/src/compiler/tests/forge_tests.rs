#[cfg(test)]
mod tests {
    use super::super::forge::{ForgeFeatures, SafetyLevel};

    #[test]
    fn test_safety_levels() {
        let source = r#"
            ~forge~ = calm
            
            @spells@
            pub fn safe_fn() -> i32 { 1 }
            
            ~forge~ = wild
            pub fn unsafe_fn() -> i32 { 2 }
            @spells@
        "#;

        let mut features = ForgeFeatures::new("test.spk");
        features.parse_features(source).unwrap();

        assert_eq!(features.get_safety_level("safe_fn"), SafetyLevel::Calm);
        assert_eq!(features.get_safety_level("unsafe_fn"), SafetyLevel::Wild);
    }

    #[test]
    fn test_invalid_safety_level() {
        let source = "~forge~ = invalid";
        let mut features = ForgeFeatures::new("test.spk");
        assert!(features.parse_features(source).is_err());
    }
}
