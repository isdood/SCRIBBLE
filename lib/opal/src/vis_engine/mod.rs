pub mod core;
pub mod renderers;
pub mod views;
pub mod utils;

pub use core::engine::VisEngine;
pub use renderers::Renderer;
pub use views::{FieldVisualizer, LatticeVisualizer};

pub const VERSION: &str = "0.1.0";
