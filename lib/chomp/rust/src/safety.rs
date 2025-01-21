//! Safety System for Rust Integration
//! ==============================
//! Author: isdood
//! Created: 2025-01-21 03:02:54 UTC
//! License: MIT

use std::any::TypeId;
use std::collections::HashMap;
use thiserror::Error;

pub struct Context {
    type_safety: HashMap<TypeId, SafetyInfo>,
}

pub struct SafetyInfo {
    is_send: bool,
    is_sync: bool,
    needs_drop: bool,
}

impl Context {
    pub fn new() -> Self {
        Self {
            type_safety: HashMap::new(),
        }
    }

    pub fn verify_type<T: 'static>(&mut self) -> Result<(), Error> {
        let type_id = TypeId::of::<T>();
        let safety_info = SafetyInfo {
            is_send: std::marker::PhantomData::<T>::default().is_send(),
            is_sync: std::marker::PhantomData::<T>::default().is_sync(),
            needs_drop: std::mem::needs_drop::<T>(),
        };

        self.type_safety.insert(type_id, safety_info);
        Ok(())
    }

    pub fn check_safety<T: 'static>(&self) -> Result<(), Error> {
        let type_id = TypeId::of::<T>();
        let safety_info = self.type_safety.get(&type_id)
        .ok_or(Error::TypeNotVerified)?;

        if !safety_info.is_send {
            return Err(Error::NotSendable);
        }

        if !safety_info.is_sync {
            return Err(Error::NotSyncable);
        }

        Ok(())
    }
}

#[derive(Debug, Error)]
pub enum Error {
    #[error("Type not verified")]
    TypeNotVerified,

    #[error("Type is not Send")]
    NotSendable,

    #[error("Type is not Sync")]
    NotSyncable,

    #[error("Safety violation: {0}")]
    SafetyViolation(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_safety_verification() {
        let mut context = Context::new();
        assert!(context.verify_type::<i32>().is_ok());
        assert!(context.check_safety::<i32>().is_ok());
    }
}
