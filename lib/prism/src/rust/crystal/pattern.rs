// pattern.rs - Crystal pattern management for Prism
// Created by: isdood
// Date: 2025-01-21 11:04:38 UTC

use std::sync::{Arc, Mutex, RwLock};
use std::collections::{HashMap, HashSet};
use std::f64::consts::PI;

use crate::types::{PrismError, PrismResult};
use super::bridge::{Crystal, CrystalNode, CrystalSystem};

/// Pattern types for crystal organization
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PatternType {
    Linear,
    Circular,
    Spiral,
    Grid,
    Cubic,
    Hexagonal,
    Custom,
}

/// Pattern configuration
#[derive(Debug, Clone)]
pub struct PatternConfig {
    pattern_type: PatternType,
    spacing: f64,
    scale: f64,
    rotation: [f64; 3],
    symmetry: u32,
}

impl Default for PatternConfig {
    fn default() -> Self {
        Self {
            pattern_type: PatternType::Cubic,
            spacing: 1.0,
            scale: 1.0,
            rotation: [0.0; 3],
            symmetry: 1,
        }
    }
}

/// Pattern generator for crystal structures
pub struct PatternGenerator {
    config: PatternConfig,
    crystal: Arc<Crystal>,
    nodes: Arc<RwLock<HashMap<[f64; 3], Arc<CrystalNode>>>>,
    stability_cache: Arc<Mutex<HashMap<PatternType, f64>>>,
}

impl PatternGenerator {
    /// Create a new pattern generator
    pub fn new(crystal: Arc<Crystal>, config: PatternConfig) -> Self {
        Self {
            config,
            crystal,
            nodes: Arc::new(RwLock::new(HashMap::new())),
            stability_cache: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /// Generate a pattern based on current configuration
    pub fn generate(&self) -> PrismResult<()> {
        match self.config.pattern_type {
            PatternType::Linear => self.generate_linear(),
            PatternType::Circular => self.generate_circular(),
            PatternType::Spiral => self.generate_spiral(),
            PatternType::Grid => self.generate_grid(),
            PatternType::Cubic => self.generate_cubic(),
            PatternType::Hexagonal => self.generate_hexagonal(),
            PatternType::Custom => Err(PrismError::InvalidArgument),
        }
    }

    /// Generate a linear pattern
    fn generate_linear(&self) -> PrismResult<()> {
        let mut nodes = self.nodes.write().unwrap();
        for i in 0..self.config.symmetry {
            let position = [
                i as f64 * self.config.spacing * self.config.scale,
                0.0,
                0.0,
            ];
            let node = self.crystal.add_node(position)?;
            nodes.insert(position, node);
        }
        self.update_stability(PatternType::Linear)
    }

    /// Generate a circular pattern
    fn generate_circular(&self) -> PrismResult<()> {
        let mut nodes = self.nodes.write().unwrap();
        let angle_step = 2.0 * PI / self.config.symmetry as f64;

        for i in 0..self.config.symmetry {
            let angle = angle_step * i as f64;
            let position = [
                angle.cos() * self.config.scale,
                angle.sin() * self.config.scale,
                0.0,
            ];
            let node = self.crystal.add_node(position)?;
            nodes.insert(position, node);
        }
        self.update_stability(PatternType::Circular)
    }

    /// Generate a spiral pattern
    fn generate_spiral(&self) -> PrismResult<()> {
        let mut nodes = self.nodes.write().unwrap();
        let angle_step = 2.0 * PI / self.config.symmetry as f64;
        let radius_step = self.config.spacing * self.config.scale;

        for i in 0..self.config.symmetry {
            let angle = angle_step * i as f64;
            let radius = radius_step * i as f64;
            let position = [
                angle.cos() * radius,
                angle.sin() * radius,
                angle * self.config.scale / (2.0 * PI),
            ];
            let node = self.crystal.add_node(position)?;
            nodes.insert(position, node);
        }
        self.update_stability(PatternType::Spiral)
    }

    /// Generate a grid pattern
    fn generate_grid(&self) -> PrismResult<()> {
        let mut nodes = self.nodes.write().unwrap();
        let size = (self.config.symmetry as f64).sqrt().ceil() as u32;

        for i in 0..size {
            for j in 0..size {
                let position = [
                    i as f64 * self.config.spacing * self.config.scale,
                    j as f64 * self.config.spacing * self.config.scale,
                    0.0,
                ];
                let node = self.crystal.add_node(position)?;
                nodes.insert(position, node);
            }
        }
        self.update_stability(PatternType::Grid)
    }

    /// Generate a cubic pattern
    fn generate_cubic(&self) -> PrismResult<()> {
        let mut nodes = self.nodes.write().unwrap();
        let size = (self.config.symmetry as f64).cbrt().ceil() as u32;

        for i in 0..size {
            for j in 0..size {
                for k in 0..size {
                    let position = [
                        i as f64 * self.config.spacing * self.config.scale,
                        j as f64 * self.config.spacing * self.config.scale,
                        k as f64 * self.config.spacing * self.config.scale,
                    ];
                    let node = self.crystal.add_node(position)?;
                    nodes.insert(position, node);
                }
            }
        }
        self.update_stability(PatternType::Cubic)
    }

    /// Generate a hexagonal pattern
    fn generate_hexagonal(&self) -> PrismResult<()> {
        let mut nodes = self.nodes.write().unwrap();
        let rings = self.config.symmetry;
        let hex_spacing = self.config.spacing * self.config.scale;

        for ring in 0..rings {
            let vertices = if ring == 0 { 1 } else { 6 * ring };
            let angle_step = 2.0 * PI / vertices as f64;

            for i in 0..vertices {
                let angle = angle_step * i as f64;
                let radius = ring as f64 * hex_spacing;
                let position = [
                    angle.cos() * radius,
                    angle.sin() * radius,
                    0.0,
                ];
                let node = self.crystal.add_node(position)?;
                nodes.insert(position, node);
            }
        }
        self.update_stability(PatternType::Hexagonal)
    }

    /// Update stability cache for pattern type
    fn update_stability(&self, pattern_type: PatternType) -> PrismResult<()> {
        self.crystal.optimize()?;
        let stability = self.crystal.stability()?;
        self.stability_cache.lock().unwrap().insert(pattern_type, stability);
        Ok(())
    }

    /// Get stability for a specific pattern type
    pub fn get_stability(&self, pattern_type: PatternType) -> Option<f64> {
        self.stability_cache.lock().unwrap().get(&pattern_type).copied()
    }

    /// Rotate the entire pattern
    pub fn rotate(&mut self, rotation: [f64; 3]) -> PrismResult<()> {
        self.config.rotation = rotation;
        let nodes = self.nodes.read().unwrap();
        
        for (pos, node) in nodes.iter() {
            let rotated_pos = self.rotate_point(*pos, rotation);
            self.crystal.remove_node(Arc::clone(node))?;
            self.crystal.add_node(rotated_pos)?;
        }
        
        self.crystal.optimize()
    }

    /// Rotate a point around the origin
    fn rotate_point(&self, point: [f64; 3], rotation: [f64; 3]) -> [f64; 3] {
        let [x, y, z] = point;
        let [rx, ry, rz] = rotation;

        // Apply rotations in order: Z, Y, X
        let (sz, cz) = rz.sin_cos();
        let (sy, cy) = ry.sin_cos();
        let (sx, cx) = rx.sin_cos();

        let x1 = x * cz - y * sz;
        let y1 = x * sz + y * cz;
        let z1 = z;

        let x2 = x1 * cy + z1 * sy;
        let y2 = y1;
        let z2 = -x1 * sy + z1 * cy;

        let x3 = x2;
        let y3 = y2 * cx - z2 * sx;
        let z3 = y2 * sx + z2 * cx;

        [x3, y3, z3]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pattern_generation() {
        let crystal = Arc::new(Crystal::new(CrystalSystem::Cubic).unwrap());
        let config = PatternConfig {
            pattern_type: PatternType::Cubic,
            spacing: 1.0,
            scale: 1.0,
            rotation: [0.0; 3],
            symmetry: 8,
        };

        let generator = PatternGenerator::new(Arc::clone(&crystal), config);
        generator.generate().unwrap();

        let stability = generator.get_stability(PatternType::Cubic).unwrap();
        assert!(stability > 0.0);
    }

    #[test]
    fn test_pattern_rotation() {
        let crystal = Arc::new(Crystal::new(CrystalSystem::Cubic).unwrap());
        let config = PatternConfig {
            pattern_type: PatternType::Linear,
            spacing: 1.0,
            scale: 1.0,
            rotation: [0.0; 3],
            symmetry: 4,
        };

        let mut generator = PatternGenerator::new(Arc::clone(&crystal), config);
        generator.generate().unwrap();
        generator.rotate([PI/4.0, 0.0, 0.0]).unwrap();

        let stability = crystal.stability().unwrap();
        assert!(stability > 0.0);
    }
}
