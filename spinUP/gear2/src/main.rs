#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[link_section = ".text.boot"]
#[no_mangle]
pub extern "C" fn _start() -> ! {
    // Write to VGA for initial visual feedback
    unsafe {
        let vga = 0xb8000 as *mut u16;
        *vga = 0x0F47;      // White on black 'G'
        *vga.add(1) = 0x0F32; // White on black '2'
    }

    // Initialize CPU state
    unsafe {
        // Disable interrupts
        core::arch::asm!("cli");

        // Set up stack
        core::arch::asm!("mov esp, 0x7C00");

        // Enable A20 line
        enable_a20();

        // Enter protected mode
        enter_protected_mode();
    }

    loop {}
}

unsafe fn enable_a20() {
    // Fast A20 method
    let mut port_a = 0x92u8;
    port_a |= 2;
    core::arch::asm!("out 0x92, al", in("al") port_a);
}

unsafe fn enter_protected_mode() {
    // Load GDT
    let gdtr = GDTDescriptor {
        size: (core::mem::size_of::<[GDTEntry; 3]>() - 1) as u16,
        offset: &GDT as *const _ as u64,
    };

    core::arch::asm!("lgdt [{0}]", in(reg) &gdtr);

    // Enable protected mode
    core::arch::asm!(
        "mov eax, cr0",
        "or eax, 1",
        "mov cr0, eax"
    );
}

#[repr(packed)]
struct GDTDescriptor {
    size: u16,
    offset: u64,
}

#[repr(packed)]
#[derive(Clone, Copy)]
struct GDTEntry {
    limit_low: u16,
    base_low: u16,
    base_middle: u8,
    access: u8,
    granularity: u8,
    base_high: u8,
}

#[used]
static GDT: [GDTEntry; 3] = [
    // Null descriptor
    GDTEntry {
        limit_low: 0,
        base_low: 0,
        base_middle: 0,
        access: 0,
        granularity: 0,
        base_high: 0,
    },
// Code segment
GDTEntry {
    limit_low: 0xFFFF,
    base_low: 0,
    base_middle: 0,
    access: 0x9A,     // Present, Ring 0, Code segment
    granularity: 0xCF, // 4KB granularity, 32-bit protected mode
    base_high: 0,
},
// Data segment
GDTEntry {
    limit_low: 0xFFFF,
    base_low: 0,
    base_middle: 0,
    access: 0x92,     // Present, Ring 0, Data segment
    granularity: 0xCF, // 4KB granularity, 32-bit protected mode
    base_high: 0,
},
];
