//! Chomp Rust Integration Library
//! ==========================
//! Author: isdood
//! Created: 2025-01-21 03:02:54 UTC
//! License: MIT

use std::sync::Arc;
use parking_lot::RwLock;

pub mod ffi;
pub mod safety;
pub mod types;
pub mod bridge;

#[derive(Debug)]
pub struct ChompRust {
    safety_context: Arc<RwLock<safety::Context>>,
    type_registry: Arc<RwLock<types::Registry>>,
    bridge_manager: Arc<bridge::Manager>,
}

impl ChompRust {
    pub fn new() -> Self {
        let safety_context = Arc::new(RwLock::new(safety::Context::new()));
        let type_registry = Arc::new(RwLock::new(types::Registry::new()));
        let bridge_manager = Arc::new(bridge::Manager::new(
            safety_context.clone(),
                                                           type_registry.clone(),
        ));

        Self {
            safety_context,
            type_registry,
            bridge_manager,
        }
    }

    pub fn register_type<T: 'static>(&self) -> Result<(), bridge::Error> {
        self.bridge_manager.register_type::<T>()
    }

    pub fn export_function<F>(&self, name: &str, func: F) -> Result<(), bridge::Error>
    where
    F: Fn() -> Result<(), bridge::Error> + Send + Sync + 'static,
    {
        self.bridge_manager.export_function(name, func)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_integration() {
        let chomp = ChompRust::new();
        assert!(chomp.register_type::<i32>().is_ok());
    }
}
