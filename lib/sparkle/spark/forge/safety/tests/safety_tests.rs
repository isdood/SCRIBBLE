use spark_safety::{SafetyChecker, SafetyLevel};

#[test]
fn test_calm_mode() {
    let mut checker = SafetyChecker::new(SafetyLevel::Calm);
    assert!(checker.check_spell("safe magic").is_ok());
    assert!(checker.check_spell("unsafe magic").is_err());
}

#[test]
fn test_balanced_mode() {
    let mut checker = SafetyChecker::new(SafetyLevel::Balanced);
    assert!(checker.check_spell("safe magic").is_ok());
    assert!(checker.check_spell("unsafe magic with safe_guard").is_ok());
}

#[test]
fn test_wild_mode() {
    let mut checker = SafetyChecker::new(SafetyLevel::Wild);
    assert!(checker.check_spell("unsafe magic").is_ok());
    assert!(checker.check_spell("forbidden_magic").is_err());
}
