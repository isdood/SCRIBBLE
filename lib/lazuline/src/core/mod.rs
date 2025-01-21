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
