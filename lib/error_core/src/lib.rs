#[derive(Debug, Default)]
pub struct QuickFix {
    pub description: String,
    pub code: String,
}

#[derive(Debug, Default)]
pub struct DiagnosticReport {
    pub message: String,
    pub suggestions: Vec<String>,
    pub quick_fixes: Vec<QuickFix>,
}

impl DiagnosticReport {
    pub fn new() -> Self {
        Self::default()
    }
}

#[derive(Debug)]
pub struct CompileTimeError {
    pub message: String,
    pub location: String,
}

pub trait Diagnose {
    fn diagnose(&self) -> DiagnosticReport;
    fn get_quick_fixes(&self) -> Vec<QuickFix>;
    fn check_at_compile_time() -> Option<CompileTimeError>;
}
