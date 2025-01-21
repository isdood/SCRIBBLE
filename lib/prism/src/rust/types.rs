// src/types.rs - Core types for Prism runtime
// Created by: isdood
// Date: 2025-01-21 11:01:25 UTC

use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Mutex};
use std::time::Duration;

/// Result type for Prism operations
pub type PrismResult<T> = Result<T, PrismError>;

/// Task handle for referencing tasks
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub struct TaskHandle(u64);

static NEXT_TASK_ID: AtomicU64 = AtomicU64::new(0);

impl TaskHandle {
    /// Create a new unique task handle
    pub fn new() -> Self {
        Self(NEXT_TASK_ID.fetch_add(1, Ordering::SeqCst))
    }

    /// Get the raw ID of the task handle
    pub fn raw_id(&self) -> u64 {
        self.0
    }
}

impl Default for TaskHandle {
    fn default() -> Self {
        Self::new()
    }
}

/// Task priority levels
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u8)]
pub enum Priority {
    Low = 0,
    Normal = 128,
    High = 192,
    Critical = 255,
}

impl Default for Priority {
    fn default() -> Self {
        Self::Normal
    }
}

/// Task execution status
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum TaskStatus {
    Ready = 0,
    Running = 1,
    Waiting = 2,
    Completed = 3,
    Failed = 4,
    Cancelled = 5,
}

impl Default for TaskStatus {
    fn default() -> Self {
        Self::Ready
    }
}

/// Error types for Prism operations
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum PrismError {
    Success = 0,
    NotInitialized = -1,
    AlreadyInitialized = -2,
    InvalidArgument = -3,
    OutOfMemory = -4,
    Timeout = -5,
    TaskNotFound = -6,
    InvalidState = -7,
    SystemError = -8,
}

impl std::error::Error for PrismError {}

impl std::fmt::Display for PrismError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PrismError::Success => write!(f, "Operation successful"),
            PrismError::NotInitialized => write!(f, "Runtime not initialized"),
            PrismError::AlreadyInitialized => write!(f, "Runtime already initialized"),
            PrismError::InvalidArgument => write!(f, "Invalid argument provided"),
            PrismError::OutOfMemory => write!(f, "Out of memory"),
            PrismError::Timeout => write!(f, "Operation timed out"),
            PrismError::TaskNotFound => write!(f, "Task not found"),
            PrismError::InvalidState => write!(f, "Invalid state"),
            PrismError::SystemError => write!(f, "System error occurred"),
        }
    }
}

/// Task configuration
#[derive(Debug, Clone)]
pub struct TaskConfig {
    pub priority: Priority,
    pub timeout: Option<Duration>,
    pub stack_size: Option<usize>,
}

impl Default for TaskConfig {
    fn default() -> Self {
        Self {
            priority: Priority::default(),
            timeout: None,
            stack_size: None,
        }
    }
}

/// Task metadata
#[derive(Debug)]
pub struct TaskMetadata {
    pub handle: TaskHandle,
    pub status: TaskStatus,
    pub priority: Priority,
    pub creation_time: std::time::Instant,
    pub start_time: Option<std::time::Instant>,
    pub completion_time: Option<std::time::Instant>,
}

impl TaskMetadata {
    pub fn new(handle: TaskHandle, priority: Priority) -> Self {
        Self {
            handle,
            status: TaskStatus::default(),
            priority,
            creation_time: std::time::Instant::now(),
            start_time: None,
            completion_time: None,
        }
    }

    pub fn duration(&self) -> Option<Duration> {
        match (self.start_time, self.completion_time) {
            (Some(start), Some(end)) => Some(end.duration_since(start)),
            _ => None,
        }
    }
}

/// Task state container
#[derive(Debug)]
pub struct TaskState {
    metadata: Arc<Mutex<TaskMetadata>>,
    result: Arc<Mutex<Option<PrismResult<()>>>>,
}

impl TaskState {
    pub fn new(handle: TaskHandle, priority: Priority) -> Self {
        Self {
            metadata: Arc::new(Mutex::new(TaskMetadata::new(handle, priority))),
            result: Arc::new(Mutex::new(None)),
        }
    }

    pub fn set_status(&self, status: TaskStatus) {
        let mut metadata = self.metadata.lock().unwrap();
        metadata.status = status;
        match status {
            TaskStatus::Running => metadata.start_time = Some(std::time::Instant::now()),
            TaskStatus::Completed | TaskStatus::Failed | TaskStatus::Cancelled => {
                metadata.completion_time = Some(std::time::Instant::now())
            }
            _ => {}
        }
    }

    pub fn set_result(&self, result: PrismResult<()>) {
        *self.result.lock().unwrap() = Some(result);
    }

    pub fn get_result(&self) -> Option<PrismResult<()>> {
        self.result.lock().unwrap().clone()
    }
}

/// Runtime statistics
#[derive(Debug, Default)]
pub struct RuntimeStats {
    pub tasks_created: u64,
    pub tasks_completed: u64,
    pub tasks_failed: u64,
    pub tasks_cancelled: u64,
    pub total_execution_time: Duration,
}

impl RuntimeStats {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn update(&mut self, metadata: &TaskMetadata) {
        match metadata.status {
            TaskStatus::Completed => self.tasks_completed += 1,
            TaskStatus::Failed => self.tasks_failed += 1,
            TaskStatus::Cancelled => self.tasks_cancelled += 1,
            _ => {}
        }

        if let Some(duration) = metadata.duration() {
            self.total_execution_time += duration;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;

    #[test]
    fn test_task_handle_uniqueness() {
        let handle1 = TaskHandle::new();
        let handle2 = TaskHandle::new();
        assert_ne!(handle1, handle2);
    }

    #[test]
    fn test_task_metadata_timing() {
        let handle = TaskHandle::new();
        let mut metadata = TaskMetadata::new(handle, Priority::Normal);

        metadata.start_time = Some(std::time::Instant::now());
        thread::sleep(Duration::from_millis(10));
        metadata.completion_time = Some(std::time::Instant::now());

        assert!(metadata.duration().unwrap().as_millis() >= 10);
    }

    #[test]
    fn test_task_state() {
        let handle = TaskHandle::new();
        let state = TaskState::new(handle, Priority::High);

        state.set_status(TaskStatus::Running);
        state.set_result(Ok(()));

        assert_eq!(state.get_result(), Some(Ok(())));
    }

    #[test]
    fn test_runtime_stats() {
        let mut stats = RuntimeStats::new();
        let handle = TaskHandle::new();
        let mut metadata = TaskMetadata::new(handle, Priority::Normal);

        metadata.status = TaskStatus::Completed;
        metadata.start_time = Some(std::time::Instant::now());
        thread::sleep(Duration::from_millis(10));
        metadata.completion_time = Some(std::time::Instant::now());

        stats.update(&metadata);
        assert_eq!(stats.tasks_completed, 1);
        assert!(stats.total_execution_time.as_millis() >= 10);
    }
}
