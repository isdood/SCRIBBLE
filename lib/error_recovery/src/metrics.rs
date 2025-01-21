//! Detailed metrics collection and analysis for error recovery
//! Created: 2025-01-21 00:07:49
//! Author: isdood

use crate::{
    analysis::ErrorPattern,
    core::RecoveryCore,
    monitor::{Alert, AlertSeverity},
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
pub struct MetricsSnapshot {
    pub timestamp: DateTime<Utc>,
    pub recovery_metrics: RecoveryMetricsData,
    pub performance_metrics: PerformanceMetrics,
    pub resource_metrics: ResourceMetrics,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecoveryMetricsData {
    pub total_recoveries: u64,
    pub successful_recoveries: u64,
    pub failed_recoveries: u64,
    pub success_rate: f64,
    pub pattern_frequencies: HashMap<String, u64>,
    pub average_recovery_time: std::time::Duration,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceMetrics {
    pub p50_recovery_time: std::time::Duration,
    pub p90_recovery_time: std::time::Duration,
    pub p99_recovery_time: std::time::Duration,
    pub max_recovery_time: std::time::Duration,
    pub throughput: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceMetrics {
    pub memory_usage: u64,
    pub cpu_usage: f64,
    pub thread_count: u32,
    pub active_recoveries: u32,
}

pub struct MetricsCollector {
    core: Arc<RecoveryCore>,
    snapshots: Arc<RwLock<Vec<MetricsSnapshot>>>,
    retention_period: Duration,
    collection_interval: std::time::Duration,
}

impl MetricsCollector {
    pub fn new(
        core: RecoveryCore,
        retention_period: Duration,
        collection_interval: std::time::Duration,
    ) -> Self {
        Self {
            core: Arc::new(core),
            snapshots: Arc::new(RwLock::new(Vec::new())),
            retention_period,
            collection_interval,
        }
    }

    pub async fn start(&self) {
        info!(
            time = Utc::now().to_rfc3339(),
              "Starting metrics collection"
        );

        let snapshots = self.snapshots.clone();
        let core = self.core.clone();
        let interval = self.collection_interval;
        let retention = self.retention_period;

        tokio::spawn(async move {
            let mut collection_timer = tokio::time::interval(interval);

            loop {
                collection_timer.tick().await;
                let snapshot = Self::collect_snapshot(&core).await;

                let mut snapshots_lock = snapshots.write().await;
                snapshots_lock.push(snapshot);

                // Clean up old snapshots
                let cutoff = Utc::now() - retention;
                snapshots_lock.retain(|s| s.timestamp > cutoff);

                debug!(
                    snapshot_count = snapshots_lock.len(),
                       "Metrics snapshot collected"
                );
            }
        });
    }

    async fn collect_snapshot(core: &RecoveryCore) -> MetricsSnapshot {
        let report = core.generate_report().await;
        let now = Utc::now();

        let recovery_metrics = Self::calculate_recovery_metrics(&report);
        let performance_metrics = Self::calculate_performance_metrics(&report);
        let resource_metrics = Self::collect_resource_metrics().await;

        MetricsSnapshot {
            timestamp: now,
            recovery_metrics,
            performance_metrics,
            resource_metrics,
        }
    }

    fn calculate_recovery_metrics(report: &crate::reporting::RecoveryReport) -> RecoveryMetricsData {
        let mut pattern_frequencies = HashMap::new();
        let mut total_time = std::time::Duration::default();
        let mut total_recoveries = 0;

        for pattern in &report.patterns {
            pattern_frequencies.insert(
                pattern.pattern.pattern_id.clone(),
                                       pattern.pattern.frequency as u64,
            );
            total_recoveries += pattern.pattern.frequency;
            total_time += pattern.resolution_time;
        }

        let average_time = if total_recoveries > 0 {
            total_time / total_recoveries as u32
        } else {
            std::time::Duration::default()
        };

        RecoveryMetricsData {
            total_recoveries: total_recoveries as u64,
            successful_recoveries: report.summary.total_recoveries as u64,
            failed_recoveries: (total_recoveries as u64) - (report.summary.total_recoveries as u64),
            success_rate: report.summary.overall_success_rate,
            pattern_frequencies,
            average_recovery_time: average_time,
        }
    }

    fn calculate_performance_metrics(report: &crate::reporting::RecoveryReport) -> PerformanceMetrics {
        // Collect all recovery times
        let mut times: Vec<_> = report.patterns
        .iter()
        .map(|p| p.resolution_time)
        .collect();
        times.sort();

        let len = times.len();
        PerformanceMetrics {
            p50_recovery_time: times.get(len / 2)
            .copied()
            .unwrap_or_default(),
            p90_recovery_time: times.get(len * 9 / 10)
            .copied()
            .unwrap_or_default(),
            p99_recovery_time: times.get(len * 99 / 100)
            .copied()
            .unwrap_or_default(),
            max_recovery_time: times.last()
            .copied()
            .unwrap_or_default(),
            throughput: len as f64 / 60.0, // recoveries per minute
        }
    }

    async fn collect_resource_metrics() -> ResourceMetrics {
        // Basic system metrics collection
        ResourceMetrics {
            memory_usage: get_memory_usage(),
            cpu_usage: get_cpu_usage(),
            thread_count: get_thread_count(),
            active_recoveries: get_active_recoveries(),
        }
    }

    pub async fn get_recent_snapshots(&self, duration: Duration) -> Vec<MetricsSnapshot> {
        let snapshots = self.snapshots.read().await;
        let cutoff = Utc::now() - duration;
        snapshots
        .iter()
        .filter(|s| s.timestamp > cutoff)
        .cloned()
        .collect()
    }

    pub async fn get_latest_snapshot(&self) -> Option<MetricsSnapshot> {
        let snapshots = self.snapshots.read().await;
        snapshots.last().cloned()
    }
}

// Helper functions for system metrics
fn get_memory_usage() -> u64 {
    // Placeholder: Implement actual memory usage collection
    0
}

fn get_cpu_usage() -> f64 {
    // Placeholder: Implement actual CPU usage collection
    0.0
}

fn get_thread_count() -> u32 {
    // Placeholder: Implement actual thread count collection
    1
}

fn get_active_recoveries() -> u32 {
    // Placeholder: Implement actual active recoveries count
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration as StdDuration;

    #[tokio::test]
    async fn test_metrics_collector() {
        let core = RecoveryCore::new();
        let collector = MetricsCollector::new(
            core,
            Duration::hours(24),
                                              StdDuration::from_secs(60),
        );

        collector.start().await;
        tokio::time::sleep(StdDuration::from_secs(1)).await;

        let snapshot = collector.get_latest_snapshot().await;
        assert!(snapshot.is_some());
    }

    #[tokio::test]
    async fn test_snapshot_retention() {
        let core = RecoveryCore::new();
        let collector = MetricsCollector::new(
            core,
            Duration::minutes(5),
                                              StdDuration::from_secs(1),
        );

        collector.start().await;
        tokio::time::sleep(StdDuration::from_secs(2)).await;

        let snapshots = collector.get_recent_snapshots(Duration::minutes(1)).await;
        assert!(!snapshots.is_empty());
    }
}
