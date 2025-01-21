//! Prism Internal Implementation Module
//! ==================================
//!
//! Core implementation layer for the quantum-harmonic computational framework.
//! This module provides the internal components and implementations that are
//! re-exported through the public API.
//!
//! Module Structure:
//! ---------------
//! - binding: FFI interface for Zig crystal operations
//! - crystal: Core quantum pattern implementation
//! - runtime: Task scheduling and execution engine
//! - types: Shared types and error definitions
//!
//! Author: Caleb J.D. Terkovics <isdood>
//! Created: 2025-01-21
//! Last Updated: 2025-01-21 13:02:37 UTC
//! Current User: isdood

// Public modules
pub mod binding;
pub mod crystal;
pub mod runtime;
pub mod types;

// Standard library imports
use std::sync::Arc;
use std::future::Future;

// Re-exports
pub use crystal::bridge::{Crystal, CrystalNode, CrystalSystem};
pub use runtime::task::{Task, TaskConfig, TaskExecutor};
pub use types::{PrismError, PrismResult, Priority, TaskStatus};

/// Result type for quantum-harmonic operations
pub type Result<T> = PrismResult<T>;

#[derive(Clone)]
pub(crate) struct InternalRuntime {
    executor: TaskExecutor,
    crystal: Arc<Crystal>,
    config: RuntimeConfig,
}

#[derive(Debug, Clone)]
pub struct RuntimeConfig {
    pub thread_count: u32,
    pub stack_size: usize,
    pub use_hardware_threads: bool,
    pub memory_limit: Option<usize>,
}

impl Default for RuntimeConfig {
    fn default() -> Self {
        Self {
            thread_count: num_cpus::get() as u32,
            stack_size: 2 * 1024 * 1024,
            use_hardware_threads: true,
            memory_limit: None,
        }
    }
}

impl InternalRuntime {
    pub fn new(config: RuntimeConfig) -> Result<Self> {
        let crystal = Arc::new(Crystal::new(CrystalSystem::Cubic)
        .map_err(|e| PrismError::Crystal(e.to_string()))?);

        let executor = TaskExecutor::new(Some(Arc::clone(&crystal)))
        .map_err(|e| PrismError::Runtime(e.to_string()))?;

        Ok(Self {
            executor,
            crystal,
            config,
        })
    }

    pub fn create_task<F>(&self, future: F, config: TaskConfig) -> Result<Task<F>>
    where
    F: Future<Output = Result<()>> + Send + 'static,
    {
        Task::new(future, config, Some(Arc::clone(&self.crystal)))
    }

    pub async fn execute<F>(&self, task: Task<F>) -> Result<()>
    where
    F: Future<Output = Result<()>> + Send + 'static,
    {
        self.executor.submit(task, Default::default())
        .await
        .map_err(|e| PrismError::Runtime(e.to_string()))
    }

    pub fn crystal(&self) -> &Arc<Crystal> {
        &self.crystal
    }

    pub fn config(&self) -> &RuntimeConfig {
        &self.config
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;
    use tokio::time;

    async fn test_task() -> Result<()> {
        time::sleep(Duration::from_millis(100)).await;
        Ok(())
    }

    #[tokio::test]
    async fn test_runtime_init() {
        let config = RuntimeConfig::default();
        let runtime = InternalRuntime::new(config).unwrap();
        assert!(Arc::strong_count(&runtime.crystal) >= 1);
    }

    #[tokio::test]
    async fn test_task_creation() {
        let runtime = InternalRuntime::new(RuntimeConfig::default()).unwrap();
        let task_config = TaskConfig::default();
        let task = runtime.create_task(test_task(), task_config).unwrap();
        assert_eq!(task.status(), TaskStatus::Ready);
    }
}
