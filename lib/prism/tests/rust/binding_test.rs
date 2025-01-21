// binding_test.rs - Tests for Rust-Zig FFI bindings
// Created by: isdood
// Date: 2025-01-21 11:12:40 UTC

use std::sync::Arc;
use std::time::Duration;

use prism::binding::ffi::{PrismConfig, PrismRuntime, PrismTask};
use prism::crystal::bridge::{Crystal, CrystalSystem};
use prism::types::{PrismError, PrismResult, TaskHandle, TaskStatus};

#[test]
fn test_runtime_creation() {
    let config = PrismConfig {
        thread_count: 4,
        stack_size: 1024 * 1024,
        use_hardware_threads: true,
    };

    unsafe {
        let runtime = prism_runtime_create(&config);
        assert!(!runtime.is_null());
        prism_runtime_destroy(runtime);
    }
}

#[test]
fn test_task_lifecycle() {
    let config = PrismConfig {
        thread_count: 1,
        stack_size: 1024 * 1024,
        use_hardware_threads: false,
    };

    unsafe {
        let runtime = prism_runtime_create(&config);
        assert!(!runtime.is_null());

        // Create task
        extern "C" fn test_callback(task: *mut PrismTask) {
            unsafe {
                (*task).status = TaskStatus::Running as i32;
            }
        }

        let task = prism_task_create(runtime, std::ptr::null_mut(), Some(test_callback));
        assert!(!task.is_null());

        // Execute task
        let result = prism_task_execute(runtime, task);
        assert_eq!(result, 0); // Success

        // Check status
        let status = prism_task_status(task);
        assert_eq!(status, TaskStatus::Completed as i32);

        // Cleanup
        prism_task_destroy(task);
        prism_runtime_destroy(runtime);
    }
}

#[test]
fn test_crystal_integration() {
    let config = PrismConfig {
        thread_count: 1,
        stack_size: 1024 * 1024,
        use_hardware_threads: false,
    };

    unsafe {
        let runtime = prism_runtime_create(&config);
        let crystal = Arc::new(Crystal::new(CrystalSystem::Cubic).unwrap());

        extern "C" fn crystal_callback(task: *mut PrismTask) {
            unsafe {
                let crystal_ptr = (*task).data as *mut Crystal;
                let crystal = &*(crystal_ptr);
                crystal.optimize().unwrap();
                (*task).status = TaskStatus::Completed as i32;
            }
        }

        let crystal_ptr = Arc::into_raw(crystal.clone()) as *mut _;
        let task = prism_task_create(runtime, crystal_ptr as *mut _, Some(crystal_callback));
        
        let result = prism_task_execute(runtime, task);
        assert_eq!(result, 0);

        // Cleanup
        prism_task_destroy(task);
        prism_runtime_destroy(runtime);
        drop(Arc::from_raw(crystal_ptr));
    }
}

#[test]
fn test_error_handling() {
    unsafe {
        // Test invalid runtime
        let result = prism_task_execute(std::ptr::null_mut(), std::ptr::null_mut());
        assert_ne!(result, 0);

        // Test error message
        let error_msg = prism_error_message(PrismError::NotInitialized as i32);
        assert!(!error_msg.is_null());
        
        let c_str = std::ffi::CStr::from_ptr(error_msg);
        assert_eq!(c_str.to_str().unwrap(), "Runtime not initialized");
        
        prism_string_free(error_msg as *mut _);
    }
}

#[test]
fn test_task_timeout() {
    let config = PrismConfig {
        thread_count: 1,
        stack_size: 1024 * 1024,
        use_hardware_threads: false,
    };

    unsafe {
        let runtime = prism_runtime_create(&config);

        extern "C" fn timeout_callback(task: *mut PrismTask) {
            std::thread::sleep(Duration::from_millis(100));
            unsafe {
                (*task).status = TaskStatus::Completed as i32;
            }
        }

        let task = prism_task_create(runtime, std::ptr::null_mut(), Some(timeout_callback));
        let result = prism_task_wait(runtime, task, 50); // 50ms timeout
        assert_eq!(result, PrismError::Timeout as i32);

        prism_task_destroy(task);
        prism_runtime_destroy(runtime);
    }
}

#[test]
fn test_concurrent_tasks() {
    let config = PrismConfig {
        thread_count: 4,
        stack_size: 1024 * 1024,
        use_hardware_threads: true,
    };

    unsafe {
        let runtime = prism_runtime_create(&config);
        let mut tasks = Vec::new();

        extern "C" fn concurrent_callback(task: *mut PrismTask) {
            std::thread::sleep(Duration::from_millis(10));
            unsafe {
                (*task).status = TaskStatus::Completed as i32;
            }
        }

        // Create multiple tasks
        for _ in 0..10 {
            let task = prism_task_create(runtime, std::ptr::null_mut(), Some(concurrent_callback));
            tasks.push(task);
        }

        // Execute all tasks
        let mut handles = Vec::new();
        for task in &tasks {
            let runtime_ptr = runtime;
            let task_ptr = *task;
            handles.push(std::thread::spawn(move || {
                prism_task_execute(runtime_ptr, task_ptr)
            }));
        }

        // Wait for completion
        for handle in handles {
            assert_eq!(handle.join().unwrap(), 0);
        }

        // Cleanup
        for task in tasks {
            prism_task_destroy(task);
        }
        prism_runtime_destroy(runtime);
    }
}

#[test]
fn test_crystal_pattern_integration() {
    let config = PrismConfig {
        thread_count: 1,
        stack_size: 1024 * 1024,
        use_hardware_threads: false,
    };

    unsafe {
        let runtime = prism_runtime_create(&config);
        let crystal = Arc::new(Crystal::new(CrystalSystem::Cubic).unwrap());

        // Test pattern creation
        extern "C" fn pattern_callback(task: *mut PrismTask) {
            unsafe {
                let crystal_ptr = (*task).data as *mut Crystal;
                let crystal = &*(crystal_ptr);
                
                // Create a simple cubic pattern
                for i in 0..3 {
                    for j in 0..3 {
                        for k in 0..3 {
                            let position = [i as f64, j as f64, k as f64];
                            crystal.add_node(position).unwrap();
                        }
                    }
                }
                
                crystal.optimize().unwrap();
                (*task).status = TaskStatus::Completed as i32;
            }
        }

        let crystal_ptr = Arc::into_raw(crystal.clone()) as *mut _;
        let task = prism_task_create(runtime, crystal_ptr as *mut _, Some(pattern_callback));
        
        let result = prism_task_execute(runtime, task);
        assert_eq!(result, 0);

        // Verify pattern creation
        let crystal = unsafe { Arc::from_raw(crystal_ptr) };
        assert!(crystal.stability().unwrap() > 0.0);

        prism_task_destroy(task);
        prism_runtime_destroy(runtime);
    }
}

#[test]
#[should_panic]
fn test_invalid_operations() {
    unsafe {
        let null_runtime: *mut PrismRuntime = std::ptr::null_mut();
        prism_runtime_destroy(null_runtime);
    }
}
