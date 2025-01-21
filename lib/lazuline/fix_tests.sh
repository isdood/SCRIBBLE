#!/bin/bash
# fix_tests.sh
# Created: 2025-01-21 18:55:10
# Author: isdood

echo "[$(date -u '+%Y-%m-%d %H:%M:%S UTC')] Updating test infrastructure..."

# Update parallel test module to use common utilities
cat > tests/parallel/mod.rs << 'END'
use crate::common::{setup, create_test_data};

#[test]
fn test_parallel_processing() {
    let mut instance = setup();
    let data = create_test_data(1000);
    let result = instance.channel_compute(&data).unwrap();
    assert_eq!(result.len(), data.len());
}

#[test]
fn test_large_data_processing() {
    let mut instance = setup();
    let data = create_test_data(1_000_000);
    let result = instance.channel_compute(&data).unwrap();
    assert_eq!(result, vec![2.0f64; 1_000_000]);
}
END

# Update integration tests to use common utilities
cat > tests/integration/mod.rs << 'END'
use crate::common::{setup, create_test_data, teardown};

#[test]
fn test_basic_initialization() {
    let instance = setup();
    assert!(instance.is_initialized());
    teardown(instance);
}

#[test]
fn test_channel_compute() {
    let mut instance = setup();
    let data = create_test_data(10);
    let result = instance.channel_compute(&data).unwrap();
    assert_eq!(result, vec![2.0f64; 10]);
    teardown(instance);
}

#[test]
fn test_empty_data() {
    let mut instance = setup();
    let data = Vec::<f64>::new();
    let result = instance.channel_compute(&data).unwrap();
    assert!(result.is_empty());
    teardown(instance);
}
END

# Update core implementation with documentation tests
cat > src/core/mod.rs << 'END'
//! Core implementation of the Lazuline library
//!
//! This module provides the main functionality for processing data channels.

use std::error::Error as StdError;

/// Main struct for the Lazuline library
///
/// # Examples
///
/// ```
/// use lazuline::Lazuline;
///
/// let mut instance = Lazuline::new().unwrap();
/// assert!(instance.is_initialized());
///
/// let data = vec![1.0, 2.0, 3.0];
/// let result = instance.channel_compute(&data).unwrap();
/// assert_eq!(result, vec![2.0, 4.0, 6.0]);
/// ```
#[derive(Debug)]
pub struct Lazuline {
    pub(crate) initialized: bool,
    channels: Vec<f64>,
}

impl Lazuline {
    /// Creates a new instance of Lazuline
    ///
    /// # Examples
    ///
    /// ```
    /// use lazuline::Lazuline;
    ///
    /// let instance = Lazuline::new().unwrap();
    /// assert!(instance.is_initialized());
    /// ```
    pub fn new() -> Result<Self, Box<dyn StdError>> {
        Ok(Self {
            initialized: true,
            channels: Vec::new(),
        })
    }

    /// Checks if the instance is initialized
    pub fn is_initialized(&self) -> bool {
        self.initialized
    }

    /// Computes channel data by doubling each value
    ///
    /// # Examples
    ///
    /// ```
    /// use lazuline::Lazuline;
    ///
    /// let mut instance = Lazuline::new().unwrap();
    /// let data = vec![1.0, 2.0, 3.0];
    /// let result = instance.channel_compute(&data).unwrap();
    /// assert_eq!(result, vec![2.0, 4.0, 6.0]);
    /// ```
    pub fn channel_compute(&mut self, data: &[f64]) -> Result<Vec<f64>, Box<dyn StdError>> {
        self.channels = data.to_vec();
        Ok(self.channels.iter().map(|x| x * 2.0).collect())
    }
}

pub type Error = Box<dyn StdError>;
END

# Create main test file with common module usage
cat > tests/lib.rs << 'END'
#![cfg(test)]

use crate::common::{setup, create_test_data, teardown};

mod common;
mod integration;
mod parallel;

#[test]
fn test_create_instance() {
    let instance = setup();
    assert!(instance.is_initialized());
    teardown(instance);
}

#[test]
fn test_multiple_computations() {
    let mut instance = setup();

    // First computation
    let data1 = create_test_data(5);
    let result1 = instance.channel_compute(&data1).unwrap();
    assert_eq!(result1, vec![2.0f64; 5]);

    // Second computation
    let data2 = vec![2.0f64; 3];
    let result2 = instance.channel_compute(&data2).unwrap();
    assert_eq!(result2, vec![4.0f64; 3]);

    teardown(instance);
}
END

echo "[$(date -u '+%Y-%m-%d %H:%M:%S UTC')] ✨ Test infrastructure updated!"
echo "Run 'cargo test' to verify."
