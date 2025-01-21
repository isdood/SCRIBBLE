// resonance_patterns.rs - Resonance pattern management for Prism
// Created by: isdood
// Date: 2025-01-21 11:23:20 UTC

use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use std::f64::consts::PI;

use crate::crystal::bridge::{Crystal, CrystalNode};
use crate::types::{PrismError, PrismResult};
use super::pattern::{Pattern, PatternConfig, PatternType};

/// Resonance mode types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ResonanceMode {
    Fundamental,
    Harmonic(u32),
    Standing,
    Traveling,
    Coupled,
    Custom(u32),
}

/// Resonance configuration
#[derive(Debug, Clone)]
pub struct ResonanceConfig {
    mode: ResonanceMode,
    frequency: f64,
    amplitude: f64,
    phase: f64,
    damping: f64,
    coupling_factor: f64,
}

impl Default for ResonanceConfig {
    fn default() -> Self {
        Self {
            mode: ResonanceMode::Fundamental,
            frequency: 1.0,
            amplitude: 1.0,
            phase: 0.0,
            damping: 0.1,
            coupling_factor: 0.5,
        }
    }
}

/// Resonance pattern manager
pub struct ResonancePattern {
    crystal: Arc<Crystal>,
    config: ResonanceConfig,
    pattern: Pattern,
    nodes: Arc<Mutex<HashMap<[f64; 3], Arc<CrystalNode>>>>,
    time: f64,
}

impl ResonancePattern {
    /// Create a new resonance pattern
    pub fn new(crystal: Arc<Crystal>, config: ResonanceConfig) -> Self {
        Self {
            crystal,
            config,
            pattern: Pattern::new(PatternConfig {
                pattern_type: PatternType::Custom,
                spacing: 1.0,
                scale: 1.0,
                rotation: [0.0; 3],
                symmetry: 1,
            }),
            nodes: Arc::new(Mutex::new(HashMap::new())),
            time: 0.0,
        }
    }

    /// Initialize the resonance pattern
    pub fn initialize(&mut self) -> PrismResult<()> {
        // Clear existing nodes
        self.nodes.lock().unwrap().clear();

        // Create base pattern based on resonance mode
        match self.config.mode {
            ResonanceMode::Fundamental => self.initialize_fundamental()?,
            ResonanceMode::Harmonic(n) => self.initialize_harmonic(n)?,
            ResonanceMode::Standing => self.initialize_standing()?,
            ResonanceMode::Traveling => self.initialize_traveling()?,
            ResonanceMode::Coupled => self.initialize_coupled()?,
            ResonanceMode::Custom(n) => self.initialize_custom(n)?,
        }

        self.time = 0.0;
        Ok(())
    }

    /// Update resonance pattern state
    pub fn update(&mut self, dt: f64) -> PrismResult<()> {
        self.time += dt;
        let nodes = self.nodes.lock().unwrap();

        for (pos, node) in nodes.iter() {
            let displacement = self.calculate_displacement(*pos);
            self.crystal.move_node(Arc::clone(node), displacement)?;
        }

        Ok(())
    }

    /// Calculate resonance energy
    pub fn calculate_energy(&self) -> f64 {
        let nodes = self.nodes.lock().unwrap();
        let mut energy = 0.0;

        for (pos, _) in nodes.iter() {
            let displacement = self.calculate_displacement(*pos);
            energy += 0.5 * self.config.amplitude * displacement.iter().map(|x| x * x).sum::<f64>();
        }

        energy
    }

    /// Initialize fundamental mode
    fn initialize_fundamental(&mut self) -> PrismResult<()> {
        let mut nodes = self.nodes.lock().unwrap();
        let spacing = 1.0;

        for i in 0..8 {
            let x = (i as f64) * spacing;
            let position = [x, 0.0, 0.0];
            let node = self.crystal.add_node(position)?;
            nodes.insert(position, node);
        }

        Ok(())
    }

    /// Initialize harmonic mode
    fn initialize_harmonic(&mut self, n: u32) -> PrismResult<()> {
        let mut nodes = self.nodes.lock().unwrap();
        let spacing = 1.0;
        let wavelength = 8.0 / n as f64;

        for i in 0..8 {
            let x = (i as f64) * spacing;
            let y = (2.0 * PI * x / wavelength).sin() * self.config.amplitude;
            let position = [x, y, 0.0];
            let node = self.crystal.add_node(position)?;
            nodes.insert(position, node);
        }

        Ok(())
    }

    /// Initialize standing wave mode
    fn initialize_standing(&mut self) -> PrismResult<()> {
        let mut nodes = self.nodes.lock().unwrap();
        let spacing = 1.0;

        for i in 0..8 {
            for j in 0..8 {
                let x = (i as f64) * spacing;
                let y = (j as f64) * spacing;
                let position = [x, y, 0.0];
                let node = self.crystal.add_node(position)?;
                nodes.insert(position, node);
            }
        }

        Ok(())
    }

    /// Initialize traveling wave mode
    fn initialize_traveling(&mut self) -> PrismResult<()> {
        let mut nodes = self.nodes.lock().unwrap();
        let spacing = 1.0;

        for i in 0..12 {
            let angle = (i as f64) * PI / 6.0;
            let x = angle.cos() * spacing;
            let y = angle.sin() * spacing;
            let position = [x, y, 0.0];
            let node = self.crystal.add_node(position)?;
            nodes.insert(position, node);
        }

        Ok(())
    }

    /// Initialize coupled mode
    fn initialize_coupled(&mut self) -> PrismResult<()> {
        let mut nodes = self.nodes.lock().unwrap();
        let spacing = 1.0;

        // Create two interacting chains
        for i in 0..6 {
            let x = (i as f64) * spacing;
            let position1 = [x, 0.0, 0.0];
            let position2 = [x, spacing, 0.0];
            
            let node1 = self.crystal.add_node(position1)?;
            let node2 = self.crystal.add_node(position2)?;
            
            nodes.insert(position1, node1);
            nodes.insert(position2, node2);
        }

        Ok(())
    }

    /// Initialize custom mode
    fn initialize_custom(&mut self, n: u32) -> PrismResult<()> {
        if n == 0 {
            return Err(PrismError::InvalidArgument);
        }

        let mut nodes = self.nodes.lock().unwrap();
        let spacing = 1.0;
        let radius = n as f64 * spacing;

        for i in 0..n {
            let angle = (i as f64) * 2.0 * PI / n as f64;
            let x = angle.cos() * radius;
            let y = angle.sin() * radius;
            let position = [x, y, 0.0];
            let node = self.crystal.add_node(position)?;
            nodes.insert(position, node);
        }

        Ok(())
    }

    /// Calculate displacement for a given position
    fn calculate_displacement(&self, pos: [f64; 3]) -> [f64; 3] {
        let omega = 2.0 * PI * self.config.frequency;
        let damped_amplitude = self.config.amplitude * (-self.config.damping * self.time).exp();

        match self.config.mode {
            ResonanceMode::Fundamental => {
                let phase = omega * self.time + self.config.phase;
                [0.0, damped_amplitude * phase.sin(), 0.0]
            },
            ResonanceMode::Harmonic(n) => {
                let phase = n as f64 * omega * self.time + self.config.phase;
                [0.0, damped_amplitude * phase.sin(), 0.0]
            },
            ResonanceMode::Standing => {
                let kx = 2.0 * PI * pos[0];
                let phase = omega * self.time + self.config.phase;
                [0.0, damped_amplitude * kx.cos() * phase.sin(), 0.0]
            },
            ResonanceMode::Traveling => {
                let kr = (pos[0] * pos[0] + pos[1] * pos[1]).sqrt();
                let phase = omega * self.time - kr + self.config.phase;
                [damped_amplitude * phase.cos(), damped_amplitude * phase.sin(), 0.0]
            },
            ResonanceMode::Coupled => {
                let coupling = self.config.coupling_factor * pos[1];
                let phase = omega * self.time + self.config.phase + coupling;
                [0.0, damped_amplitude * phase.sin(), 0.0]
            },
            ResonanceMode::Custom(n) => {
                let r = (pos[0] * pos[0] + pos[1] * pos[1]).sqrt();
                let theta = pos[1].atan2(pos[0]);
                let phase = n as f64 * theta + omega * self.time + self.config.phase;
                [
                    damped_amplitude * phase.cos() * r.cos(),
                    damped_amplitude * phase.sin() * r.sin(),
                    0.0,
                ]
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_resonance_initialization() {
        let crystal = Arc::new(Crystal::new(crate::crystal::bridge::CrystalSystem::Cubic).unwrap());
        let config = ResonanceConfig::default();
        let mut pattern = ResonancePattern::new(Arc::clone(&crystal), config);
        
        assert!(pattern.initialize().is_ok());
        assert!(pattern.nodes.lock().unwrap().len() > 0);
    }

    #[test]
    fn test_resonance_update() {
        let crystal = Arc::new(Crystal::new(crate::crystal::bridge::CrystalSystem::Cubic).unwrap());
        let config = ResonanceConfig::default();
        let mut pattern = ResonancePattern::new(Arc::clone(&crystal), config);
        
        pattern.initialize().unwrap();
        assert!(pattern.update(0.1).is_ok());
    }

    #[test]
    fn test_energy_calculation() {
        let crystal = Arc::new(Crystal::new(crate::crystal::bridge::CrystalSystem::Cubic).unwrap());
        let config = ResonanceConfig::default();
        let mut pattern = ResonancePattern::new(Arc::clone(&crystal), config);
        
        pattern.initialize().unwrap();
        let energy = pattern.calculate_energy();
        assert!(energy >= 0.0);
    }
}
