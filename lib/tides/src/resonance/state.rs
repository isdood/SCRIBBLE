//! Crystal state management via Julia
use crate::bridge::julia::JuliaBridge;

pub struct CrystalState {
    bridge: Arc<JuliaBridge>,
}
