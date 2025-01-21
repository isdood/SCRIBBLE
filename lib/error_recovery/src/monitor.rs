//! Real-time monitoring and telemetry for error recovery
//! Created: 2025-01-21 00:04:40
//! Author: isdood

use crate::{
    analysis::ErrorPattern,
    core::RecoveryCore,
    reporting::RecoveryReport,
};
use chrono::{DateTime, Utc};
use std::{
    collections::HashMap,
    sync::Arc,
    time::{Duration, Instant},
};
use tokio::sync::RwLock;
use tracing::{debug, info, warn};

/// Metrics collected for error recovery monitoring
#[derive(Debug, Clone)]
pub struct RecoveryMetrics {
    pub timestamp: DateTime<Utc>,
    pub total_attempts: u64,
    pub successful_recoveries: u64,
    pub failed_recoveries: u64,
    pub average_recovery_time: Duration,
    pub error_patterns: HashMap<String, u64>,
}

impl RecoveryMetrics {
    fn new() -> Self {
        Self {
            timestamp: Utc::now(),
            total_attempts: 0,
            successful_recoveries: 0,
            failed_recoveries: 0,
            average_recovery_time: Duration::default(),
            error_patterns: HashMap::new(),
        }
    }

    fn success_rate(&self) -> f64 {
        if self.total_attempts == 0 {
            0.0
        } else {
            self.successful_recoveries as f64 / self.total_attempts as f64
        }
    }
}

/// Monitor for tracking recovery system performance
#[derive(Clone)]
pub struct RecoveryMonitor {
    core: Arc<RecoveryCore>,
    metrics: Arc<RwLock<RecoveryMetrics>>,
    history: Arc<RwLock<Vec<RecoveryMetrics>>>,
    alerts: Arc<RwLock<Vec<Alert>>>,
}

#[derive(Debug, Clone)]
pub struct Alert {
    pub timestamp: DateTime<Utc>,
    pub severity: AlertSeverity,
    pub message: String,
    pub pattern_id: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum AlertSeverity {
    Critical,
    Warning,
    Info,
}

impl RecoveryMonitor {
    pub fn new(core: RecoveryCore) -> Self {
        Self {
            core: Arc::new(core),
            metrics: Arc::new(RwLock::new(RecoveryMetrics::new())),
            history: Arc::new(RwLock::new(Vec::new())),
            alerts: Arc::new(RwLock::new(Vec::new())),
        }
    }

    /// Start monitoring the recovery system
    pub async fn start(&self) {
        info!("Starting recovery system monitoring");

        let metrics_clone = self.metrics.clone();
        let history_clone = self.history.clone();
        let alerts_clone = self.alerts.clone();
        let core_clone = self.core.clone();

        // Spawn metrics collection task
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(Duration::from_secs(60));
            loop {
                interval.tick().await;

                // Generate new metrics
                let report = core_clone.generate_report().await;
                let new_metrics = Self::generate_metrics(&report).await;

                // Update current metrics
                let mut metrics = metrics_clone.write().await;
                *metrics = new_metrics.clone();

                // Update history
                let mut history = history_clone.write().await;
                history.push(new_metrics.clone());

                // Trim history to last 24 hours
                let day_ago = Utc::now() - chrono::Duration::hours(24);
                history.retain(|m| m.timestamp > day_ago);

                // Check for alerts
                let alerts = Self::check_alerts(&new_metrics).await;
                if !alerts.is_empty() {
                    let mut alert_store = alerts_clone.write().await;
                    alert_store.extend(alerts);
                }

                debug!("Metrics updated successfully");
            }
        });
    }

    /// Get current metrics
    pub async fn get_metrics(&self) -> RecoveryMetrics {
        self.metrics.read().await.clone()
    }

    /// Get historical metrics for a time range
    pub async fn get_history(
        &self,
        start: DateTime<Utc>,
        end: DateTime<Utc>,
    ) -> Vec<RecoveryMetrics> {
        let history = self.history.read().await;
        history
        .iter()
        .filter(|m| m.timestamp >= start && m.timestamp <= end)
        .cloned()
        .collect()
    }

    /// Get active alerts
    pub async fn get_alerts(&self) -> Vec<Alert> {
        self.alerts.read().await.clone()
    }

    async fn generate_metrics(report: &RecoveryReport) -> RecoveryMetrics {
        let mut metrics = RecoveryMetrics::new();

        for pattern in &report.patterns {
            metrics.total_attempts += pattern.pattern.frequency as u64;
            let successes = (pattern.pattern.frequency as f64 * pattern.pattern.success_rate) as u64;
            metrics.successful_recoveries += successes;
            metrics.failed_recoveries += pattern.pattern.frequency as u64 - successes;
            metrics.error_patterns.insert(
                pattern.pattern.pattern_id.clone(),
                                          pattern.pattern.frequency as u64,
            );
        }

        metrics
    }

    async fn check_alerts(metrics: &RecoveryMetrics) -> Vec<Alert> {
        let mut alerts = Vec::new();
        let success_rate = metrics.success_rate();

        // Check overall success rate
        if success_rate < 0.5 {
            alerts.push(Alert {
                timestamp: Utc::now(),
                        severity: AlertSeverity::Critical,
                        message: format!(
                            "Low recovery success rate: {:.1}%",
                            success_rate * 100.0
                        ),
                        pattern_id: None,
            });
        }

        // Check individual patterns
        for (pattern_id, count) in &metrics.error_patterns {
            if *count > 100 {
                alerts.push(Alert {
                    timestamp: Utc::now(),
                            severity: AlertSeverity::Warning,
                            message: format!("High frequency of pattern: {} occurrences", count),
                            pattern_id: Some(pattern_id.clone()),
                });
            }
        }

        alerts
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[tokio::test]
    async fn test_monitor_metrics() {
        let core = RecoveryCore::new();
        let monitor = RecoveryMonitor::new(core);

        // Get initial metrics
        let metrics = monitor.get_metrics().await;
        assert_eq!(metrics.total_attempts, 0);

        // Test history
        let start = Utc::now() - chrono::Duration::hours(1);
        let end = Utc::now();
        let history = monitor.get_history(start, end).await;
        assert!(history.is_empty());
    }

    #[tokio::test]
    async fn test_alert_generation() {
        let mut metrics = RecoveryMetrics::new();
        metrics.total_attempts = 100;
        metrics.successful_recoveries = 40;
        metrics.failed_recoveries = 60;

        let alerts = RecoveryMonitor::check_alerts(&metrics).await;
        assert!(!alerts.is_empty());
        assert_eq!(alerts[0].severity, AlertSeverity::Critical);
    }
}
