//! Bridge System Between Zig and Rust
//! ==============================
//! Author: isdood
//! Created: 2025-01-21 03:02:54 UTC
//! License: MIT

use std::sync::Arc;
use std::any::TypeId;
use std::collections::HashMap;
use parking_lot::RwLock;
use thiserror::Error;

use crate::safety;
use crate::types;

pub struct Manager {
    safety_context: Arc<RwLock<safety::Context>>,
    type_registry: Arc<RwLock<types::Registry>>,
    functions: RwLock<HashMap<String, Box<dyn Fn() -> Result<(), Error> + Send + Sync>>>,
}

impl Manager {
    pub fn new(
        safety_context: Arc<RwLock<safety::Context>>,
        type_registry: Arc<RwLock<types::Registry>>,
    ) -> Self {
        Self {
            safety_context,
            type_registry,
            functions: RwLock::new(HashMap::new()),
        }
    }

    pub fn register_type<T: 'static>(&self) -> Result<(), Error> {
        let type_id = TypeId::of::<T>();
        let mut registry = self.type_registry.write();

        if registry.contains_type(type_id) {
            return Err(Error::TypeAlreadyRegistered);
        }

        registry.register::<T>()?;
        Ok(())
    }

    pub fn export_function<F>(&self, name: &str, func: F) -> Result<(), Error>
    where
    F: Fn() -> Result<(), Error> + Send + Sync + 'static,
    {
        let mut functions = self.functions.write();

        if functions.contains_key(name) {
            return Err(Error::FunctionAlreadyExported);
        }

        functions.insert(name.to_string(), Box::new(func));
        Ok(())
    }

    pub fn call_function(&self, name: &str) -> Result<(), Error> {
        let functions = self.functions.read();
        let func = functions.get(name)
        .ok_or(Error::FunctionNotFound)?;

        func()
    }
}

#[derive(Debug, Error)]
pub enum Error {
    #[error("Type already registered")]
    TypeAlreadyRegistered,

    #[error("Function already exported: {0}")]
    FunctionAlreadyExported(String),

    #[error("Function not found: {0}")]
    FunctionNotFound(String),

    #[error("Safety violation: {0}")]
    SafetyViolation(String),

    #[error("Type error: {0}")]
    TypeError(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_function_export() {
        let safety_context = Arc::new(RwLock::new(safety::Context::new()));
        let type_registry = Arc::new(RwLock::new(types::Registry::new()));
        let manager = Manager::new(safety_context, type_registry);

        let result = manager.export_function("test", || Ok(()));
        assert!(result.is_ok());

        let result = manager.call_function("test");
        assert!(result.is_ok());
    }
}
