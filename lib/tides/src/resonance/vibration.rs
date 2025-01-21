//! Crystal vibration management via Julia
use crate::bridge::julia::JuliaBridge;

pub struct CrystalVibration {
    bridge: Arc<JuliaBridge>,
}
