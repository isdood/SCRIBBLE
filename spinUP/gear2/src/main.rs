#![no_std]
#![no_main]

use core::panic::PanicInfo;
use core::ptr;
use unstable_matter::UnstableMatter;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

const VGA_BUFFER: usize = 0xb8000;

// GDT structures
#[repr(C, packed(1))]
struct GDTDescriptor {
    size: u16,
    offset: u32,
}

#[repr(C, packed(1))]
#[derive(Clone, Copy)]
struct GDTEntry {
    limit_low: u16,
    base_low: u16,
    base_middle: u8,
    access: u8,
    granularity: u8,
    base_high: u8,
}

// Page table structures
#[repr(C, align(4096))]
struct PageTable {
    entries: [u64; 512],
}

#[link_section = ".gdt"]
static GDT: [GDTEntry; 4] = [
    // Null descriptor
    GDTEntry {
        limit_low: 0,
        base_low: 0,
        base_middle: 0,
        access: 0,
        granularity: 0,
        base_high: 0,
    },
// 32-bit Code segment
GDTEntry {
    limit_low: 0xFFFF,
    base_low: 0,
    base_middle: 0,
    access: 0x9A,     // Present, Ring 0, Code
    granularity: 0xCF, // 4KB granularity, 32-bit protected mode
    base_high: 0,
},
// 64-bit Code segment
GDTEntry {
    limit_low: 0,
    base_low: 0,
    base_middle: 0,
    access: 0x9A,     // Present, Ring 0, Code
    granularity: 0x20, // Long mode
    base_high: 0,
},
// Data segment
GDTEntry {
    limit_low: 0,
    base_low: 0,
    base_middle: 0,
    access: 0x92,     // Present, Ring 0, Data
    granularity: 0,
    base_high: 0,
},
];

// Aligned page tables
#[link_section = ".page_tables"]
static mut PML4: PageTable = PageTable { entries: [0; 512] };
#[link_section = ".page_tables"]
static mut PDPT: PageTable = PageTable { entries: [0; 512] };
#[link_section = ".page_tables"]
static mut PD: PageTable = PageTable { entries: [0; 512] };

#[no_mangle]
#[link_section = ".start"]
pub extern "C" fn _start() -> ! {
    unsafe {
        // Disable interrupts
        core::arch::asm!("cli");

        // Enable A20 line
        enable_a20();

        // Enter protected mode
        enter_protected_mode();

        // We should never reach here
        loop {}
    }
}

unsafe fn enable_a20() {
    const FAST_A20_PORT: usize = 0x92;
    let port = UnstableMatter::<u8>::at(FAST_A20_PORT);
    let val = port.read() | 2;
    port.write(val);
}

unsafe fn enter_protected_mode() {
    let gdt_desc = GDTDescriptor {
        size: (core::mem::size_of::<[GDTEntry; 4]>() - 1) as u16,
        offset: &GDT as *const _ as u32,
    };

    // Setup initial identity mapping for first 2MB
    setup_page_tables();

    core::arch::asm!(
        // 16-bit code
        ".code16",
        // Load GDT
        "lgdt [{0:e}]",  // Use :e modifier to force 32-bit addressing
        // Enable protected mode
        "mov eax, cr0",
        "or al, 1",
        "mov cr0, eax",
        // Far jump to 32-bit code
        ".byte 0xEA",       // Far jump opcode
        ".long 2f",         // 32-bit offset
        ".word 0x08",       // Code segment selector
        // 32-bit protected mode
        ".code32",
        "2:",
        // Set up segment registers
        "mov ax, 0x10",
        "mov ds, ax",
        "mov es, ax",
        "mov fs, ax",
        "mov gs, ax",
        "mov ss, ax",
        // Enable PAE
        "mov eax, cr4",
        "or eax, 1 << 5",
        "mov cr4, eax",
        // Load page table
        "mov eax, {1:e}",  // Use :e modifier for 32-bit value
        "mov cr3, eax",
        // Enable long mode
        "mov ecx, 0xC0000080",
        "rdmsr",
        "or eax, 1 << 8",
        "wrmsr",
        // Enable paging
        "mov eax, cr0",
        "or eax, 1 << 31",
        "mov cr0, eax",
        // Jump to 64-bit mode
        ".byte 0xEA",       // Far jump opcode
        ".long 3f",         // 32-bit offset
        ".word 0x10",       // Code segment selector
        ".code64",
        "3:",
        // Now in 64-bit mode
        "mov rax, {2}",
        "jmp rax",
        in(reg) &gdt_desc,
                     in(reg) &raw const PML4 as *const PageTable,
                     sym rust_main,
                     options(noreturn)
    );
}

unsafe fn setup_page_tables() {
    PML4.entries[0] = (&raw const PDPT as *const PageTable as u64) | 0x3;
    PDPT.entries[0] = (&raw const PD as *const PageTable as u64) | 0x3;
    PD.entries[0] = 0x83;  // Present + Write + Huge (2MB)
}

#[no_mangle]
extern "C" fn rust_main() -> ! {
    let vga = unsafe { UnstableMatter::<u16>::at(VGA_BUFFER) };

    // Write "G2" to screen (White on black)
    vga.write(0x0F47);
    unsafe {
        let next_char = vga.as_ptr().add(1);
        ptr::write_volatile(next_char, 0x0F32);
    }

    loop {}
}

#[used]
#[link_section = ".stack"]
static mut STACK: [u8; 4096] = [0; 4096];
