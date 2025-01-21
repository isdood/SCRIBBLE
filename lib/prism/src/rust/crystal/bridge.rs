// bridge.rs - Bridge between Rust and Zig crystal implementations
// Created by: isdood
// Date: 2025-01-21 11:03:20 UTC

use std::ffi::{c_void, CString};
use std::os::raw::{c_char, c_int, c_uint, c_ulonglong};
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::{Arc, Mutex};

use crate::types::{PrismError, PrismResult};

/// Crystal system types matching Zig implementation
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum CrystalSystem {
    Cubic,
    Tetragonal,
    Hexagonal,
    Orthorhombic,
    Monoclinic,
    Triclinic,
    Rhombohedral,
}

/// Bridge to Zig crystal lattice
#[repr(C)]
pub struct CrystalBridge {
    handle: *mut c_void,
    system: CrystalSystem,
    initialized: AtomicBool,
    node_count: AtomicU64,
}

/// Crystal node representation
#[repr(C)]
pub struct CrystalNode {
    position: [f64; 3],
    energy: f64,
    stability: f64,
}

/// Crystal configuration
#[repr(C)]
pub struct CrystalConfig {
    system: CrystalSystem,
    initial_capacity: c_uint,
    stability_threshold: f64,
}

extern "C" {
    fn zig_crystal_create(config: *const CrystalConfig) -> *mut c_void;
    fn zig_crystal_destroy(handle: *mut c_void);
    fn zig_crystal_add_node(handle: *mut c_void, position: *const [f64; 3]) -> *mut c_void;
    fn zig_crystal_remove_node(handle: *mut c_void, node: *mut c_void) -> c_int;
    fn zig_crystal_update(handle: *mut c_void) -> c_int;
    fn zig_crystal_get_stability(handle: *mut c_void) -> f64;
    fn zig_crystal_optimize(handle: *mut c_void) -> c_int;
}

impl CrystalBridge {
    /// Create a new crystal bridge
    pub fn new(config: CrystalConfig) -> PrismResult<Self> {
        let handle = unsafe { zig_crystal_create(&config) };
        if handle.is_null() {
            return Err(PrismError::SystemError);
        }

        Ok(Self {
            handle,
            system: config.system,
            initialized: AtomicBool::new(true),
            node_count: AtomicU64::new(0),
        })
    }

    /// Add a new node to the crystal
    pub fn add_node(&self, position: [f64; 3]) -> PrismResult<Arc<CrystalNode>> {
        if !self.initialized.load(Ordering::SeqCst) {
            return Err(PrismError::NotInitialized);
        }

        let node_ptr = unsafe { zig_crystal_add_node(self.handle, &position) };
        if node_ptr.is_null() {
            return Err(PrismError::SystemError);
        }

        self.node_count.fetch_add(1, Ordering::SeqCst);

        Ok(Arc::new(CrystalNode {
            position,
            energy: 1.0,
            stability: 1.0,
        }))
    }

    /// Remove a node from the crystal
    pub fn remove_node(&self, node: Arc<CrystalNode>) -> PrismResult<()> {
        if !self.initialized.load(Ordering::SeqCst) {
            return Err(PrismError::NotInitialized);
        }

        let result = unsafe { 
            zig_crystal_remove_node(self.handle, Arc::into_raw(node) as *mut c_void)
        };

        if result == 0 {
            self.node_count.fetch_sub(1, Ordering::SeqCst);
            Ok(())
        } else {
            Err(PrismError::SystemError)
        }
    }

    /// Update the crystal state
    pub fn update(&self) -> PrismResult<()> {
        if !self.initialized.load(Ordering::SeqCst) {
            return Err(PrismError::NotInitialized);
        }

        let result = unsafe { zig_crystal_update(self.handle) };
        if result == 0 {
            Ok(())
        } else {
            Err(PrismError::SystemError)
        }
    }

    /// Get current crystal stability
    pub fn get_stability(&self) -> PrismResult<f64> {
        if !self.initialized.load(Ordering::SeqCst) {
            return Err(PrismError::NotInitialized);
        }

        Ok(unsafe { zig_crystal_get_stability(self.handle) })
    }

    /// Optimize crystal structure
    pub fn optimize(&self) -> PrismResult<()> {
        if !self.initialized.load(Ordering::SeqCst) {
            return Err(PrismError::NotInitialized);
        }

        let result = unsafe { zig_crystal_optimize(self.handle) };
        if result == 0 {
            Ok(())
        } else {
            Err(PrismError::SystemError)
        }
    }

    /// Get current node count
    pub fn node_count(&self) -> u64 {
        self.node_count.load(Ordering::SeqCst)
    }

    /// Get crystal system type
    pub fn system(&self) -> CrystalSystem {
        self.system
    }

    /// Check if the crystal is initialized
    pub fn is_initialized(&self) -> bool {
        self.initialized.load(Ordering::SeqCst)
    }
}

impl Drop for CrystalBridge {
    fn drop(&mut self) {
        if !self.handle.is_null() {
            unsafe {
                zig_crystal_destroy(self.handle);
            }
            self.initialized.store(false, Ordering::SeqCst);
        }
    }
}

/// Safe wrapper for crystal operations
pub struct Crystal {
    bridge: Arc<CrystalBridge>,
    nodes: Arc<Mutex<Vec<Arc<CrystalNode>>>>,
}

impl Crystal {
    /// Create a new crystal instance
    pub fn new(system: CrystalSystem) -> PrismResult<Self> {
        let config = CrystalConfig {
            system,
            initial_capacity: 1024,
            stability_threshold: 0.8,
        };

        let bridge = Arc::new(CrystalBridge::new(config)?);
        
        Ok(Self {
            bridge,
            nodes: Arc::new(Mutex::new(Vec::new())),
        })
    }

    /// Add a node at the specified position
    pub fn add_node(&self, position: [f64; 3]) -> PrismResult<Arc<CrystalNode>> {
        let node = self.bridge.add_node(position)?;
        self.nodes.lock().unwrap().push(Arc::clone(&node));
        Ok(node)
    }

    /// Remove a specific node
    pub fn remove_node(&self, node: Arc<CrystalNode>) -> PrismResult<()> {
        self.bridge.remove_node(Arc::clone(&node))?;
        let mut nodes = self.nodes.lock().unwrap();
        nodes.retain(|n| !Arc::ptr_eq(n, &node));
        Ok(())
    }

    /// Update crystal state
    pub fn update(&self) -> PrismResult<()> {
        self.bridge.update()
    }

    /// Get current stability
    pub fn stability(&self) -> PrismResult<f64> {
        self.bridge.get_stability()
    }

    /// Optimize crystal structure
    pub fn optimize(&self) -> PrismResult<()> {
        self.bridge.optimize()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;
    use std::time::Duration;

    #[test]
    fn test_crystal_creation() {
        let crystal = Crystal::new(CrystalSystem::Cubic).unwrap();
        assert!(crystal.bridge.is_initialized());
        assert_eq!(crystal.bridge.node_count(), 0);
    }

    #[test]
    fn test_node_management() {
        let crystal = Crystal::new(CrystalSystem::Cubic).unwrap();
        
        let node = crystal.add_node([0.0, 0.0, 0.0]).unwrap();
        assert_eq!(crystal.bridge.node_count(), 1);

        crystal.remove_node(node).unwrap();
        assert_eq!(crystal.bridge.node_count(), 0);
    }

    #[test]
    fn test_crystal_optimization() {
        let crystal = Crystal::new(CrystalSystem::Cubic).unwrap();
        
        // Add some nodes
        for i in 0..5 {
            let x = i as f64;
            crystal.add_node([x, 0.0, 0.0]).unwrap();
        }

        crystal.optimize().unwrap();
        let stability = crystal.stability().unwrap();
        assert!(stability > 0.0);
    }

    #[test]
    fn test_concurrent_access() {
        let crystal = Arc::new(Crystal::new(CrystalSystem::Cubic).unwrap());
        let mut handles = vec![];

        for i in 0..10 {
            let crystal = Arc::clone(&crystal);
            handles.push(thread::spawn(move || {
                let x = i as f64;
                crystal.add_node([x, 0.0, 0.0]).unwrap();
                thread::sleep(Duration::from_millis(10));
                crystal.update().unwrap();
            }));
        }

        for handle in handles {
            handle.join().unwrap();
        }

        assert_eq!(crystal.bridge.node_count(), 10);
    }
}
