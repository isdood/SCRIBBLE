pub enum SafetyLevel {
    Calm,
    Balanced,
    Wild,
}

pub struct SafetyChecker {
    level: SafetyLevel,
}

impl SafetyChecker {
    pub fn new(level: SafetyLevel) -> Self {
        Self { level }
    }

    pub fn check_safety(&self, ast: &SparkAst) -> Result<(), SafetyError> {
        match self.level {
            SafetyLevel::Calm => self.check_calm(ast),
            SafetyLevel::Balanced => self.check_balanced(ast),
            SafetyLevel::Wild => Ok(()), // No checks in wild mode
        }
    }
}
