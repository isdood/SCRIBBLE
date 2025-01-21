//! Wave convergence management via Chapel
use crate::bridge::chapel::ChapelBridge;

pub struct WaveConvergence {
    bridge: Arc<ChapelBridge>,
}
