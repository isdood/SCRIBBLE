#!/bin/bash

# Spark Scribe Fix Script (Part 4)
# Author: isdood
# Created: 2025-01-25 20:03:28 UTC
# Repository: isdood/scribble
# Description: Fixes scribe module duplicated test attribute warning

PURPLE='\033[0;35m'
NC='\033[0m'

print_purple() {
    echo -e "${PURPLE}$1${NC}"
}

fix_scribe_module() {
    cd forge/std || exit 1

    # Update lattice module with fixed test attributes
    cat > src/scribe/lattice/mod.rs << 'EOL'
//! Lattice module for resonance patterns

use super::ResonancePoint;

/// A point in the crystal lattice
#[derive(Debug, Clone, Copy)]
pub struct LatticePoint {
    index: [i32; 3],
    resonance: f64,
}

impl LatticePoint {
    /// Creates a new lattice point
    pub fn new(x: i32, y: i32, z: i32) -> Self {
        Self {
            index: [x, y, z],
            resonance: 0.0,
        }
    }

    /// Updates the resonance value
    pub fn update_resonance(&mut self, point: &ResonancePoint) {
        let dx = self.index[0] as f64 - point.x;
        let dy = self.index[1] as f64 - point.y;
        let dz = self.index[2] as f64 - point.z;
        let distance = (dx * dx + dy * dy + dz * dz).sqrt();

        self.resonance += point.intensity() * (-distance).exp();
    }

    /// Gets the current resonance value
    pub fn resonance(&self) -> f64 {
        self.resonance
    }

    /// Gets the lattice coordinates
    pub fn coordinates(&self) -> &[i32; 3] {
        &self.index
    }
}

/// A resonance node in the crystal lattice
#[derive(Debug, Clone)]
pub struct ResonanceNode {
    point: LatticePoint,
    connections: Vec<usize>,
}

impl ResonanceNode {
    /// Creates a new resonance node
    pub fn new(point: LatticePoint) -> Self {
        Self {
            point,
            connections: Vec::new(),
        }
    }

    /// Gets a reference to the lattice point
    pub fn point(&self) -> &LatticePoint {
        &self.point
    }

    /// Gets a mutable reference to the lattice point
    pub fn point_mut(&mut self) -> &mut LatticePoint {
        &mut self.point
    }

    /// Gets the resonance value at this node
    pub fn resonance(&self) -> f64 {
        self.point.resonance()
    }

    /// Updates the resonance value at this node
    pub fn update_resonance(&mut self, point: &ResonancePoint) {
        self.point.update_resonance(point);
    }

    /// Adds a connection to another node
    pub fn add_connection(&mut self, index: usize) {
        if !self.connections.contains(&index) {
            self.connections.push(index);
        }
    }

    /// Gets the list of connections
    pub fn connections(&self) -> &[usize] {
        &self.connections
    }

    /// Gets the coordinates of this node
    pub fn coordinates(&self) -> &[i32; 3] {
        self.point.coordinates()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lattice_point_creation() {
        let point = LatticePoint::new(1, 2, 3);
        assert_eq!(point.resonance(), 0.0);
        assert_eq!(point.coordinates(), &[1, 2, 3]);
    }

    #[test]
    fn test_lattice_point_resonance() {
        let mut point = LatticePoint::new(0, 0, 0);
        let res_point = ResonancePoint::new(0.1, 0.1, 0.1, 1.0, 0.0);
        point.update_resonance(&res_point);
        assert!(point.resonance() > 0.0);
    }

    #[test]
    fn test_resonance_node() {
        let point = LatticePoint::new(1, 2, 3);
        let mut node = ResonanceNode::new(point);

        assert_eq!(node.resonance(), 0.0);
        assert_eq!(node.coordinates(), &[1, 2, 3]);

        node.add_connection(1);
        assert_eq!(node.connections(), &[1]);
    }
}
EOL

    print_purple "✓ Fixed scribe module implementation"
}

main() {
    print_purple "🔮 Fixing Spark Scribe Module..."
    fix_scribe_module
    print_purple "✨ Scribe module fixes applied!

Fixed Issues:
- Fixed duplicated test attributes
- Split lattice point tests
- Added resonance testing
- Improved test organization
- Enhanced code clarity
- Better test coverage
- Fixed all warnings

Run 'cargo test' to verify the fixes!"
}

main
