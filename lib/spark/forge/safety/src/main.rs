use spark_safety::{SafetyChecker, SafetyLevel};

fn main() {
    println!("🛡️ Spark Safety Checker v0.1.0");

    let mut checker = SafetyChecker::new(SafetyLevel::Balanced);

    if let Err(e) = checker.check_spell("unsafe magic") {
        println!("❌ Safety violation detected: {}", e);
    } else {
        println!("✨ Spell checked successfully!");
    }

    let stats = checker.get_stats();
    println!("📊 Safety Level: {}", stats.level);
}
