//! Native marker type for crystal-space path tracking.
//!
//! This module provides a high-performance marker type optimized for
//! tracking computation paths through crystal-space resonance fields.

pub mod crystal;
pub mod trace;

use std::sync::Arc;
use std::hash::{Hash, Hasher};
use crystal::Crystal;
use trace::Trace;

/// Result type for mark operations
pub type MarkResult<T> = Result<T, MarkError>;

/// Error type for mark operations
#[derive(Debug)]
pub enum MarkError {
    /// Crystal resonance error
    ResonanceError(String),
    /// Path tracking error
    PathError(String),
    /// Marker collision
    CollisionError(String),
}

impl From<String> for MarkError {
    fn from(error: String) -> Self {
        MarkError::PathError(error)
    }
}

/// Native marker for crystal-space path tracking
#[derive(Debug, Clone)]
pub struct Mark {
    /// Unique identifier
    id: u64,
    /// Crystal field
    crystal: Arc<Crystal>,
    /// Path trace
    trace: Arc<Trace>,
    /// Marker data
    data: Vec<u8>,
}

impl Mark {
    /// Creates a new marker
    pub fn new(data: impl AsRef<[u8]>) -> Self {
        let data = data.as_ref().to_vec();
        let id = {
            use std::collections::hash_map::DefaultHasher;
            let mut hasher = DefaultHasher::new();
            data.hash(&mut hasher);
            hasher.finish()
        };

        Self {
            id,
            crystal: Arc::new(Crystal::default()),
            trace: Arc::new(Trace::default()),
            data,
        }
    }

    /// Gets the marker's unique ID
    pub fn id(&self) -> u64 {
        self.id
    }

    /// Gets the marker's crystal field
    pub fn crystal(&self) -> &Crystal {
        &self.crystal
    }

    /// Gets the marker's path trace
    pub fn trace(&self) -> &Trace {
        &self.trace
    }

    /// Gets the marker's data
    pub fn data(&self) -> &[u8] {
        &self.data
    }

    /// Places a marker in crystal space
    pub fn place(&self, position: impl Into<[f64; 3]>) -> MarkResult<()> {
        let pos = position.into();
        self.crystal.resonate(pos)?;
        self.trace.record(pos)?;
        Ok(())
    }

    /// Moves a marker through crystal space
    pub fn shift(&self, offset: impl Into<[f64; 3]>) -> MarkResult<()> {
        let off = offset.into();
        self.crystal.shift(off)?;
        self.trace.extend(off)?;
        Ok(())
    }

    /// Checks for marker collision
    pub fn collides_with(&self, other: &Self) -> MarkResult<bool> {
        if self.crystal.interferes_with(&other.crystal)? {
            if self.trace.intersects(&other.trace)? {
                Ok(true)
            } else {
                Ok(false)
            }
        } else {
            Ok(false)
        }
    }

    /// Merges two markers
    pub fn merge(&self, other: &Self) -> MarkResult<Self> {
        if self.collides_with(other)? {
            return Err(MarkError::CollisionError(
                "Cannot merge colliding markers".to_string(),
            ));
        }

        let mut data = self.data.clone();
        data.extend(&other.data);

        let mut mark = Self::new(data);
        mark.crystal = Arc::new(self.crystal.merge(&other.crystal)?);
        mark.trace = Arc::new(self.trace.merge(&other.trace)?);

        Ok(mark)
    }
}

impl PartialEq for Mark {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for Mark {}

impl Hash for Mark {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}
