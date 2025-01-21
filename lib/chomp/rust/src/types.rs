//! Type System for Rust-Zig Integration
//! ================================
//! Author: isdood
//! Created: 2025-01-21 03:02:54 UTC
//! License: MIT

use std::any::TypeId;
use std::collections::HashMap;
use thiserror::Error;

pub struct Registry {
    types: HashMap<TypeId, TypeInfo>,
}

pub struct TypeInfo {
    name: String,
    size: usize,
    alignment: usize,
    type_id: TypeId,
}

impl Registry {
    pub fn new() -> Self {
        Self {
            types: HashMap::new(),
        }
    }

    pub fn register<T: 'static>(&mut self) -> Result<(), Error> {
        let type_id = TypeId::of::<T>();
        let type_info = TypeInfo {
            name: std::any::type_name::<T>().to_string(),
            size: std::mem::size_of::<T>(),
            alignment: std::mem::align_of::<T>(),
            type_id,
        };

        self.types.insert(type_id, type_info);
        Ok(())
    }

    pub fn contains_type(&self, type_id: TypeId) -> bool {
        self.types.contains_key(&type_id)
    }

    pub fn get_type_info(&self, type_id: TypeId) -> Option<&TypeInfo> {
        self.types.get(&type_id)
    }
}

#[derive(Debug, Error)]
pub enum Error {
    #[error("Type registration failed: {0}")]
    RegistrationFailed(String),

    #[error("Type not found: {0}")]
    TypeNotFound(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_type_registration() {
        let mut registry = Registry::new();
        assert!(registry.register::<i32>().is_ok());
        assert!(registry.contains_type(TypeId::of::<i32>()));
    }
}
