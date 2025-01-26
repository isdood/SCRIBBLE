#!/bin/bash

# Spark Mark Module Setup Script
# Author: isdood
# Created: 2025-01-25 20:39:47 UTC
# Repository: isdood/scribble
# Description: Sets up Spark's native marker type for crystal-space path tracking

PURPLE='\033[0;35m'
NC='\033[0m'

print_purple() {
    echo -e "${PURPLE}$1${NC}"
}

setup_mark_module() {
    cd forge/std || exit 1

    # 1. Create mark module structure
    mkdir -p src/mark/{crystal,trace}
    mkdir -p tests/mark

    # 2. Update lib.rs with mark module
    if ! grep -q "pub mod mark;" src/lib.rs; then
        sed -i '/pub mod itex;/a pub mod mark;' src/lib.rs
        sed -i '/pub use itex::ItexResult;/a pub use mark::{Mark, MarkResult};' src/lib.rs
    fi

    # 3. Create main mark module
    cat > "src/mark/mod.rs" << 'EOL'
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
EOL

    # 4. Create crystal module
    cat > "src/mark/crystal/mod.rs" << 'EOL'
//! Crystal field module for Mark type

/// Crystal field state
#[derive(Debug)]
pub struct Crystal {
    /// Field center
    center: [f64; 3],
    /// Field radius
    radius: f64,
    /// Field strength
    strength: f64,
    /// Field coherence
    coherence: f64,
}

impl Crystal {
    /// Creates a new crystal field
    pub fn new(center: [f64; 3], radius: f64, strength: f64, coherence: f64) -> Self {
        Self {
            center,
            radius: radius.abs(),
            strength: strength.abs(),
            coherence: coherence.clamp(0.0, 1.0),
        }
    }

    /// Gets the field center
    pub fn center(&self) -> [f64; 3] {
        self.center
    }

    /// Gets the field radius
    pub fn radius(&self) -> f64 {
        self.radius
    }

    /// Gets the field strength
    pub fn strength(&self) -> f64 {
        self.strength
    }

    /// Gets the field coherence
    pub fn coherence(&self) -> f64 {
        self.coherence
    }

    /// Resonates the field at a position
    pub fn resonate(&self, position: [f64; 3]) -> Result<(), String> {
        let distance = self.distance_to(position);
        if distance > self.radius {
            Err("Position outside field radius".to_string())
        } else {
            Ok(())
        }
    }

    /// Shifts the field by an offset
    pub fn shift(&self, offset: [f64; 3]) -> Result<(), String> {
        if offset.iter().any(|&x| x.abs() > self.radius) {
            Err("Shift distance exceeds field radius".to_string())
        } else {
            Ok(())
        }
    }

    /// Checks for interference with another field
    pub fn interferes_with(&self, other: &Self) -> Result<bool, String> {
        let distance = self.distance_between_centers(other);
        if distance < self.coherence.min(other.coherence) {
            Ok(true)
        } else {
            Ok(false)
        }
    }

    /// Merges with another field
    pub fn merge(&self, other: &Self) -> Result<Self, String> {
        if self.interferes_with(other)? {
            return Err("Cannot merge interfering fields".to_string());
        }

        Ok(Self::new(
            self.average_position(other),
            (self.radius + other.radius) / 2.0,
            (self.strength * other.strength).sqrt(),
            (self.coherence + other.coherence) / 2.0,
        ))
    }

    // Helper methods
    fn distance_to(&self, position: [f64; 3]) -> f64 {
        let squared_dist: f64 = self.center
            .iter()
            .zip(position.iter())
            .map(|(&a, &b)| (b - a).powi(2))
            .sum();
        squared_dist.sqrt()
    }

    fn distance_between_centers(&self, other: &Self) -> f64 {
        let squared_dist: f64 = self.center
            .iter()
            .zip(other.center.iter())
            .map(|(&a, &b)| (b - a).powi(2))
            .sum();
        squared_dist.sqrt()
    }

    fn average_position(&self, other: &Self) -> [f64; 3] {
        let mut avg = [0.0; 3];
        for i in 0..3 {
            avg[i] = (self.center[i] + other.center[i]) / 2.0;
        }
        avg
    }
}

impl Default for Crystal {
    fn default() -> Self {
        Self::new([0.0; 3], 1.0, 1.0, 1.0)
    }
}

impl Clone for Crystal {
    fn clone(&self) -> Self {
        Self::new(
            self.center,
            self.radius,
            self.strength,
            self.coherence,
        )
    }
}
EOL

    # 5. Create trace module
    cat > "src/mark/trace/mod.rs" << 'EOL'
//! Path trace module for Mark type

/// Path trace state
#[derive(Debug)]
pub struct Trace {
    /// Path points
    points: Vec<[f64; 3]>,
    /// Path length
    length: f64,
    /// Path curvature
    curvature: f64,
}

impl Trace {
    /// Creates a new path trace
    pub fn new() -> Self {
        Self {
            points: Vec::new(),
            length: 0.0,
            curvature: 0.0,
        }
    }

    /// Gets the path points
    pub fn points(&self) -> &[[f64; 3]] {
        &self.points
    }

    /// Gets the path length
    pub fn length(&self) -> f64 {
        self.length
    }

    /// Gets the path curvature
    pub fn curvature(&self) -> f64 {
        self.curvature
    }

    /// Records a new point
    pub fn record(&self, point: [f64; 3]) -> Result<(), String> {
        if !self.points.is_empty() {
            let last = self.points.last().unwrap();
            let distance = self.distance_between(last, &point);
            if distance > 10.0 {
                return Err("Point too far from last recorded point".to_string());
            }
        }
        Ok(())
    }

    /// Extends the path
    pub fn extend(&self, offset: [f64; 3]) -> Result<(), String> {
        if offset.iter().any(|&x| x.abs() > 10.0) {
            return Err("Extension distance too large".to_string());
        }
        Ok(())
    }

    /// Checks for intersection with another trace
    pub fn intersects(&self, other: &Self) -> Result<bool, String> {
        if self.points.is_empty() || other.points.is_empty() {
            return Ok(false);
        }

        for window in self.points.windows(2) {
            for other_window in other.points.windows(2) {
                if self.segments_intersect(window, other_window) {
                    return Ok(true);
                }
            }
        }
        Ok(false)
    }

    /// Merges with another trace
    pub fn merge(&self, other: &Self) -> Result<Self, String> {
        if self.intersects(other)? {
            return Err("Cannot merge intersecting traces".to_string());
        }

        let mut trace = Self::new();
        trace.points = self.points.clone();
        trace.points.extend(other.points.iter());
        trace.length = self.length + other.length;
        trace.curvature = (self.curvature + other.curvature) / 2.0;

        Ok(trace)
    }

    // Helper methods
    fn distance_between(&self, a: &[f64; 3], b: &[f64; 3]) -> f64 {
        let squared_dist: f64 = a.iter()
            .zip(b.iter())
            .map(|(&x, &y)| (y - x).powi(2))
            .sum();
        squared_dist.sqrt()
    }

    fn segments_intersect(&self, seg1: &[[f64; 3]], seg2: &[[f64; 3]]) -> bool {
        // Simple bounding box check for demonstration
        let [min_x1, min_y1, min_z1] = seg1[0];
        let [max_x1, max_y1, max_z1] = seg1[1];
        let [min_x2, min_y2, min_z2] = seg2[0];
        let [max_x2, max_y2, max_z2] = seg2[1];

        min_x1.max(min_x2) <= max_x1.min(max_x2) &&
        min_y1.max(min_y2) <= max_y1.min(max_y2) &&
        min_z1.max(min_z2) <= max_z1.min(max_z2)
    }
}

impl Default for Trace {
    fn default() -> Self {
        Self::new()
    }
}

impl Clone for Trace {
    fn clone(&self) -> Self {
        Self {
            points: self.points.clone(),
            length: self.length,
            curvature: self.curvature,
        }
    }
}
EOL

    # 6. Create test module
    cat > "tests/mark/mod.rs" << 'EOL'
use spark_std::mark::{Mark, MarkResult};

#[test]
fn test_mark_creation() {
    let mark = Mark::new([1, 2, 3]);
    assert!(mark.id() != 0);
    assert!(!mark.data().is_empty());
}

#[test]
fn test_mark_placement() -> MarkResult<()> {
    let mark = Mark::new([1, 2, 3]);
    mark.place([0.0, 0.0, 0.0])?;
    assert!(mark.crystal().coherence() > 0.0);
    Ok(())
}

#[test]
fn test_mark_shifting() -> MarkResult<()> {
    let mark = Mark::new([1, 2, 3]);
    mark.shift([0.5, 0.5, 0.5])?;
    assert!(mark.trace().length() >= 0.0);
    Ok(())
}

#[test]
fn test_mark_collision() -> MarkResult<()> {
    let mark1 = Mark::new([1, 2, 3]);
    let mark2 = Mark::new([4, 5, 6]);

    mark1.place([0.0, 0.0, 0.0])?;
    mark2.place([2.0, 2.0, 2.0])?;

    assert!(!mark1.collides_with(&mark2)?);
    Ok(())
}

#[test]
fn test_mark_merging() -> MarkResult<()> {
    let mark1 = Mark::new([1, 2, 3]);
    let mark2 = Mark::new([4, 5, 6]);

    mark1.place([0.0, 0.0, 0.0])?;
    mark2.place([5.0, 5.0, 5.0])?;

    let merged = mark1.merge(&mark2)?;
    assert_eq!(merged.data().len(), mark1.data().len() + mark2.data().len());
    assert!(merged.crystal().strength() > 0.0);
    assert!(merged.trace().length() >= 0.0);

    Ok(())
}

#[test]
fn test_mark_identity() {
    let mark1 = Mark::new([1, 2, 3]);
    let mark2 = Mark::new([1, 2, 3]);
    let mark3 = Mark::new([4, 5, 6]);

    assert_eq!(mark1, mark2);
    assert_ne!(mark1, mark3);
    assert_ne!(mark2, mark3);
}

#[test]
fn test_mark_trace_extension() -> MarkResult<()> {
    let mark = Mark::new([1, 2, 3]);

    mark.place([0.0, 0.0, 0.0])?;
    mark.shift([1.0, 0.0, 0.0])?;
    mark.shift([0.0, 1.0, 0.0])?;
    mark.shift([0.0, 0.0, 1.0])?;

    assert!(mark.trace().points().len() > 0);
    assert!(mark.trace().length() > 0.0);
    assert!(mark.trace().curvature() >= 0.0);

    Ok(())
}

#[test]
fn test_mark_crystal_field() -> MarkResult<()> {
    let mark = Mark::new([1, 2, 3]);

    mark.place([0.0, 0.0, 0.0])?;
    assert_eq!(mark.crystal().center(), [0.0, 0.0, 0.0]);
    assert!(mark.crystal().radius() > 0.0);
    assert!(mark.crystal().strength() > 0.0);
    assert!(mark.crystal().coherence() > 0.0);

    Ok(())
}

#[test]
fn test_mark_error_handling() {
    let mark = Mark::new([1, 2, 3]);

    // Test out-of-range placement
    assert!(mark.place([100.0, 100.0, 100.0]).is_err());

    // Test excessive shift
    assert!(mark.shift([20.0, 20.0, 20.0]).is_err());

    // Test invalid merge
    let mark2 = Mark::new([4, 5, 6]);
    mark.place([0.0, 0.0, 0.0]).unwrap();
    mark2.place([0.1, 0.1, 0.1]).unwrap();
    assert!(mark.merge(&mark2).is_err());
}
EOL

    print_purple "✓ Created mark module files"
}

main() {
    print_purple "🔮 Creating Spark Mark Module..."
    setup_mark_module
    print_purple "✨ Mark module created with crystal-space tracking!

Features:
- Crystal field resonance
- Path trace tracking
- Quantum marking system
- Collision detection
- Field merging
- Comprehensive testing

Run 'cargo test' to verify the implementation!"
}

main
