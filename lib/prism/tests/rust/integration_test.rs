// integration_test.rs - Integration tests for Prism runtime
// Created by: isdood
// Date: 2025-01-21 11:14:09 UTC

use std::sync::Arc;
use std::time::Duration;
use tokio::time::sleep;

use prism::crystal::bridge::{Crystal, CrystalSystem};
use prism::crystal::pattern::{PatternGenerator, PatternType, PatternConfig};
use prism::runtime::future::FutureExt;
use prism::runtime::task::{Task, TaskConfig, TaskExecutor};
use prism::types::{Priority, PrismError, PrismResult, TaskStatus};

#[tokio::test]
async fn test_full_task_lifecycle() {
    // Initialize crystal system
    let crystal = Arc::new(Crystal::new(CrystalSystem::Cubic).unwrap());
    let executor = TaskExecutor::new(Some(Arc::clone(&crystal)));

    // Create pattern generator
    let pattern_config = PatternConfig {
        pattern_type: PatternType::Cubic,
        spacing: 1.0,
        scale: 1.0,
        rotation: [0.0; 3],
        symmetry: 8,
    };

    let pattern_generator = PatternGenerator::new(Arc::clone(&crystal), pattern_config);

    // Define task that generates crystal pattern
    let pattern_task = async move {
        pattern_generator.generate()?;
        Ok(())
    };

    // Submit pattern generation task
    let pattern_handle = executor.submit(pattern_task, TaskConfig {
        priority: Priority::High,
        timeout: Some(Duration::from_secs(1)),
        crystal_alignment: true,
        ..Default::default()
    }).unwrap();

    // Execute pattern generation
    executor.execute_all().await.unwrap();

    // Verify pattern creation
    assert!(crystal.stability().unwrap() > 0.0);
}

#[tokio::test]
async fn test_concurrent_pattern_modification() {
    let crystal = Arc::new(Crystal::new(CrystalSystem::Cubic).unwrap());
    let executor = TaskExecutor::new(Some(Arc::clone(&crystal)));

    // Create multiple pattern tasks
    let mut handles = Vec::new();
    for i in 0..3 {
        let crystal = Arc::clone(&crystal);
        let task = async move {
            let config = PatternConfig {
                pattern_type: match i {
                    0 => PatternType::Cubic,
                    1 => PatternType::Hexagonal,
                    _ => PatternType::Grid,
                },
                spacing: 1.0,
                scale: 1.0,
                rotation: [0.0; 3],
                symmetry: 4,
            };

            let generator = PatternGenerator::new(crystal, config);
            generator.generate()?;
            sleep(Duration::from_millis(10)).await;
            Ok(())
        };

        let handle = executor.submit(task, TaskConfig {
            priority: Priority::Normal,
            timeout: Some(Duration::from_secs(1)),
            crystal_alignment: true,
            ..Default::default()
        }).unwrap();

        handles.push(handle);
    }

    // Execute all tasks
    executor.execute_all().await.unwrap();

    // Verify final crystal state
    assert!(crystal.stability().unwrap() > 0.0);
}

#[tokio::test]
async fn test_task_priorities() {
    let crystal = Arc::new(Crystal::new(CrystalSystem::Cubic).unwrap());
    let executor = TaskExecutor::new(Some(Arc::clone(&crystal)));

    let mut completion_order = Vec::new();
    let completion_order = Arc::new(tokio::sync::Mutex::new(completion_order));

    // Create tasks with different priorities
    for priority in [Priority::Low, Priority::High, Priority::Normal] {
        let completion_order = Arc::clone(&completion_order);
        let task = async move {
            sleep(Duration::from_millis(10)).await;
            completion_order.lock().await.push(priority);
            Ok(())
        };

        executor.submit(task, TaskConfig {
            priority,
            ..Default::default()
        }).unwrap();
    }

    // Execute all tasks
    executor.execute_all().await.unwrap();

    // Verify execution order
    let final_order = completion_order.lock().await;
    assert_eq!(final_order[0], Priority::High);
    assert!(final_order[2] == Priority::Low);
}

#[tokio::test]
async fn test_timeout_handling() {
    let executor = TaskExecutor::new(None);

    // Create a long-running task
    let task = async {
        sleep(Duration::from_millis(100)).await;
        Ok(())
    };

    // Submit with short timeout
    let handle = executor.submit(task, TaskConfig {
        timeout: Some(Duration::from_millis(50)),
        ..Default::default()
    }).unwrap();

    // Execute and verify timeout
    let result = executor.execute_all().await;
    assert!(matches!(result, Err(PrismError::Timeout)));
}

#[tokio::test]
async fn test_crystal_pattern_transitions() {
    let crystal = Arc::new(Crystal::new(CrystalSystem::Cubic).unwrap());
    let executor = TaskExecutor::new(Some(Arc::clone(&crystal)));

    // Create a sequence of pattern transitions
    let patterns = vec![
        PatternType::Cubic,
        PatternType::Hexagonal,
        PatternType::Grid,
    ];

    for pattern_type in patterns {
        let crystal = Arc::clone(&crystal);
        let task = async move {
            let config = PatternConfig {
                pattern_type,
                spacing: 1.0,
                scale: 1.0,
                rotation: [0.0; 3],
                symmetry: 4,
            };

            let generator = PatternGenerator::new(crystal, config);
            generator.generate()?;
            
            // Allow time for crystal stabilization
            sleep(Duration::from_millis(20)).await;
            Ok(())
        };

        let handle = executor.submit(task, TaskConfig {
            priority: Priority::Normal,
            crystal_alignment: true,
            ..Default::default()
        }).unwrap();

        executor.execute_all().await.unwrap();
    }

    // Verify final stability
    assert!(crystal.stability().unwrap() > 0.0);
}

#[tokio::test]
async fn test_error_propagation() {
    let executor = TaskExecutor::new(None);

    // Create a task that returns an error
    let task = async {
        Err(PrismError::InvalidArgument)
    };

    let handle = executor.submit(task, TaskConfig::default()).unwrap();
    let result = executor.execute_all().await;
    assert!(matches!(result, Err(PrismError::InvalidArgument)));
}

#[tokio::test]
async fn test_task_chaining() {
    let crystal = Arc::new(Crystal::new(CrystalSystem::Cubic).unwrap());
    let executor = TaskExecutor::new(Some(Arc::clone(&crystal)));

    // Create a chain of tasks
    let task1 = async {
        sleep(Duration::from_millis(10)).await;
        Ok(())
    };

    let task2 = async {
        sleep(Duration::from_millis(10)).await;
        Ok(())
    };

    let chained = task1.chain(|result| async move {
        result?;
        task2.await
    });

    let handle = executor.submit(chained, TaskConfig::default()).unwrap();
    executor.execute_all().await.unwrap();
}

#[tokio::test]
async fn test_stress() {
    let crystal = Arc::new(Crystal::new(CrystalSystem::Cubic).unwrap());
    let executor = TaskExecutor::new(Some(Arc::clone(&crystal)));

    // Submit many concurrent tasks
    for _ in 0..100 {
        let crystal = Arc::clone(&crystal);
        let task = async move {
            let config = PatternConfig {
                pattern_type: PatternType::Cubic,
                spacing: 1.0,
                scale: 1.0,
                rotation: [0.0; 3],
                symmetry: 2,
            };

            let generator = PatternGenerator::new(crystal, config);
            generator.generate()?;
            sleep(Duration::from_millis(1)).await;
            Ok(())
        };

        executor.submit(task, TaskConfig::default()).unwrap();
    }

    // Execute all tasks
    let result = executor.execute_all().await;
    assert!(result.is_ok());
}
