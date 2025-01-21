// src/mod.rs
// Created: 2025-01-21 18:44:05 UTC
// Author: isdood

//! # Prismancer Engine
//!
//! A crystal-based high-performance game engine built on the Scribble framework.
//!
//! ## Architecture
//!
//! Prismancer is organized into several main components:
//! - Core: Crystal computing and fundamental systems
//! - Render: Vulkan-based rendering pipeline
//! - Physics: Harmony-aware physics simulation
//! - Systems: Entity and component management
//!
//! ## Usage
//!
//! ```rust,no_run
//! use prismancer::{Engine, EngineConfig};
//!
//! let config = EngineConfig::default();
//! let engine = Engine::new(config)?;
//! engine.start()?;
//! ```

// Version information
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
pub const BUILD_DATE: &str = "2025-01-21 18:44:05";
pub const BUILD_USER: &str = "isdood";

// Re-export main engine types
pub use crate::engine::{Engine, EngineConfig, EngineError, EngineResult, EngineMetrics};

// Core module
pub mod core {
    //! Core engine systems and utilities

    pub mod crystal;
    pub mod systems;
    pub mod parallel;
    pub mod memory;
    pub mod utils;

    // Re-export common types
    pub use crystal::{Crystal, CrystalConfig, CrystalError};
    pub use systems::SystemManager;
    pub use parallel::TaskScheduler;
    pub use memory::MemoryPool;
}

// Render module
pub mod render {
    //! Rendering system and graphics pipeline

    pub mod vulkan;
    pub mod pipeline;
    pub mod mesh;
    pub mod material;
    pub mod shader;
    pub mod texture;

    // Re-export common types
    pub use vulkan::{VulkanContext, VulkanError};
    pub use pipeline::{RenderPipeline, PipelineConfig};
    pub use mesh::{Mesh, MeshBuilder};
    pub use material::{Material, MaterialType};
    pub use shader::{Shader, ShaderStage};
    pub use texture::{Texture, TextureFormat};

    mod manager;
    pub use manager::RenderManager;
}

// Physics module
pub mod physics {
    //! Physics simulation and harmony effects

    pub mod world;
    pub mod harmony;
    pub mod collision;
    pub mod forces;
    pub mod constraints;

    // Re-export common types
    pub use world::PhysicsWorld;
    pub use harmony::HarmonySimulator;
}

// Systems module
pub mod systems {
    //! Entity and component management

    pub mod ecs;
    pub mod resources;
    pub mod events;
    pub mod scheduling;

    // Re-export common types
    pub use ecs::{World, Entity, Component};
    pub use resources::ResourceManager;
    pub use events::EventSystem;
    pub use scheduling::Scheduler;
}

// Prelude module for convenient imports
pub mod prelude {
    //! Common types and traits

    pub use crate::{
        Engine,
        EngineConfig,
        EngineResult,
        core::{Crystal, CrystalConfig},
        render::{Mesh, Material, Shader},
        physics::PhysicsWorld,
        systems::{World, Entity, Component},
    };
}

// Tests module
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version_info() {
        assert!(!VERSION.is_empty());
        assert!(!BUILD_DATE.is_empty());
        assert!(!BUILD_USER.is_empty());
    }
}
