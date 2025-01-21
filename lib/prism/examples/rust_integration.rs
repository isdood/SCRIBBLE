// rust_integration.rs - Rust-Zig integration layer for Prism
// Created by: isdood
// Date: 2025-01-21 11:24:58 UTC

use std::ffi::{c_void, CStr, CString};
use std::os::raw::c_char;
use std::sync::Arc;

use crate::crystal::bridge::{Crystal, CrystalSystem};
use crate::runtime::task::{Task, TaskConfig, TaskExecutor};
use crate::types::{PrismError, PrismResult, Priority, TaskStatus};

#[repr(C)]
pub struct PrismRuntime {
    executor: TaskExecutor,
    crystal: Option<Arc<Crystal>>,
}

#[repr(C)]
pub struct PrismTask {
    handle: usize,
    status: i32,
    data: *mut c_void,
    callback: Option<extern "C" fn(*mut PrismTask)>,
}

#[repr(C)]
pub struct PrismConfig {
    pub thread_count: u32,
    pub stack_size: usize,
    pub use_hardware_threads: bool,
}

#[no_mangle]
pub unsafe extern "C" fn prism_runtime_create(config: *const PrismConfig) -> *mut PrismRuntime {
    let config = &*config;
    let crystal = match Crystal::new(CrystalSystem::Cubic) {
        Ok(c) => Some(Arc::new(c)),
        Err(_) => None,
    };

    let executor = match TaskExecutor::new(crystal.as_ref().map(Arc::clone)) {
        Ok(e) => e,
        Err(_) => return std::ptr::null_mut(),
    };

    let runtime = Box::new(PrismRuntime {
        executor,
        crystal,
    });

    Box::into_raw(runtime)
}

#[no_mangle]
pub unsafe extern "C" fn prism_runtime_destroy(runtime: *mut PrismRuntime) {
    if !runtime.is_null() {
        drop(Box::from_raw(runtime));
    }
}

#[no_mangle]
pub unsafe extern "C" fn prism_task_create(
    runtime: *mut PrismRuntime,
    data: *mut c_void,
    callback: Option<extern "C" fn(*mut PrismTask)>,
) -> *mut PrismTask {
    let runtime = &mut *runtime;
    
    let task = Box::new(PrismTask {
        handle: 0,
        status: TaskStatus::Ready as i32,
        data,
        callback,
    });

    Box::into_raw(task)
}

#[no_mangle]
pub unsafe extern "C" fn prism_task_destroy(task: *mut PrismTask) {
    if !task.is_null() {
        drop(Box::from_raw(task));
    }
}

#[no_mangle]
pub unsafe extern "C" fn prism_task_execute(
    runtime: *mut PrismRuntime,
    task: *mut PrismTask,
) -> i32 {
    let runtime = &mut *runtime;
    let task = &mut *task;

    let config = TaskConfig {
        priority: Priority::Normal,
        timeout: None,
        crystal_alignment: true,
        ..Default::default()
    };

    let future = async move {
        if let Some(callback) = task.callback {
            callback(task);
        }
        Ok(())
    };

    match runtime.executor.submit(future, config) {
        Ok(handle) => {
            task.handle = handle.into();
            0
        }
        Err(_) => PrismError::TaskCreationFailed as i32,
    }
}

#[no_mangle]
pub unsafe extern "C" fn prism_task_status(task: *const PrismTask) -> i32 {
    if task.is_null() {
        return TaskStatus::Failed as i32;
    }
    (*task).status
}

#[no_mangle]
pub unsafe extern "C" fn prism_task_wait(
    runtime: *mut PrismRuntime,
    task: *const PrismTask,
    timeout_ms: u32,
) -> i32 {
    let runtime = &mut *runtime;
    let task = &*task;

    let future = async move {
        let handle = task.handle;
        runtime.executor.wait_for_task(handle.into()).await
    };

    match runtime.executor.block_on(future) {
        Ok(_) => 0,
        Err(e) => e as i32,
    }
}

#[no_mangle]
pub unsafe extern "C" fn prism_crystal_create(
    system: i32,
) -> *mut Crystal {
    let system = match system {
        0 => CrystalSystem::Cubic,
        1 => CrystalSystem::Hexagonal,
        2 => CrystalSystem::Tetragonal,
        _ => return std::ptr::null_mut(),
    };

    match Crystal::new(system) {
        Ok(crystal) => Box::into_raw(Box::new(crystal)),
        Err(_) => std::ptr::null_mut(),
    }
}

#[no_mangle]
pub unsafe extern "C" fn prism_crystal_destroy(crystal: *mut Crystal) {
    if !crystal.is_null() {
        drop(Box::from_raw(crystal));
    }
}

#[no_mangle]
pub unsafe extern "C" fn prism_crystal_optimize(
    crystal: *mut Crystal,
) -> i32 {
    let crystal = &mut *crystal;
    match crystal.optimize() {
        Ok(_) => 0,
        Err(e) => e as i32,
    }
}

#[no_mangle]
pub unsafe extern "C" fn prism_error_message(
    error_code: i32,
) -> *const c_char {
    let message = match PrismError::from(error_code) {
        PrismError::NotInitialized => "Runtime not initialized",
        PrismError::TaskCreationFailed => "Task creation failed",
        PrismError::Timeout => "Operation timed out",
        PrismError::InvalidArgument => "Invalid argument provided",
        _ => "Unknown error",
    };

    let c_str = match CString::new(message) {
        Ok(s) => s,
        Err(_) => return std::ptr::null(),
    };

    c_str.into_raw()
}

#[no_mangle]
pub unsafe extern "C" fn prism_string_free(s: *mut c_char) {
    if !s.is_null() {
        drop(CString::from_raw(s));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ptr;

    #[test]
    fn test_runtime_lifecycle() {
        unsafe {
            let config = PrismConfig {
                thread_count: 4,
                stack_size: 1024 * 1024,
                use_hardware_threads: true,
            };

            let runtime = prism_runtime_create(&config);
            assert!(!runtime.is_null());

            prism_runtime_destroy(runtime);
        }
    }

    #[test]
    fn test_task_execution() {
        unsafe {
            let config = PrismConfig {
                thread_count: 1,
                stack_size: 1024 * 1024,
                use_hardware_threads: false,
            };

            let runtime = prism_runtime_create(&config);
            assert!(!runtime.is_null());

            extern "C" fn test_callback(task: *mut PrismTask) {
                unsafe {
                    (*task).status = TaskStatus::Completed as i32;
                }
            }

            let task = prism_task_create(runtime, ptr::null_mut(), Some(test_callback));
            assert!(!task.is_null());

            let result = prism_task_execute(runtime, task);
            assert_eq!(result, 0);

            prism_task_destroy(task);
            prism_runtime_destroy(runtime);
        }
    }

    #[test]
    fn test_crystal_operations() {
        unsafe {
            let crystal = prism_crystal_create(0); // Cubic system
            assert!(!crystal.is_null());

            let result = prism_crystal_optimize(crystal);
            assert_eq!(result, 0);

            prism_crystal_destroy(crystal);
        }
    }

    #[test]
    fn test_error_handling() {
        unsafe {
            let error_msg = prism_error_message(PrismError::NotInitialized as i32);
            assert!(!error_msg.is_null());

            let c_str = CStr::from_ptr(error_msg);
            assert_eq!(c_str.to_str().unwrap(), "Runtime not initialized");

            prism_string_free(error_msg as *mut c_char);
        }
    }
}
