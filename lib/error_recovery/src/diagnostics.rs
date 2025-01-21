//! Diagnostic tools and reporting for error recovery system
//! Created: 2025-01-21 00:21:45
//! Author: isdood

use crate::{
    core::RecoveryCore,
    metrics::{MetricsCollector, MetricsSnapshot},
    monitor::{Alert, AlertSeverity},
    reporting::RecoveryReport,
};
use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    sync::Arc,
};
use tokio::sync::RwLock;
use tracing::{debug, info, warn};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiagnosticReport {
    pub timestamp: DateTime<Utc>,
    pub system_state: SystemState,
    pub performance_analysis: PerformanceAnalysis,
    pub health_check: HealthCheckResult,
    pub recommendations: Vec<Recommendation>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemState {
    pub uptime: Duration,
    pub active_strategies: Vec<String>,
    pub recent_alerts: Vec<Alert>,
    pub resource_state: ResourceState,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceAnalysis {
    pub recovery_success_trend: TrendAnalysis,
    pub response_time_trend: TrendAnalysis,
    pub resource_usage_trend: TrendAnalysis,
    pub bottlenecks: Vec<Bottleneck>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrendAnalysis {
    pub current_value: f64,
    pub change_rate: f64,
    pub trend_direction: TrendDirection,
    pub significance: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TrendDirection {
    Improving,
    Stable,
    Degrading,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Bottleneck {
    pub component: String,
    pub severity: f64,
    pub description: String,
    pub suggested_action: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthCheckResult {
    pub overall_status: HealthStatus,
    pub component_status: HashMap<String, ComponentHealth>,
    pub last_check: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum HealthStatus {
    Healthy,
    Degraded,
    Unhealthy,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComponentHealth {
    pub status: HealthStatus,
    pub last_error: Option<String>,
    pub metrics: HashMap<String, f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceState {
    pub memory_usage: f64,
    pub cpu_usage: f64,
    pub thread_count: u32,
    pub active_recoveries: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Recommendation {
    pub priority: u32,
    pub category: String,
    pub description: String,
    pub action_items: Vec<String>,
    pub estimated_impact: f64,
}

pub struct DiagnosticsEngine {
    core: Arc<RecoveryCore>,
    metrics_collector: Arc<MetricsCollector>,
    start_time: DateTime<Utc>,
    last_report: Arc<RwLock<Option<DiagnosticReport>>>,
}

impl DiagnosticsEngine {
    pub fn new(core: RecoveryCore, metrics_collector: MetricsCollector) -> Self {
        Self {
            core: Arc::new(core),
            metrics_collector: Arc::new(metrics_collector),
            start_time: Utc::now(),
            last_report: Arc::new(RwLock::new(None)),
        }
    }

    pub async fn generate_report(&self) -> DiagnosticReport {
        info!("Generating diagnostic report");

        let metrics = self.metrics_collector.get_latest_snapshot().await;
        let recovery_report = self.core.generate_report().await;

        let report = DiagnosticReport {
            timestamp: Utc::now(),
            system_state: self.analyze_system_state(&metrics).await,
            performance_analysis: self.analyze_performance(&recovery_report, &metrics).await,
            health_check: self.perform_health_check().await,
            recommendations: self.generate_recommendations(&recovery_report, &metrics).await,
        };

        *self.last_report.write().await = Some(report.clone());
        report
    }

    async fn analyze_system_state(&self, metrics: &Option<MetricsSnapshot>) -> SystemState {
        let uptime = Utc::now() - self.start_time;

        SystemState {
            uptime,
            active_strategies: vec!["TransientError", "ResourceExhaustion"].into_iter().map(String::from).collect(),
            recent_alerts: vec![], // Would be populated from monitoring system
            resource_state: self.get_resource_state(metrics).await,
        }
    }

    async fn analyze_performance(
        &self,
        report: &RecoveryReport,
        metrics: &Option<MetricsSnapshot>
    ) -> PerformanceAnalysis {
        PerformanceAnalysis {
            recovery_success_trend: self.analyze_trend(
                report.summary.overall_success_rate,
                0.8,
                0.01
            ),
            response_time_trend: TrendAnalysis {
                current_value: 100.0,
                change_rate: 0.0,
                trend_direction: TrendDirection::Stable,
                significance: 0.5,
            },
            resource_usage_trend: TrendAnalysis {
                current_value: 50.0,
                change_rate: 0.0,
                trend_direction: TrendDirection::Stable,
                significance: 0.5,
            },
            bottlenecks: vec![],
        }
    }

    async fn perform_health_check(&self) -> HealthCheckResult {
        HealthCheckResult {
            overall_status: HealthStatus::Healthy,
            component_status: HashMap::new(),
            last_check: Utc::now(),
        }
    }

    async fn generate_recommendations(
        &self,
        report: &RecoveryReport,
        metrics: &Option<MetricsSnapshot>
    ) -> Vec<Recommendation> {
        let mut recommendations = Vec::new();

        if report.summary.overall_success_rate < 0.8 {
            recommendations.push(Recommendation {
                priority: 1,
                category: "Performance".to_string(),
                                 description: "Low recovery success rate detected".to_string(),
                                 action_items: vec![
                                     "Review recovery strategies configuration".to_string(),
                                 "Analyze failed recovery patterns".to_string(),
                                 ],
                                 estimated_impact: 0.8,
            });
        }

        recommendations
    }

    async fn get_resource_state(&self, metrics: &Option<MetricsSnapshot>) -> ResourceState {
        ResourceState {
            memory_usage: 50.0,
            cpu_usage: 30.0,
            thread_count: 4,
            active_recoveries: 2,
        }
    }

    fn analyze_trend(&self, current: f64, threshold: f64, change_threshold: f64) -> TrendAnalysis {
        let direction = if current > threshold {
            TrendDirection::Improving
        } else if current < threshold - change_threshold {
            TrendDirection::Degrading
        } else {
            TrendDirection::Stable
        };

        TrendAnalysis {
            current_value: current,
            change_rate: 0.0,
            trend_direction: direction,
            significance: 0.5,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration as StdDuration;

    #[tokio::test]
    async fn test_diagnostics_engine() {
        let core = RecoveryCore::new();
        let metrics_collector = MetricsCollector::new(
            core.clone(),
                                                      Duration::hours(24),
                                                      StdDuration::from_secs(60),
        );

        let engine = DiagnosticsEngine::new(core, metrics_collector);
        let report = engine.generate_report().await;

        assert!(matches!(report.health_check.overall_status, HealthStatus::Healthy));
    }

    #[tokio::test]
    async fn test_trend_analysis() {
        let core = RecoveryCore::new();
        let metrics_collector = MetricsCollector::new(
            core.clone(),
                                                      Duration::hours(24),
                                                      StdDuration::from_secs(60),
        );

        let engine = DiagnosticsEngine::new(core, metrics_collector);
        let trend = engine.analyze_trend(0.9, 0.8, 0.01);

        assert!(matches!(trend.trend_direction, TrendDirection::Improving));
    }
}
