//! Crystal-optimized 3D resonance-based formatting system.
//!
//! This module provides a high-performance formatting system based on
//! wave harmonics and crystal lattice resonance patterns.

pub mod harmony;
pub mod wave;
pub mod lattice;

use std::fmt;
use std::sync::Arc;
use std::marker::PhantomData;
use self::harmony::HarmonicPattern;
pub use self::wave::{Wave, Amplitude, Phase};
use self::lattice::{LatticePoint, ResonanceNode};

/// A 3D coordinate in resonance space
#[derive(Debug, Clone, Copy)]
pub struct ResonancePoint {
    x: f64,
    y: f64,
    z: f64,
    amplitude: Amplitude,
    phase: Phase,
}

impl ResonancePoint {
    /// Creates a new resonance point
    pub fn new(x: f64, y: f64, z: f64, amplitude: f64, phase: f64) -> Self {
        Self {
            x,
            y,
            z,
            amplitude: Amplitude::new(amplitude),
            phase: Phase::new(phase),
        }
    }

    /// Calculates the resonance intensity at this point
    pub fn intensity(&self) -> f64 {
        let distance = (self.x * self.x + self.y * self.y + self.z * self.z).sqrt();
        self.amplitude.value() * (-distance * self.phase.value()).exp()
    }
}

/// A resonance pattern in 3D space
#[derive(Debug, Clone)]
pub struct Resonance {
    points: Vec<ResonancePoint>,
    pattern: HarmonicPattern,
    lattice_nodes: Vec<ResonanceNode>,
}

impl Resonance {
    /// Creates a new resonance pattern
    pub fn new() -> Self {
        Self {
            points: Vec::new(),
            pattern: HarmonicPattern::default(),
            lattice_nodes: Vec::new(),
        }
    }

    /// Adds a resonance point
    /// Adds a resonance point
    pub fn add_point(&mut self, point: ResonancePoint) {
        // Create a new lattice node
        let lattice_point = LatticePoint::new(
            point.x.round() as i32,
            point.y.round() as i32,
            point.z.round() as i32,
        );
        let mut node = ResonanceNode::new(lattice_point);
        
        // Update resonance
        node.update_resonance(&point);
        
        // Connect to nearby nodes
        for (i, existing) in self.lattice_nodes.iter().enumerate() {
            let coords = existing.coordinates();
            let dx = coords[0] - node.coordinates()[0];
            let dy = coords[1] - node.coordinates()[1];
            let dz = coords[2] - node.coordinates()[2];
            
            if dx * dx + dy * dy + dz * dz <= 3 {
                node.add_connection(i);
            }
        }
        
        self.lattice_nodes.push(node);
        
        // Update pattern
        self.points.push(point);
        self.pattern.update(&self.points);
    }

    /// Gets the total resonance intensity
    pub fn total_intensity(&self) -> f64 {
        self.points.iter().map(|p| p.intensity()).sum()
    }

    /// Gets a reference to the lattice nodes
    pub fn lattice_nodes(&self) -> &[ResonanceNode] {
        &self.lattice_nodes
    }
}

impl Default for Resonance {
    fn default() -> Self {
        Self::new()
    }
}

/// Main formatting interface using resonance patterns
#[derive(Debug, Clone)]
pub struct Scribe {
    resonance: Arc<Resonance>,
    wave: Wave,
}

impl Scribe {
    /// Creates a new Scribe instance
    pub fn new() -> Self {
        Self {
            resonance: Arc::new(Resonance::new()),
            wave: Wave::default(),
        }
    }

    /// Adds a resonance point to the pattern
    pub fn add_point(&mut self, x: f64, y: f64, z: f64, amplitude: f64, phase: f64) {
        Arc::make_mut(&mut self.resonance).add_point(ResonancePoint::new(x, y, z, amplitude, phase));
    }

    /// Gets the current resonance pattern
    pub fn resonance(&self) -> &Resonance {
        &self.resonance
    }

    /// Gets the current wave
    pub fn wave(&self) -> &Wave {
        &self.wave
    }

    /// Formats a value using resonance patterns
    pub fn format<T: fmt::Display>(&self, value: T) -> String {
        let mut result = Vec::new();
        let text = value.to_string();

        // Convert each character to a resonance point
        for (i, c) in text.chars().enumerate() {
            let x = (i as f64 * std::f64::consts::PI * 2.0 / text.len() as f64).cos();
            let y = (i as f64 * std::f64::consts::PI * 2.0 / text.len() as f64).sin();
            let z = (c as u32 as f64) / 128.0;

            let point = ResonancePoint::new(x, y, z, 1.0, 0.0);
            result.push(point);
        }

        // Apply resonance pattern
        let mut output = String::with_capacity(text.len());
        for point in result {
            let intensity = point.intensity();
            let c = (intensity * 128.0) as u8 as char;
            output.push(c);
        }

        output
    }
}

impl Default for Scribe {
    fn default() -> Self {
        Self::new()
    }
}

/// A formatted value using resonance patterns
#[derive(Debug, Clone)]
pub struct ResonanceFormat<T> {
    value: T,
    resonance: Arc<Resonance>,
    _phantom: PhantomData<T>,
}

impl<T: fmt::Display> ResonanceFormat<T> {
    /// Creates a new resonance format
    pub fn new(value: T, resonance: Arc<Resonance>) -> Self {
        Self {
            value,
            resonance,
            _phantom: PhantomData,
        }
    }

    /// Gets the formatted value
    pub fn get(&self) -> String {
        let scribe = Scribe {
            resonance: Arc::clone(&self.resonance),
            wave: Wave::default(),
        };
        scribe.format(&self.value)
    }
}
