//! Error pattern reporting and analytics
//! Created: 2025-01-20 23:58:03
//! Author: isdood

use crate::{
    analysis::{AnalysisResult, ErrorPattern},
    validation::ValidationResult,
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct RecoveryReport {
    pub generated_at: DateTime<Utc>,
    pub generated_by: String,
    pub summary: ReportSummary,
    pub patterns: Vec<PatternAnalysis>,
    pub recommendations: Vec<Recommendation>,
    pub validation_results: Vec<ValidationResult>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ReportSummary {
    pub total_patterns: usize,
    pub total_recoveries: usize,
    pub overall_success_rate: f64,
    pub critical_patterns: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PatternAnalysis {
    pub pattern: ErrorPattern,
    pub impact_score: f64,
    pub trend: PatternTrend,
    pub first_seen: DateTime<Utc>,
    pub resolution_time: std::time::Duration,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum PatternTrend {
    Improving,
    Stable,
    Degrading,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Recommendation {
    pub priority: Priority,
    pub description: String,
    pub action_items: Vec<String>,
    pub estimated_impact: f64,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum Priority {
    Critical,
    High,
    Medium,
    Low,
}

pub struct ReportBuilder {
    patterns: HashMap<String, Vec<ErrorPattern>>,
    analysis_results: Vec<AnalysisResult>,
    start_time: DateTime<Utc>,
}

impl ReportBuilder {
    pub fn new() -> Self {
        Self {
            patterns: HashMap::new(),
            analysis_results: Vec::new(),
            start_time: Utc::now(),
        }
    }

    pub fn add_analysis_result(&mut self, result: AnalysisResult) {
        self.analysis_results.push(result);
    }

    pub fn build(self) -> RecoveryReport {
        let mut all_patterns: Vec<ErrorPattern> = Vec::new();
        let mut all_recommendations: Vec<String> = Vec::new();
        let mut all_validations: Vec<ValidationResult> = Vec::new();

        // Collect all patterns and recommendations
        for result in self.analysis_results {
            all_patterns.extend(result.patterns);
            all_recommendations.extend(result.recommendations);
            all_validations.extend(result.validation_results);
        }

        // Calculate summary statistics
        let total_patterns = all_patterns.len();
        let total_recoveries: usize = all_patterns.iter().map(|p| p.frequency).sum();
        let overall_success_rate = if total_recoveries > 0 {
            all_patterns.iter().map(|p| p.success_rate * p.frequency as f64).sum::<f64>()
            / total_recoveries as f64
        } else {
            0.0
        };

        // Analyze patterns
        let pattern_analyses: Vec<PatternAnalysis> = all_patterns
        .iter()
        .map(|pattern| {
            let impact_score = calculate_impact_score(pattern);
            PatternAnalysis {
                pattern: pattern.clone(),
             impact_score,
             trend: determine_trend(pattern),
             first_seen: self.start_time,
             resolution_time: std::time::Duration::from_secs(300), // Example duration
            }
        })
        .collect();

        // Generate recommendations
        let recommendations = generate_recommendations(&pattern_analyses, &all_validations);

        RecoveryReport {
            generated_at: Utc::now(),
            generated_by: "isdood".to_string(),
            summary: ReportSummary {
                total_patterns,
                total_recoveries,
                overall_success_rate,
                critical_patterns: pattern_analyses
                .iter()
                .filter(|p| p.impact_score > 0.8)
                .count(),
            },
            patterns: pattern_analyses,
            recommendations,
            validation_results: all_validations,
        }
    }
}

fn calculate_impact_score(pattern: &ErrorPattern) -> f64 {
    let frequency_factor = (pattern.frequency as f64).log10().min(1.0);
    let success_penalty = 1.0 - pattern.success_rate;
    let recency_factor = 1.0 - (Utc::now() - pattern.last_occurrence).num_minutes() as f64 / 1440.0;

    (frequency_factor + success_penalty + recency_factor.max(0.0)) / 3.0
}

fn determine_trend(pattern: &ErrorPattern) -> PatternTrend {
    // Example trend determination logic
    if pattern.success_rate > 0.8 {
        PatternTrend::Improving
    } else if pattern.success_rate < 0.2 {
        PatternTrend::Degrading
    } else {
        PatternTrend::Stable
    }
}

fn generate_recommendations(
    patterns: &[PatternAnalysis],
    validations: &[ValidationResult],
) -> Vec<Recommendation> {
    let mut recommendations = Vec::new();

    // Pattern-based recommendations
    for pattern in patterns {
        if pattern.impact_score > 0.8 {
            recommendations.push(Recommendation {
                priority: Priority::Critical,
                description: format!("Critical pattern detected: {}", pattern.pattern.description),
                                 action_items: vec![
                                     "Review recovery strategy configuration".to_string(),
                                 "Consider implementing circuit breaker".to_string(),
                                 "Add additional monitoring".to_string(),
                                 ],
                                 estimated_impact: pattern.impact_score,
            });
        }
    }

    // Validation-based recommendations
    for validation in validations {
        if !validation.is_valid {
            recommendations.push(Recommendation {
                priority: Priority::High,
                description: validation.message.clone(),
                                 action_items: vec!["Fix validation error".to_string()],
                                 estimated_impact: 0.7,
            });
        }
    }

    recommendations.sort_by_key(|r| std::cmp::Reverse(r.priority.clone()));
    recommendations
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::validation::ValidationSeverity;

    #[test]
    fn test_report_builder() {
        let mut builder = ReportBuilder::new();

        let pattern = ErrorPattern {
            pattern_id: "TEST_1".to_string(),
            description: "Test error".to_string(),
            frequency: 10,
            success_rate: 0.7,
            last_occurrence: Utc::now(),
        };

        let validation_result = ValidationResult {
            is_valid: true,
            message: "Test validation".to_string(),
            severity: ValidationSeverity::Info,
        };

        let analysis_result = AnalysisResult {
            patterns: vec![pattern],
            recommendations: vec!["Test recommendation".to_string()],
            validation_results: vec![validation_result],
        };

        builder.add_analysis_result(analysis_result);
        let report = builder.build();

        assert_eq!(report.summary.total_patterns, 1);
        assert_eq!(report.summary.total_recoveries, 10);
        assert!((report.summary.overall_success_rate - 0.7).abs() < 0.001);
    }

    #[test]
    fn test_impact_score_calculation() {
        let pattern = ErrorPattern {
            pattern_id: "TEST_2".to_string(),
            description: "Test error".to_string(),
            frequency: 100,
            success_rate: 0.3,
            last_occurrence: Utc::now(),
        };

        let score = calculate_impact_score(&pattern);
        assert!(score > 0.0 && score <= 1.0);
    }
}
