// boot/spinUP/src/main.rs
// Last Updated: 2025-01-13 05:19:08 UTC
// Author: Caleb J.D. Terkovics (isdood)
// Current User: isdood

#![no_std]
#![no_main]
#![feature(naked_functions)]

use core::panic::PanicInfo;
use unstable_matter::{
    SpaceTime,
    vector_space::VectorSpace,
    ufo_states::UFOState,
    space_config::{SpaceConfig, SpaceMetadata},
    vector::Vector3D,
    MemoryAddress,
    Dimensions,
};

// Memory configuration constants
const KERNEL_LOAD_ADDR: u64 = 0x100000;  // Load kernel at 1MB
const KERNEL_SECTOR_START: u16 = 33;     // Kernel starts at sector 33
const SECTORS_TO_READ: u16 = 100;        // Adjust based on kernel size
const VECTOR_CELL_SIZE: usize = 4096;    // 4KB per cell
const MESH_DENSITY: usize = 16;          // 16x16x16 mesh
const VGA_BUFFER: *mut u8 = 0xb8000 as *mut u8;
static mut VGA_CURSOR: usize = 0;

#[repr(C)]
pub struct BootParams {
    pub kernel_load_addr: u32,
    pub kernel_size: u32,
    pub space_metadata: *const SpaceMetadata,
    pub vector_space: *const VectorSpace,
}

// Initialize space configuration
fn init_space_config() -> SpaceConfig {
    SpaceConfig::new(
        Vector3D::new(MESH_DENSITY, MESH_DENSITY, MESH_DENSITY),
                     Vector3D::new(VECTOR_CELL_SIZE, VECTOR_CELL_SIZE, VECTOR_CELL_SIZE)
    )
}

// Initialize vector space
fn init_vector_space(base_addr: usize) -> VectorSpace {
    let metadata = SpaceMetadata::new(VECTOR_CELL_SIZE * MESH_DENSITY.pow(3));
    VectorSpace::new(base_addr, metadata)
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

unsafe fn read_disk_sector(sector: u16, buffer: *mut u8) {
    let mut disk_packet = [0u8; 16];
    disk_packet[0] = 16;    // Size of packet
    disk_packet[1] = 0;     // Reserved
    disk_packet[2] = 1;     // Number of sectors to read
    disk_packet[3] = 0;     // Reserved
    disk_packet[4..8].copy_from_slice(&(buffer as u32).to_le_bytes());
    disk_packet[8..12].copy_from_slice(&(sector as u32).to_le_bytes());
    disk_packet[12..16].fill(0);

    core::arch::asm!(
        ".code32",
        "mov ah, 0x42",
        "mov dl, 0x00",
        "int 0x13",
        in("si") disk_packet.as_ptr(),
                     options(preserves_flags)
    );
}

unsafe fn print(s: &str) {
    for byte in s.bytes() {
        let char_ptr = (VGA_BUFFER as *mut u16).add(VGA_CURSOR);
        *char_ptr = (0x0F << 8) | byte as u16; // White on black
        VGA_CURSOR += 1;
    }
}

unsafe fn println(s: &str) {
    print(s);
    VGA_CURSOR = VGA_CURSOR + 80 - VGA_CURSOR % 80; // New line
}

#[no_mangle]
pub unsafe extern "C" fn _start() -> ! {
    // Initialize space-time system
    println("spinUP: Initializing space-time system...");
    let space_config = init_space_config();
    let mut vector_space = init_vector_space(KERNEL_LOAD_ADDR as usize);

    // Set up the space-time region for kernel loading
    println("spinUP: Setting up kernel space...");
    let mut kernel_space = SpaceTime::<u8>::new(
        KERNEL_LOAD_ADDR as usize,
        (SECTORS_TO_READ as usize) * 512,
                                                0
    );

    // Load kernel into vector space
    println("spinUP: Loading kernel...");
    vector_space.transition_state(UFOState::Hovering);

    let mut progress = 0;
    for sector in 0..SECTORS_TO_READ {
        let sector_offset = (sector * 512) as usize;
        read_disk_sector(
            KERNEL_SECTOR_START + sector,
            (KERNEL_LOAD_ADDR as usize + sector_offset) as *mut u8
        );

        if sector % 10 == 0 {
            print(".");
            progress += 1;
        }
    }
    println("\nspinUP: Kernel loaded successfully");

    vector_space.transition_state(UFOState::Landed);
    println("spinUP: Vector space initialized");

    // Set up boot parameters with proper memory addressing
    let boot_params = BootParams {
        kernel_load_addr: KERNEL_LOAD_ADDR as u32,
        kernel_size: (SECTORS_TO_READ as u32) * 512,
        space_metadata: vector_space.get_metadata() as *const SpaceMetadata,
        vector_space: &vector_space as *const VectorSpace,
    };

    println("spinUP: Jumping to kernel...");

    // Get kernel entry point from memory address
    let kernel_entry = KERNEL_LOAD_ADDR as *const fn(*const BootParams) -> !;
    (*kernel_entry)(&boot_params);

    loop {
        core::arch::asm!("hlt");
    }
}

#[no_mangle]
pub extern "C" fn rust_main() -> ! {
    unsafe {
        println("spinUP: System initialized");
    }
    loop {
        unsafe {
            core::arch::asm!("hlt");
        }
    }
}
