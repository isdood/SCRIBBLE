#[allow(dead_code)]  // Allow unused variants since they'll be used in the future
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SafetyLevel {
    Calm,
    Balanced,
    Wild,
}

impl Default for SafetyLevel {
    fn default() -> Self {
        SafetyLevel::Calm
    }
}
