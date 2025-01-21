// task.rs - Task management for Prism runtime
// Created by: isdood
// Date: 2025-01-21 11:08:32 UTC

use std::future::Future;
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll, Waker};
use std::time::{Duration, Instant};

use crate::crystal::bridge::Crystal;
use crate::types::{PrismError, PrismResult, Priority, TaskHandle, TaskMetadata, TaskState};

/// Task configuration
#[derive(Debug, Clone)]
pub struct TaskConfig {
    pub priority: Priority,
    pub timeout: Option<Duration>,
    pub stack_size: Option<usize>,
    pub crystal_alignment: bool,
}

impl Default for TaskConfig {
    fn default() -> Self {
        Self {
            priority: Priority::Normal,
            timeout: None,
            stack_size: None,
            crystal_alignment: true,
        }
    }
}

/// Task execution context
pub struct TaskContext {
    handle: TaskHandle,
    state: Arc<TaskState>,
    crystal: Option<Arc<Crystal>>,
    config: TaskConfig,
    start_time: Option<Instant>,
    waker: Option<Waker>,
}

impl TaskContext {
    /// Create a new task context
    pub fn new(handle: TaskHandle, config: TaskConfig, crystal: Option<Arc<Crystal>>) -> Self {
        Self {
            handle,
            state: Arc::new(TaskState::new(handle, config.priority)),
            crystal,
            config,
            start_time: None,
            waker: None,
        }
    }

    /// Get task handle
    pub fn handle(&self) -> TaskHandle {
        self.handle
    }

    /// Get task metadata
    pub fn metadata(&self) -> PrismResult<TaskMetadata> {
        Ok(self.state.metadata.lock().unwrap().clone())
    }

    /// Get task priority
    pub fn priority(&self) -> Priority {
        self.config.priority
    }

    /// Check if task has timed out
    fn check_timeout(&self) -> PrismResult<()> {
        if let (Some(timeout), Some(start_time)) = (self.config.timeout, self.start_time) {
            if start_time.elapsed() > timeout {
                return Err(PrismError::Timeout);
            }
        }
        Ok(())
    }
}

/// Executable task wrapper
pub struct Task<F>
where
    F: Future<Output = PrismResult<()>>,
{
    context: TaskContext,
    future: Pin<Box<F>>,
}

impl<F> Task<F>
where
    F: Future<Output = PrismResult<()>>,
{
    /// Create a new task
    pub fn new(future: F, config: TaskConfig, crystal: Option<Arc<Crystal>>) -> Self {
        Self {
            context: TaskContext::new(TaskHandle::new(), config, crystal),
            future: Box::pin(future),
        }
    }

    /// Get task context
    pub fn context(&self) -> &TaskContext {
        &self.context
    }

    /// Execute the task
    pub async fn execute(mut self) -> PrismResult<()> {
        self.context.start_time = Some(Instant::now());
        self.context.state.set_status(crate::types::TaskStatus::Running);

        // Align with crystal pattern if enabled
        if self.context.config.crystal_alignment {
            if let Some(crystal) = &self.context.crystal {
                crystal.optimize()?;
            }
        }

        let result = self.future.as_mut().await;
        
        match &result {
            Ok(_) => self.context.state.set_status(crate::types::TaskStatus::Completed),
            Err(_) => self.context.state.set_status(crate::types::TaskStatus::Failed),
        }

        result
    }
}

/// Task executor for running multiple tasks
pub struct TaskExecutor {
    tasks: Arc<Mutex<Vec<Box<dyn TaskTrait + Send>>>>,
    crystal: Option<Arc<Crystal>>,
}

impl TaskExecutor {
    /// Create a new task executor
    pub fn new(crystal: Option<Arc<Crystal>>) -> Self {
        Self {
            tasks: Arc::new(Mutex::new(Vec::new())),
            crystal,
        }
    }

    /// Submit a task for execution
    pub fn submit<F>(&self, future: F, config: TaskConfig) -> PrismResult<TaskHandle>
    where
        F: Future<Output = PrismResult<()>> + Send + 'static,
    {
        let task = Task::new(future, config, self.crystal.clone());
        let handle = task.context().handle();
        
        self.tasks.lock().unwrap().push(Box::new(TaskWrapper(task)));
        Ok(handle)
    }

    /// Execute all pending tasks
    pub async fn execute_all(&self) -> PrismResult<()> {
        let mut tasks = self.tasks.lock().unwrap();
        let mut results = Vec::new();

        // Sort tasks by priority
        tasks.sort_by(|a, b| b.priority().cmp(&a.priority()));

        for task in tasks.drain(..) {
            results.push(task.execute().await);
        }

        // Check for any errors
        for result in results {
            result?;
        }

        Ok(())
    }
}

/// Task trait for type erasure
trait TaskTrait {
    fn execute(self: Box<Self>) -> Pin<Box<dyn Future<Output = PrismResult<()>> + Send>>;
    fn priority(&self) -> Priority;
}

/// Task wrapper for trait implementation
struct TaskWrapper<F>(Task<F>) where F: Future<Output = PrismResult<()>>;

impl<F> TaskTrait for TaskWrapper<F>
where
    F: Future<Output = PrismResult<()>> + Send + 'static,
{
    fn execute(self: Box<Self>) -> Pin<Box<dyn Future<Output = PrismResult<()>> + Send>> {
        Box::pin(self.0.execute())
    }

    fn priority(&self) -> Priority {
        self.0.context().priority()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[tokio::test]
    async fn test_task_execution() {
        let future = async { Ok(()) };
        let config = TaskConfig::default();
        let task = Task::new(future, config, None);
        
        let result = task.execute().await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_task_timeout() {
        let future = async {
            tokio::time::sleep(Duration::from_millis(100)).await;
            Ok(())
        };

        let config = TaskConfig {
            timeout: Some(Duration::from_millis(50)),
            ..Default::default()
        };

        let task = Task::new(future, config, None);
        let result = task.execute().await;
        
        assert!(matches!(result, Err(PrismError::Timeout)));
    }

    #[tokio::test]
    async fn test_task_executor() {
        let executor = TaskExecutor::new(None);
        
        let future1 = async { Ok(()) };
        let future2 = async { Ok(()) };

        let handle1 = executor.submit(future1, TaskConfig::default()).unwrap();
        let handle2 = executor.submit(future2, TaskConfig {
            priority: Priority::High,
            ..Default::default()
        }).unwrap();

        executor.execute_all().await.unwrap();
    }

    #[tokio::test]
    async fn test_crystal_alignment() {
        let crystal = Arc::new(Crystal::new(crate::crystal::bridge::CrystalSystem::Cubic).unwrap());
        let executor = TaskExecutor::new(Some(Arc::clone(&crystal)));

        let future = async { Ok(()) };
        let config = TaskConfig {
            crystal_alignment: true,
            ..Default::default()
        };

        executor.submit(future, config).unwrap();
        executor.execute_all().await.unwrap();
    }
}
