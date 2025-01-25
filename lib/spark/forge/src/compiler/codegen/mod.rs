use crate::compiler::safety::SafetyLevel;

pub struct SparkCodeGen {
    safety: SafetyLevel,
}

impl SparkCodeGen {
    pub fn new(safety: SafetyLevel) -> Self {
        Self { safety }
    }

    pub fn generate(&self) -> Result<(), String> {
        println!("Generating code with {:?} safety", self.safety);
        Ok(())
    }
}
