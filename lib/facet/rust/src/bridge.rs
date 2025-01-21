//! Facet Bridge Interface
//! Author: @isdood
//! Created: 2025-01-21 13:11:31 UTC

use std::{
    ffi::{c_void, CStr, CString},
    mem::MaybeUninit,
    ptr::NonNull,
    sync::atomic::{AtomicBool, Ordering},
};

use crossbeam::channel::{bounded, Receiver, Sender};
use parking_lot::RwLock;
use rayon::ThreadPoolBuilder;
use thiserror::Error;

use crate::{
    crystal::CrystalLattice,
    compute::ComputeEngine,
    resonance::ResonanceState,
    types::{Result, Vec3d},
};

/// Bridge initialization status
static BRIDGE_INITIALIZED: AtomicBool = AtomicBool::new(false);

/// Bridge configuration from Zig
#[repr(C)]
#[derive(Debug, Clone)]
pub struct BridgeConfig {
    /// Memory allocation strategy
    pub allocator_type: u8,
    /// Thread pool size
    pub thread_count: u32,
    /// Enable SIMD operations
    pub enable_simd: bool,
    /// Debug level
    pub debug_level: u8,
    /// Reserved for future use
    _padding: [u8; 4],
}

/// Computation context shared with Zig
#[repr(C)]
#[derive(Debug, Clone)]
pub struct ComputeContext {
    /// Operation identifier
    pub op_id: u64,
    /// Crystal clarity
    pub clarity: f64,
    /// Resonance factor
    pub resonance: f64,
    /// Error code
    pub error_code: u32,
    /// Status flags
    pub flags: u32,
}

/// Bridge error types
#[derive(Error, Debug)]
pub enum BridgeError {
    #[error("Bridge already initialized")]
    AlreadyInitialized,
    #[error("Bridge not initialized")]
    NotInitialized,
    #[error("Thread pool creation failed")]
    ThreadPoolError,
    #[error("Crystal lattice error: {0}")]
    CrystalError(String),
    #[error("Computation error: {0}")]
    ComputeError(String),
    #[error("Memory allocation error")]
    AllocationError,
}

/// Bridge state manager
pub struct Bridge {
    /// Configuration
    config: BridgeConfig,
    /// Compute engine
    engine: ComputeEngine,
    /// Crystal lattice
    crystal: CrystalLattice,
    /// Resonance state
    resonance: ResonanceState,
    /// Command channel
    cmd_tx: Sender<ComputeCommand>,
    cmd_rx: Receiver<ComputeCommand>,
    /// Last error message
    last_error: RwLock<Option<String>>,
}

/// Compute command type
#[derive(Debug)]
enum ComputeCommand {
    /// Perform computation
    Compute {
        data: Vec<u8>,
        context: ComputeContext,
        response_tx: Sender<Result<Vec<u8>>>,
    },
    /// Shutdown bridge
    Shutdown,
}

impl Bridge {
    /// Create new bridge instance
    pub fn new(config: BridgeConfig) -> Result<Self, BridgeError> {
        if BRIDGE_INITIALIZED.load(Ordering::SeqCst) {
            return Err(BridgeError::AlreadyInitialized);
        }

        // Initialize thread pool
        ThreadPoolBuilder::new()
        .num_threads(config.thread_count as usize)
        .build_global()
        .map_err(|_| BridgeError::ThreadPoolError)?;

        // Create command channel
        let (cmd_tx, cmd_rx) = bounded(32);

        // Create bridge instance
        let bridge = Self {
            config,
            engine: ComputeEngine::new(),
            crystal: CrystalLattice::new(),
            resonance: ResonanceState::new(),
            cmd_tx,
            cmd_rx,
            last_error: RwLock::new(None),
        };

        BRIDGE_INITIALIZED.store(true, Ordering::SeqCst);
        Ok(bridge)
    }

    /// Process compute commands
    pub fn process_commands(&self) {
        while let Ok(cmd) = self.cmd_rx.recv() {
            match cmd {
                ComputeCommand::Compute { data, context, response_tx } => {
                    let result = self.handle_computation(data, context);
                    let _ = response_tx.send(result);
                }
                ComputeCommand::Shutdown => break,
            }
        }
    }

    /// Handle computation request
    fn handle_computation(&self, data: Vec<u8>, context: ComputeContext) -> Result<Vec<u8>> {
        // Update crystal and resonance state
        self.crystal.update_clarity(context.clarity);
        self.resonance.update_factor(context.resonance);

        // Perform computation
        let result = self.engine.compute(&data, &self.crystal, &self.resonance)?;

        Ok(result)
    }

    /// Set error message
    fn set_error(&self, error: &BridgeError) {
        let msg = error.to_string();
        *self.last_error.write() = Some(msg);
    }
}

/// FFI interface

#[no_mangle]
pub extern "C" fn rust_init(config: *const BridgeConfig) -> bool {
    let config = unsafe { *config };
    match Bridge::new(config) {
        Ok(_) => true,
        Err(e) => {
            eprintln!("Bridge initialization failed: {}", e);
            false
        }
    }
}

#[no_mangle]
pub extern "C" fn rust_cleanup() {
    BRIDGE_INITIALIZED.store(false, Ordering::SeqCst);
}

#[no_mangle]
pub extern "C" fn rust_compute(
    ctx: *mut ComputeContext,
    input: *const Vec3d,
) -> Vec3d {
    let context = unsafe { &mut *ctx };
    let input_vec = unsafe { (*input).to_vec() };

    let bridge = match BRIDGE_INITIALIZED.load(Ordering::SeqCst) {
        true => Bridge::new(BridgeConfig {
            allocator_type: 0,
            thread_count: 1,
            enable_simd: true,
            debug_level: 0,
            _padding: [0; 4],
        }).unwrap(),
        false => return Vec3d::default(),
    };

    let (response_tx, response_rx) = bounded(1);
    let cmd = ComputeCommand::Compute {
        data: input_vec,
        context: context.clone(),
        response_tx,
    };

    if bridge.cmd_tx.send(cmd).is_err() {
        context.error_code = 1;
        return Vec3d::default();
    }

    match response_rx.recv() {
        Ok(Ok(result)) => Vec3d::from_slice(&result),
        Ok(Err(_)) | Err(_) => {
            context.error_code = 2;
            Vec3d::default()
        }
    }
}

#[no_mangle]
pub extern "C" fn rust_get_error_message(error_code: u32) -> *const i8 {
    let msg = match error_code {
        1 => "Command send failed",
        2 => "Computation failed",
        _ => "Unknown error",
    };

    CString::new(msg)
    .map(|s| s.into_raw() as *const i8)
    .unwrap_or(std::ptr::null())
}

#[no_mangle]
pub extern "C" fn rust_release_string(ptr: *mut i8) {
    if !ptr.is_null() {
        unsafe {
            let _ = CString::from_raw(ptr as *mut _);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bridge_init() {
        let config = BridgeConfig {
            allocator_type: 0,
            thread_count: 2,
            enable_simd: true,
            debug_level: 0,
            _padding: [0; 4],
        };

        assert!(rust_init(&config));
        rust_cleanup();
    }

    #[test]
    fn test_computation() {
        let config = BridgeConfig {
            allocator_type: 0,
            thread_count: 1,
            enable_simd: true,
            debug_level: 0,
            _padding: [0; 4],
        };

        assert!(rust_init(&config));

        let mut context = ComputeContext {
            op_id: 1,
            clarity: 0.95,
            resonance: 0.98,
            error_code: 0,
            flags: 0,
        };

        let input = Vec3d::new(1.0, 2.0, 3.0);
        let result = rust_compute(&mut context, &input);

        assert_eq!(context.error_code, 0);
        assert!(result != Vec3d::default());

        rust_cleanup();
    }
}
