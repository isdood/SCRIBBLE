//! Architecture-specific implementations for crystal computing

mod platform;
mod features;
mod dispatch;

pub use platform::{Platform, Architecture};
pub use features::{CpuFeature, detect_features};
pub use dispatch::Dispatcher;

/// Represents a hardware-specific shard implementation
#[derive(Debug)]
pub struct Shard {
    platform: Platform,
    features: Vec<CpuFeature>,
    dispatcher: Dispatcher,
}

impl Shard {
    /// Creates a new shard for the current hardware
    pub fn new() -> Self {
        let platform = Platform::detect();
        let features = detect_features();
        let dispatcher = Dispatcher::new(&platform, &features);

        Shard {
            platform,
            features,
            dispatcher,
        }
    }

    /// Returns the current platform architecture
    pub fn architecture(&self) -> Architecture {
        self.platform.architecture()
    }

    /// Checks if a specific CPU feature is available
    pub fn has_feature(&self, feature: CpuFeature) -> bool {
        self.features.contains(&feature)
    }

    /// Gets the optimal dispatch path for crystal operations
    pub fn get_dispatch_path(&self) -> &str {
        self.dispatcher.optimal_path()
    }
}
