//! IDK (Intermediate Data Kernel) Module
//! ===================================
//!
//! Author: Caleb J.D. Terkovics <isdood>
//! Current User: isdood
//! Created: 2025-01-19
//! Last Updated: 2025-01-20 20:41:18 UTC
//! Version: 0.1.0
//! License: MIT

use core::fmt::{self, Display, Debug};

use magicmath::{
    traits::MeshValue,
    base::{Field, FieldValue},
    resonance::Resonance,
    spatial::Vector3D,
};

use errors::{
    core::MathError,
    core::Result as MathResult,
};

use scribe::{
    Scribe,
    native_string::String,
};

/// Uninitialized shard state
#[derive(Debug)]
pub struct ShardUninit<T> {
    value: T,
    initialized: bool,
}

impl<T: Default> ShardUninit<T> {
    /// Create new uninitialized shard
    pub fn new() -> Self {
        Self {
            value: T::default(),
            initialized: false,
        }
    }

    /// Initialize shard with value
    pub fn init(&mut self, value: T) {
        self.value = value;
        self.initialized = true;
    }

    /// Check if shard is initialized
    pub fn is_init(&self) -> bool {
        self.initialized
    }

    /// Get value if initialized
    pub fn value(&self) -> Option<&T> {
        if self.initialized {
            Some(&self.value)
        } else {
            None
        }
    }
}

/// Harmony state container
#[derive(Debug)]
pub struct HarmonyState<T> {
    value: T,
    position: Vector3D,
    field: Field,
    resonance: Resonance,
}

impl<T: MeshValue + Display> HarmonyState<T> {
    /// Create new harmony state
    pub fn new(value: T, position: Vector3D) -> Self {
        Self {
            value,
            position,
            field: Field::default(),
            resonance: Resonance::new(),
        }
    }

    /// Get current value
    pub fn value(&self) -> &T {
        &self.value
    }

    /// Get current position
    pub fn position(&self) -> &Vector3D {
        &self.position
    }

    /// Update field state
    pub fn update_field(&mut self) -> MathResult<()> {
        self.field.transform(&self.position)?;
        Ok(())
    }

    /// Calculate total energy
    pub fn energy(&self) -> MathResult<f64> {
        let field_energy = self.field.energy()?;
        let value_energy = self.value.energy()?;
        Ok(field_energy + value_energy)
    }
}

impl<T: MeshValue + Display> Display for HarmonyState<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "State({}) at {:?}", self.value, self.position)
    }
}

impl<T: MeshValue + Display> Scribe for HarmonyState<T> {
    fn scribe(&self) -> String {
        let mut result = String::new();
        result.push_str("Harmony State:\n");
        result.push_str("Value: ");
        result.push_str(&self.value.to_string());
        result.push_str("\nPosition: ");
        result.push_str(&format!("{:?}", self.position));
        result.push_str("\nField Energy: ");
        result.push_str(&self.field.energy().unwrap_or(0.0).to_string());
        result
    }
}
