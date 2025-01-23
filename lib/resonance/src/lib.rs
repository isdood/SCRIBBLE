pub mod core;
pub mod harmony;
pub mod crystals;

pub use crate::core::ResonanceCore;
pub use crate::harmony::HarmonyWeaver;
pub use crate::crystals::CrystalField;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum ResonanceError {
    #[error("harmony disruption: {0}")]
    HarmonyDisrupted(String),
    #[error("whimsy overflow: {0}")]
    WhimsyOverflow(String),
}

pub type Result<T> = std::result::Result<T, ResonanceError>;
