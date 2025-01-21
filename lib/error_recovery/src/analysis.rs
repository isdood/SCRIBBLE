//! Static analysis integration for error recovery patterns
//! Created: 2025-01-20 23:54:07
//! Author: isdood

use crate::{
    validation::{ValidationRegistry, ValidationResult, ValidationSeverity},
    RecoveryStrategy,
};
use error_integration::context::ErrorContext;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

/// Represents a pattern of errors and their recovery behavior
#[derive(Debug, Clone)]
pub struct ErrorPattern {
    pub pattern_id: String,
    pub description: String,
    pub frequency: usize,
    pub success_rate: f64,
    pub last_occurrence: chrono::DateTime<chrono::Utc>,
}

/// Analysis results for error recovery patterns
#[derive(Debug)]
pub struct AnalysisResult {
    pub patterns: Vec<ErrorPattern>,
    pub recommendations: Vec<String>,
    pub validation_results: Vec<ValidationResult>,
}

/// Static analyzer for error recovery strategies
pub struct RecoveryAnalyzer {
    patterns: Arc<RwLock<HashMap<String, ErrorPattern>>>,
    validation_registry: ValidationRegistry,
}

impl RecoveryAnalyzer {
    pub fn new(validation_registry: ValidationRegistry) -> Self {
        Self {
            patterns: Arc::new(RwLock::new(HashMap::new())),
            validation_registry,
        }
    }

    pub async fn record_pattern(
        &self,
        error: &(dyn std::error::Error + Send + Sync),
                                context: &ErrorContext,
                                success: bool,
    ) {
        let pattern_id = self.generate_pattern_id(error);
        let mut patterns = self.patterns.write().await;

        let pattern = patterns.entry(pattern_id.clone()).or_insert_with(|| ErrorPattern {
            pattern_id: pattern_id.clone(),
                                                                        description: error.to_string(),
                                                                        frequency: 0,
                                                                        success_rate: 0.0,
                                                                        last_occurrence: chrono::Utc::now(),
        });

        pattern.frequency += 1;
        pattern.last_occurrence = chrono::Utc::now();

        // Update success rate
        let total = pattern.frequency as f64;
        let current_successes = pattern.success_rate * (total - 1.0);
        pattern.success_rate = (current_successes + if success { 1.0 } else { 0.0 }) / total;

        tracing::debug!(
            pattern_id = %pattern_id,
            frequency = pattern.frequency,
            success_rate = %pattern.success_rate,
            "Updated error pattern"
        );
    }

    pub async fn analyze_strategy(
        &self,
        strategy: &dyn RecoveryStrategy,
    ) -> AnalysisResult {
        let patterns = self.patterns.read().await;
        let validation_results = self.validation_registry.validate_strategy(strategy).await;

        let mut recommendations = Vec::new();

        // Analyze patterns and generate recommendations
        for pattern in patterns.values() {
            if pattern.frequency >= 10 && pattern.success_rate < 0.5 {
                recommendations.push(format!(
                    "Consider revising recovery strategy for pattern '{}'. Success rate: {:.2}%",
                    pattern.description,
                    pattern.success_rate * 100.0
                ));
            }
        }

        // Add validation-based recommendations
        for result in &validation_results {
            if !result.is_valid && result.severity == ValidationSeverity::Error {
                recommendations.push(result.message.clone());
            }
        }

        AnalysisResult {
            patterns: patterns.values().cloned().collect(),
            recommendations,
            validation_results,
        }
    }

    fn generate_pattern_id(&self, error: &(dyn std::error::Error + Send + Sync)) -> String {
        use std::hash::{Hash, Hasher};
        let mut hasher = std::collections::hash_map::DefaultHasher::new();

        error.to_string().hash(&mut hasher);
        format!("ERR_{:x}", hasher.finish())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{RecoveryConfig, RecoveryError};
    use async_trait::async_trait;
    use std::io::{Error as IoError, ErrorKind};
    use std::time::Duration;

    struct TestStrategy {
        config: RecoveryConfig,
    }

    #[async_trait]
    impl RecoveryStrategy for TestStrategy {
        type Error = std::io::Error;

        async fn attempt_recovery(
            &self,
            _error: &Self::Error,
            _context: &ErrorContext,
        ) -> Result<(), RecoveryError> {
            Ok(())
        }

        fn can_handle(&self, _error: &Self::Error) -> bool {
            true
        }

        fn config(&self) -> &RecoveryConfig {
            &self.config
        }
    }

    #[tokio::test]
    async fn test_pattern_recording() {
        let validation_registry = ValidationRegistry::new();
        let analyzer = RecoveryAnalyzer::new(validation_registry);
        let context = ErrorContext::new();

        let error = IoError::new(ErrorKind::WouldBlock, "test error");

        // Record multiple occurrences
        for success in [true, false, true] {
            analyzer.record_pattern(&error, &context, success).await;
        }

        let patterns = analyzer.patterns.read().await;
        let pattern = patterns.values().next().unwrap();

        assert_eq!(pattern.frequency, 3);
        assert!((pattern.success_rate - 0.666).abs() < 0.001);
    }

    #[tokio::test]
    async fn test_strategy_analysis() {
        let validation_registry = ValidationRegistry::new();
        let analyzer = RecoveryAnalyzer::new(validation_registry);

        let strategy = TestStrategy {
            config: RecoveryConfig {
                max_attempts: 3,
                retry_delay: Duration::from_millis(100),
                use_backoff: true,
            },
        };

        let result = analyzer.analyze_strategy(&strategy).await;
        assert!(result.recommendations.is_empty());
    }
}
