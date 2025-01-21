// ffi.rs - FFI bindings for Prism runtime
// Created by: isdood
// Date: 2025-01-21 10:59:59 UTC

use std::ffi::{c_void, CStr};
use std::os::raw::{c_char, c_int, c_uint, c_ulonglong};
use std::ptr::NonNull;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};

use crate::types::{PrismError, PrismResult, TaskHandle, TaskStatus};

#[repr(C)]
pub struct PrismRuntime {
    initialized: AtomicBool,
    next_task_id: AtomicU64,
    thread_count: c_uint,
}

#[repr(C)]
pub struct PrismTask {
    id: c_ulonglong,
    status: c_int,
    data: *mut c_void,
    callback: Option<extern "C" fn(*mut PrismTask)>,
}

#[repr(C)]
pub struct PrismConfig {
    thread_count: c_uint,
    stack_size: c_ulonglong,
    use_hardware_threads: bool,
}

#[no_mangle]
pub extern "C" fn prism_runtime_create(config: *const PrismConfig) -> *mut PrismRuntime {
    let config = unsafe { &*config };
    
    let runtime = Box::new(PrismRuntime {
        initialized: AtomicBool::new(true),
        next_task_id: AtomicU64::new(0),
        thread_count: config.thread_count,
    });

    Box::into_raw(runtime)
}

#[no_mangle]
pub extern "C" fn prism_runtime_destroy(runtime: *mut PrismRuntime) {
    if !runtime.is_null() {
        unsafe {
            let _ = Box::from_raw(runtime);
        }
    }
}

#[no_mangle]
pub extern "C" fn prism_task_create(
    runtime: *mut PrismRuntime,
    data: *mut c_void,
    callback: Option<extern "C" fn(*mut PrismTask)>,
) -> *mut PrismTask {
    let runtime = unsafe { &*runtime };
    
    let task = Box::new(PrismTask {
        id: runtime.next_task_id.fetch_add(1, Ordering::SeqCst),
        status: TaskStatus::Ready as c_int,
        data,
        callback,
    });

    Box::into_raw(task)
}

#[no_mangle]
pub extern "C" fn prism_task_destroy(task: *mut PrismTask) {
    if !task.is_null() {
        unsafe {
            let _ = Box::from_raw(task);
        }
    }
}

#[no_mangle]
pub extern "C" fn prism_task_execute(
    runtime: *mut PrismRuntime,
    task: *mut PrismTask,
) -> c_int {
    let runtime = unsafe { &*runtime };
    let task = unsafe { &mut *task };

    if !runtime.initialized.load(Ordering::SeqCst) {
        return PrismError::NotInitialized as c_int;
    }

    task.status = TaskStatus::Running as c_int;
    
    if let Some(callback) = task.callback {
        callback(task);
    }

    task.status = TaskStatus::Completed as c_int;
    PrismResult::Success as c_int
}

#[no_mangle]
pub extern "C" fn prism_task_status(task: *const PrismTask) -> c_int {
    let task = unsafe { &*task };
    task.status
}

#[no_mangle]
pub extern "C" fn prism_task_wait(
    runtime: *mut PrismRuntime,
    task: *const PrismTask,
    timeout_ms: c_ulonglong,
) -> c_int {
    let runtime = unsafe { &*runtime };
    let task = unsafe { &*task };

    if !runtime.initialized.load(Ordering::SeqCst) {
        return PrismError::NotInitialized as c_int;
    }

    let start = std::time::Instant::now();
    let timeout = std::time::Duration::from_millis(timeout_ms as u64);

    while task.status != TaskStatus::Completed as c_int {
        if timeout_ms > 0 && start.elapsed() > timeout {
            return PrismError::Timeout as c_int;
        }
        std::thread::yield_now();
    }

    PrismResult::Success as c_int
}

#[no_mangle]
pub extern "C" fn prism_error_message(error_code: c_int) -> *const c_char {
    let message = match error_code {
        x if x == PrismError::NotInitialized as c_int => "Runtime not initialized",
        x if x == PrismError::Timeout as c_int => "Operation timed out",
        x if x == PrismError::InvalidArgument as c_int => "Invalid argument provided",
        x if x == PrismError::OutOfMemory as c_int => "Out of memory",
        _ => "Unknown error",
    };

    std::ffi::CString::new(message)
        .map(|s| s.into_raw())
        .unwrap_or(std::ptr::null())
}

#[no_mangle]
pub extern "C" fn prism_string_free(ptr: *mut c_char) {
    if !ptr.is_null() {
        unsafe {
            let _ = CStr::from_ptr(ptr);
            let _ = Box::from_raw(ptr);
        }
    }
}

// Safe wrapper functions for internal use
impl PrismRuntime {
    pub fn new(config: PrismConfig) -> PrismResult<NonNull<PrismRuntime>> {
        let runtime = prism_runtime_create(&config);
        NonNull::new(runtime).ok_or(PrismError::OutOfMemory)
    }

    pub fn create_task(&self, data: *mut c_void, callback: Option<extern "C" fn(*mut PrismTask)>) 
        -> PrismResult<NonNull<PrismTask>> {
        let task = prism_task_create(self as *const _ as *mut _, data, callback);
        NonNull::new(task).ok_or(PrismError::OutOfMemory)
    }
}

impl Drop for PrismRuntime {
    fn drop(&mut self) {
        self.initialized.store(false, Ordering::SeqCst);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ptr;

    #[test]
    fn test_runtime_creation() {
        let config = PrismConfig {
            thread_count: 4,
            stack_size: 1024 * 1024,
            use_hardware_threads: true,
        };

        let runtime = prism_runtime_create(&config);
        assert!(!runtime.is_null());

        unsafe {
            let runtime_ref = &*runtime;
            assert!(runtime_ref.initialized.load(Ordering::SeqCst));
            assert_eq!(runtime_ref.thread_count, 4);
        }

        prism_runtime_destroy(runtime);
    }

    #[test]
    fn test_task_execution() {
        let config = PrismConfig {
            thread_count: 1,
            stack_size: 1024 * 1024,
            use_hardware_threads: false,
        };

        let runtime = prism_runtime_create(&config);
        
        extern "C" fn test_callback(task: *mut PrismTask) {
            unsafe {
                (*task).status = TaskStatus::Running as c_int;
            }
        }

        let task = prism_task_create(runtime, ptr::null_mut(), Some(test_callback));
        assert!(!task.is_null());

        let result = prism_task_execute(runtime, task);
        assert_eq!(result, PrismResult::Success as c_int);

        let status = prism_task_status(task);
        assert_eq!(status, TaskStatus::Completed as c_int);

        prism_task_destroy(task);
        prism_runtime_destroy(runtime);
    }

    #[test]
    fn test_task_wait_timeout() {
        let config = PrismConfig {
            thread_count: 1,
            stack_size: 1024 * 1024,
            use_hardware_threads: false,
        };

        let runtime = prism_runtime_create(&config);
        
        extern "C" fn endless_callback(task: *mut PrismTask) {
            unsafe {
                (*task).status = TaskStatus::Running as c_int;
                loop {
                    std::thread::sleep(std::time::Duration::from_millis(100));
                }
            }
        }

        let task = prism_task_create(runtime, ptr::null_mut(), Some(endless_callback));
        let result = prism_task_wait(runtime, task, 50);
        assert_eq!(result, PrismError::Timeout as c_int);

        prism_task_destroy(task);
        prism_runtime_destroy(runtime);
    }
}
