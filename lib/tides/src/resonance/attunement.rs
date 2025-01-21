//! Crystal attunement management via Julia
use crate::bridge::julia::JuliaBridge;

pub struct CrystalAttunement {
    bridge: Arc<JuliaBridge>,
}
