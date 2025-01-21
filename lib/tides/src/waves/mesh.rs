//! Wave mesh management via Chapel
use crate::bridge::chapel::ChapelBridge;

pub struct WaveMesh {
    bridge: Arc<ChapelBridge>,
}
