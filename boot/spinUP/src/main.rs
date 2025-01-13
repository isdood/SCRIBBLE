// boot/spinUP/src/main.rs
// Last Updated: 2025-01-13 06:04:16 UTC
// Author: Caleb J.D. Terkovics (isdood)
// Current User: isdood

#![no_std]
#![no_main]
#![feature(naked_functions)]

use spinup::{
    serial,
    serial_println,
    memory::{self, init_memory, is_aligned, AlignedMemoryRegion},
};

use core::panic::PanicInfo;
use unstable_matter::{
    SpaceTime,
    vector_space::VectorSpace,
    ufo_states::UFOState,
    space_config::{SpaceConfig, SpaceMetadata},
    vector::Vector3D,
};

// Memory configuration constants
const KERNEL_LOAD_ADDR: u64 = 0x100000;  // Load kernel at 1MB
const KERNEL_SECTOR_START: u16 = 33;     // Kernel starts at sector 33
const SECTORS_TO_READ: u16 = 100;        // Adjust based on kernel size
const VECTOR_CELL_SIZE: usize = 4096;    // 4KB per cell
const MESH_DENSITY: usize = 16;          // 16x16x16 mesh
const VGA_BUFFER: *mut u8 = 0xb8000 as *mut u8;
static mut VGA_CURSOR: usize = 0;

#[repr(C, align(4096))]
pub struct MainBootParams {
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

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    unsafe {
        println("PANIC: System halted");
        loop {
            core::arch::asm!("hlt");
        }
    }
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

#[naked]
#[no_mangle]
#[link_section = ".text._start"]
pub unsafe extern "C" fn _start() -> ! {
    core::arch::naked_asm!(
        // Save registers
        "push rax",
        "push rbx",
        "push rcx",
        "push rdx",
        "push rsi",
        "push rdi",
        "push rbp",

        // Call our real entry point
        "call {}",

        // We shouldn't return, but if we do:
        "cli",
        "2:",
        "hlt",
        "jmp 2b",

        sym real_start
    );
}

#[no_mangle]
unsafe extern "C" fn real_start() -> ! {
    serial::init_serial();
    serial_println!("spinUP: Bootloader starting...");

    // Initialize vector space with proper memory checks
    let vector_space = {
        // Verify alignment
        assert!(
            is_aligned(KERNEL_LOAD_ADDR as usize, VECTOR_CELL_SIZE),
                "Kernel load address must be aligned to vector cell size"
        );

        // Create space configuration
        let space_config = init_space_config();

        // Initialize memory system
        let vs = init_memory(KERNEL_LOAD_ADDR as usize);

        // Ensure proper initialization state
        if vs.get_state() != UFOState::Hovering {
            serial_println!("spinUP: WARNING - Vector space in unexpected state: {:?}", vs.get_state());
            memory::transition_vector_space(vs, UFOState::Hovering);

            // Verify state transition
            assert_eq!(
                vs.get_state(),
                       UFOState::Hovering,
                       "Failed to transition vector space to hovering state"
            );
        }

        serial_println!(
            "spinUP: Vector space initialized at {:#x} with {} cells",
            KERNEL_LOAD_ADDR,
            MESH_DENSITY.pow(3)
        );
        vs
    };

    // Set up the space-time region for kernel loading
    println("spinUP: Setting up kernel space...");
    let kernel_space = {
        let size = (SECTORS_TO_READ as usize) * 512;
        SpaceTime::<u8>::new(
            KERNEL_LOAD_ADDR as usize,
            size,
            0
        )
    };

    // Load kernel into vector space
    println("spinUP: Loading kernel...");
    assert_eq!(
        vector_space.get_state(),
               UFOState::Hovering,
               "Vector space not in hovering state"
    );

    let mut aligned_buffer = AlignedMemoryRegion::new();
    for sector in 0..SECTORS_TO_READ {
        let sector_offset = (sector * 512) as usize;
        let target_addr = KERNEL_LOAD_ADDR as usize + sector_offset;

        read_disk_sector(
            KERNEL_SECTOR_START + sector,
            aligned_buffer.as_mut_ptr()
        );

        // Copy from aligned buffer to target location
        core::ptr::copy_nonoverlapping(
            aligned_buffer.as_ptr(),
                                       target_addr as *mut u8,
                                       512
        );

        if sector % 10 == 0 {
            print(".");
        }
    }
    println("\nspinUP: Kernel loaded successfully");

    // Transition vector space state
    vector_space.transition_state(UFOState::Landed);
    println("spinUP: Vector space landed");

    // Set up boot parameters
    let boot_params = MainBootParams {
        kernel_load_addr: KERNEL_LOAD_ADDR as u32,
        kernel_size: (SECTORS_TO_READ as u32) * 512,
        space_metadata: vector_space.get_metadata() as *const SpaceMetadata,
        vector_space: vector_space as *const VectorSpace,
    };

    // Log memory configuration
    serial_println!(
        "spinUP: Memory configuration:\n\
- Kernel at: {:#x}\n\
- Kernel size: {} bytes\n\
- Vector cells: {}\n\
- Cell size: {} bytes",
KERNEL_LOAD_ADDR,
boot_params.kernel_size,
MESH_DENSITY.pow(3),
                    VECTOR_CELL_SIZE
    );

    println("spinUP: Jumping to kernel...");

    // Jump to kernel entry point
    let kernel_entry = KERNEL_LOAD_ADDR as *const fn(*const MainBootParams) -> !;
    (*kernel_entry)(&boot_params)
}

#[no_mangle]
pub extern "C" fn rust_main() -> ! {
    unsafe {
        println("spinUP: System initialized");
        loop {
            core::arch::asm!("hlt");
        }
    }
}
