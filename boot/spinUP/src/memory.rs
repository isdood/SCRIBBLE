// boot/spinUP/src/memory.rs
// Last Updated: 2025-01-13 06:08:23 UTC
// Author: Caleb J.D. Terkovics (isdood)
// Current User: isdood

use core::ptr;
use core::sync::atomic::{AtomicUsize, Ordering};
use unstable_matter::{
    vector_space::VectorSpace,
    ufo_states::UFOState,
    space_config::SpaceMetadata,
};

// Constants for vector mesh configuration
const VECTOR_CELL_SIZE: usize = 4096;  // 4KB per cell
const MESH_DENSITY: usize = 16;        // 16x16x16 mesh
const TOTAL_MESH_SIZE: usize = VECTOR_CELL_SIZE * MESH_DENSITY * MESH_DENSITY * MESH_DENSITY;

static VECTOR_SPACE_PTR: AtomicUsize = AtomicUsize::new(0);
static CURRENT_POSITION: AtomicUsize = AtomicUsize::new(0);

#[repr(C, align(4096))]
pub struct AlignedMemoryRegion {
    data: [u8; VECTOR_CELL_SIZE],
}

impl AlignedMemoryRegion {
    pub const fn new() -> Self {
        AlignedMemoryRegion {
            data: [0; VECTOR_CELL_SIZE],
        }
    }

    pub fn as_ptr(&self) -> *const u8 {
        self.data.as_ptr()
    }

    pub fn as_mut_ptr(&mut self) -> *mut u8 {
        self.data.as_mut_ptr()
    }
}

#[no_mangle]
pub unsafe extern "C" fn memcpy(dest: *mut u8, src: *const u8, n: usize) -> *mut u8 {
    ptr::copy_nonoverlapping(src, dest, n);
    dest
}

#[no_mangle]
pub unsafe extern "C" fn memset(s: *mut u8, c: i32, n: usize) -> *mut u8 {
    ptr::write_bytes(s, c as u8, n);
    s
}

#[no_mangle]
pub extern "C" fn rust_eh_personality() -> ! {
    loop {}
}

pub struct VectorMeshManager {
    initialized: AtomicUsize,
    base_address: AtomicUsize,
}

impl VectorMeshManager {
    pub const fn new() -> Self {
        VectorMeshManager {
            initialized: AtomicUsize::new(0),
            base_address: AtomicUsize::new(0),
        }
    }

    pub unsafe fn init(&self, base_addr: usize) {
        if self.initialized.load(Ordering::Relaxed) == 0 {
            let metadata = SpaceMetadata::new(TOTAL_MESH_SIZE);
            let mut vector_space = VectorSpace::new(base_addr, metadata);
            vector_space.transition_state(UFOState::Hovering);

            self.base_address.store(base_addr, Ordering::SeqCst);
            VECTOR_SPACE_PTR.store(base_addr, Ordering::SeqCst);
            self.initialized.store(1, Ordering::SeqCst);
        }
    }

    unsafe fn get_vector_space(&self) -> Option<&'static mut VectorSpace> {
        let ptr = VECTOR_SPACE_PTR.load(Ordering::SeqCst);
        if ptr != 0 {
            Some(&mut *(ptr as *mut VectorSpace))
        } else {
            None
        }
    }

    unsafe fn allocate_region(&self, size: usize) -> Option<*mut u8> {
        if let Some(vs) = self.get_vector_space() {
            let current = CURRENT_POSITION.fetch_add(1, Ordering::SeqCst);
            let x = (current / (MESH_DENSITY * MESH_DENSITY)) % MESH_DENSITY;
            let y = (current / MESH_DENSITY) % MESH_DENSITY;
            let z = current % MESH_DENSITY;

            if x < MESH_DENSITY {
                let base_addr = self.base_address.load(Ordering::SeqCst);
                let offset = (x * MESH_DENSITY * MESH_DENSITY + y * MESH_DENSITY + z) * VECTOR_CELL_SIZE;
                if size <= VECTOR_CELL_SIZE {
                    return Some((base_addr + offset) as *mut u8);
                }
            }
        }
        None
    }
}

static MESH_MANAGER: VectorMeshManager = VectorMeshManager::new();

// Helper functions for memory alignment
pub fn is_aligned(addr: usize, align: usize) -> bool {
    addr & (align - 1) == 0
}

pub fn align_up(addr: usize, align: usize) -> usize {
    (addr + align - 1) & !(align - 1)
}

pub fn align_down(addr: usize, align: usize) -> usize {
    addr & !(align - 1)
}

// Initialize memory system with vector space support
pub unsafe fn init_memory(base_addr: usize) -> &'static mut VectorSpace {
    MESH_MANAGER.init(base_addr);
    MESH_MANAGER.get_vector_space()
    .expect("Memory system not initialized")
}

// Get current vector space state
pub fn get_vector_space_state(vs: &VectorSpace) -> UFOState {
    vs.get_state()
}

// Helper to transition vector space state
pub fn transition_vector_space(vs: &mut VectorSpace, new_state: UFOState) {
    vs.transition_state(new_state);
}

// Allocate aligned memory region
pub unsafe fn allocate_aligned(size: usize) -> Option<*mut u8> {
    MESH_MANAGER.allocate_region(size)
}
