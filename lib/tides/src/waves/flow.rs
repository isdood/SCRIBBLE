//! Wave flow management via Chapel
use crate::bridge::chapel::ChapelBridge;

pub struct WaveFlow {
    bridge: Arc<ChapelBridge>,
}
