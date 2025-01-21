// lib.rs - Main entry point for Prism library
// Created by: isdood
// Date: 2025-01-21 11:37:46 UTC

//! Prism - A high-performance crystal pattern simulation system
//! 
//! This library provides tools for generating, analyzing, and optimizing crystal structures
//! with a focus on performance and accuracy.

pub mod crystal;
pub mod pattern;
pub mod runtime;
pub mod types;
pub mod integration;

use std::sync::Arc;

pub use crate::crystal::bridge::{Crystal, CrystalNode, CrystalSystem};
pub use crate::pattern::{Pattern, PatternConfig, PatternType};
pub use crate::runtime::task::{Task, TaskConfig, TaskExecutor};
pub use crate::types::{PrismError, PrismResult, Priority, TaskStatus};

/// Version of the Prism library
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Result type for Prism operations
pub type Result<T> = std::result::Result<T, PrismError>;

/// Runtime configuration for Prism
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
            stack_size: 2 * 1024 * 1024, // 2MB
            use_hardware_threads: true,
            memory_limit: None,
        }
    }
}

/// Main runtime for Prism operations
pub struct Runtime {
    executor: TaskExecutor,
    crystal: Arc<Crystal>,
    config: RuntimeConfig,
}

impl Runtime {
    /// Initialize a new Prism runtime with the given configuration
    pub fn init(config: RuntimeConfig) -> Result<Self> {
        let crystal = Arc::new(Crystal::new(CrystalSystem::Cubic)?);
        let executor = TaskExecutor::new(Some(Arc::clone(&crystal)))?;

        Ok(Self {
            executor,
            crystal,
            config,
        })
    }

    /// Create a new task with the given configuration
    pub fn create_task(&self, config: TaskConfig) -> Result<Task> {
        Task::new(config)
    }

    /// Execute a task
    pub async fn execute(&self, task: Task) -> Result<()> {
        self.executor.submit(task, Default::default()).await
    }

    /// Get a reference to the current crystal system
    pub fn crystal(&self) -> &Arc<Crystal> {
        &self.crystal
    }

    /// Get the current runtime configuration
    pub fn config(&self) -> &RuntimeConfig {
        &self.config
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_runtime_init() {
        let config = RuntimeConfig::default();
        let runtime = Runtime::init(config).unwrap();
        assert!(Arc::strong_count(&runtime.crystal) >= 1);
    }

    #[test]
    fn test_task_creation() {
        let runtime = Runtime::init(RuntimeConfig::default()).unwrap();
        let task_config = TaskConfig::default();
        let task = runtime.create_task(task_config).unwrap();
        assert_eq!(task.status(), TaskStatus::Ready);
    }

    #[test]
    fn test_version() {
        assert!(!VERSION.is_empty());
    }
}

// Re-exports for commonly used types
pub mod prelude {
    pub use super::{
        Crystal,
        CrystalSystem,
        Pattern,
        PatternConfig,
        PatternType,
        PrismError,
        PrismResult,
        Priority,
        Result,
        Runtime,
        RuntimeConfig,
        Task,
        TaskConfig,
        TaskStatus,
    };
}
