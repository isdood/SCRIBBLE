// lib/carve/tests/translation_tests.rs

use carve::{UnifiedTranslator, ZigTranslator};

#[test]
fn test_unified_translator() {
    let mut translator = UnifiedTranslator::new();

    // Test Zig translation
    let zig_source = r#"!zig!
    fn Vector => struct {
    x: f32,
    y: f32,

    fn init(x: f32, y: f32) => Self {
    return .{ .x = x, .y = y };
}

fn add(self, other: Vector) => Vector {
return Vector.init(self.x + other.x, self.y + other.y);
}
}

fn main() => void {
const v1 := Vector.init(1.0, 2.0);
const v2 := Vector.init(3.0, 4.0);
const v3 := v1.add(v2);
}
!zig!"#;

let result = translator.translate(zig_source).unwrap();

// Verify Zig translation results
assert!(result.contains("pub fn Vector = struct {"));
assert!(result.contains("pub fn init(x: f32, y: f32) Self {"));
assert!(result.contains("const v1 = Vector.init(1.0, 2.0);"));
}

#[test]
fn test_mixed_language_translation() {
    let mut translator = UnifiedTranslator::new();

    let mixed_source = r#"
    // First some Zig code
    !zig!
    fn add(a: i32, b: i32) => i32 {
    return a + b;
}
!zig!

// Then some Python
!snek!
def multiply(a, b):
return a * b
!snek!

// Finally some SQL
!sql!
SELECT * FROM calculations WHERE result > 10;
!sql!
"#;

let result = translator.translate(mixed_source).unwrap();

// Verify each language section
assert!(result.contains("pub fn add(a: i32, b: i32) i32 {"));
assert!(result.contains("def multiply(a, b):"));
assert!(result.contains("SELECT * FROM calculations"));
}

#[test]
fn test_quantum_coherence() {
    let mut translator = UnifiedTranslator::new();

    // Test multiple translations to verify quantum coherence
    for i in 0..100 {
        let zig_source = format!("!zig!fn test{}() => void {{ }}!zig!", i);
        let result = translator.translate(&zig_source);
        assert!(result.is_ok(), "Translation failed at iteration {}", i);
    }
}

#[test]
fn test_zig_specific_features() {
    let mut translator = ZigTranslator::new();

    // Test struct declarations
    let struct_def = "struct Point => { x: f32, y: f32 }";
    assert!(translator.process_line(struct_def).unwrap().contains("struct Point = {"));

    // Test function declarations
    let fn_def = "fn calculate(value: f64) => ?f64 {";
    assert!(translator.process_line(fn_def).unwrap().contains("pub fn calculate(value: f64) ?f64 {"));

    // Test const declarations
    let const_def = "const PI := 3.14159;";
    assert!(translator.process_line(const_def).unwrap().contains("const PI = 3.14159;"));

    // Test type annotations
    let type_def = "var name: str = \"test\";";
    assert!(translator.process_line(type_def).unwrap().contains("[]const u8"));
}

#[test]
fn test_error_handling() {
    let mut translator = UnifiedTranslator::new();

    // Test unclosed block
    let unclosed = "!zig!fn test() => void {";
    assert!(translator.translate(unclosed).is_err());

    // Test invalid language
    let invalid = "!invalid!test!invalid!";
    let result = translator.translate(invalid);
    assert_eq!(result.unwrap(), invalid);

    // Test empty translation
    let empty = "!zig!!zig!";
    assert!(translator.translate(empty).unwrap().is_empty());
}

#[test]
fn test_inline_translations() {
    let mut translator = UnifiedTranslator::new();

    let source = "Computing result: !zig! const x := 42; !zig! and !sql! SELECT value FROM data; !sql!";
    let result = translator.translate(source).unwrap();

    assert!(result.contains("inline_zig!(const x = 42;)"));
    assert!(result.contains("inline_sql!(SELECT value FROM data;)"));
}

#[test]
fn test_indentation_handling() {
    let mut translator = ZigTranslator::new();

    let code = vec![
        "fn test() => void {",
        "    const x := 1;",
        "    if (x > 0) {",
        "        const y := 2;",
        "    }",
        "}"
    ];

    let mut result = String::new();
    for line in code {
        result.push_str(&translator.process_line(line).unwrap());
        result.push('\n');
    }

    // Verify indentation
    assert!(result.contains("    const x = 1;"));
    assert!(result.contains("        const y = 2;"));
}
