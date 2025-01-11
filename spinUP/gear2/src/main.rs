#![no_std]
#![no_main]

use core::panic::PanicInfo;
use unstable_matter::UnstableMatter;

// GDT Structures
#[repr(C, packed)]
struct GDTEntry {
    limit_low: u16,
    base_low: u16,
    base_middle: u8,
    access: u8,
    granularity: u8,
    base_high: u8,
}

#[repr(C, packed)]
struct GDTPointer {
    limit: u16,
    base: u32,
}

#[repr(C, align(8))]
struct GDTTable {
    entries: [GDTEntry; 3]
}

// Page Tables
#[repr(C, align(4096))]
struct PageTable {
    entries: [u64; 512],
}

#[repr(C, align(4096))]
struct PageTables {
    pml4: PageTable,
    pdpt: PageTable,
    pd: PageTable,
}

// Stack
#[repr(align(16))]
struct Stack {
    data: [u8; 4096]
}

// Static Resources
static mut GDT: GDTTable = GDTTable {
    entries: [
        // Null descriptor
        GDTEntry {
            limit_low: 0,
            base_low: 0,
            base_middle: 0,
            access: 0,
            granularity: 0,
            base_high: 0
        },
        // Code segment (0x08)
        GDTEntry {
            limit_low: 0xFFFF,
            base_low: 0,
            base_middle: 0,
            access: 0x9A,     // Present, Ring 0, Code Segment, Executable, Readable
            granularity: 0xAF, // 4k blocks, 64-bit mode, limit 0xF
            base_high: 0
        },
        // Data segment (0x10)
        GDTEntry {
            limit_low: 0xFFFF,
            base_low: 0,
            base_middle: 0,
            access: 0x92,     // Present, Ring 0, Data Segment, Writable
            granularity: 0xAF, // 4k blocks, 64-bit mode, limit 0xF
            base_high: 0
        },
    ]
};

static mut PAGE_TABLES: PageTables = PageTables {
    pml4: PageTable { entries: [0; 512] },
    pdpt: PageTable { entries: [0; 512] },
    pd: PageTable { entries: [0; 512] },
};

static mut STACK: Stack = Stack {
    data: [0; 4096]
};

// Entry Point
#[no_mangle]
pub extern "C" fn _start() -> ! {
    unsafe {
        enter_long_mode();
    }
}

unsafe fn enter_long_mode() -> ! {
    disable_interrupts();
    setup_page_tables();
    setup_gdt();
    enable_paging();
    jump_to_long_mode()
}

unsafe fn disable_interrupts() {
    core::arch::asm!("cli");
    disable_pic();
}

unsafe fn setup_page_tables() {
    let pml4 = UnstableMatter::at(&raw const PAGE_TABLES.pml4 as *const _ as usize);
    let pdpt = UnstableMatter::at(&raw const PAGE_TABLES.pdpt as *const _ as usize);
    let pd = UnstableMatter::at(&raw const PAGE_TABLES.pd as *const _ as usize);

    // Identity map first 2MB
    let entries = &mut PAGE_TABLES.pml4.entries;
    entries[0] = pdpt.addr() as u64 | 0x3;

    let entries = &mut PAGE_TABLES.pdpt.entries;
    entries[0] = pd.addr() as u64 | 0x3;

    let entries = &mut PAGE_TABLES.pd.entries;
    entries[0] = 0x83;  // Present + Write + Huge (2MB)
}

unsafe fn setup_gdt() {
    let gdt = UnstableMatter::at(&raw const GDT.entries as *const _ as usize);
    let gdt_ptr = GDTPointer {
        limit: (core::mem::size_of::<[GDTEntry; 3]>() - 1) as u16,
        base: gdt.addr() as u32,
    };
    core::arch::asm!("lgdtl ({0:e})", in(reg) &gdt_ptr);
}

unsafe fn enable_paging() {
    let pml4_addr = &raw const PAGE_TABLES.pml4 as *const _ as u32;

    core::arch::asm!(
        // Enable PAE
        "movl %cr4, %eax",
        "btsl $5, %eax",
        "movl %eax, %cr4",

        // Load CR3
        "movl {0:e}, %eax",
        "movl %eax, %cr3",

        // Enable long mode
        "movl $0xC0000080, %ecx",
        "rdmsr",
        "btsl $8, %eax",
        "wrmsr",

        // Enable paging
        "movl %cr0, %eax",
        "btsl $31, %eax",
        "btsl $0, %eax",
        "movl %eax, %cr0",
        in(reg) pml4_addr,
    );
}

unsafe fn jump_to_long_mode() -> ! {
    let stack_top = &raw const STACK.data as *const u8 as u64 + 4096;

    core::arch::asm!(
        // Jump to 64-bit code
        "pushl $0x08",
        "pushl $1f",
        "lretl",

        ".align 8",
        ".code64",
        "1:",

        // Setup segments
        "movw $0x10, %ax",
        "movw %ax, %ds",
        "movw %ax, %es",
        "movw %ax, %fs",
        "movw %ax, %gs",
        "movw %ax, %ss",

        // Setup stack
        "movq {0}, %rsp",
        "xorq %rbp, %rbp",

        // Jump to Rust
        "pushq {1}",
        "retq",

        in(reg) stack_top,
                     sym rust_main,
                     options(noreturn)
    );
}

unsafe fn disable_pic() {
    let mut port = |addr: u16| UnstableMatter::<u8>::at(addr as usize);

    // Remap and mask PIC
    port(0x20).write(0x11);
    port(0xA0).write(0x11);
    port(0x21).write(0x20);
    port(0xA1).write(0x28);
    port(0x21).write(0x04);
    port(0xA1).write(0x02);
    port(0x21).write(0x01);
    port(0xA1).write(0x01);
    port(0x21).write(0xFF);
    port(0xA1).write(0xFF);
}

#[no_mangle]
pub extern "C" fn rust_main() -> ! {
    let vga = unsafe { UnstableMatter::<u16>::at(0xB8000) };
    let msg = b"Long Mode OK!";

    // Clear screen
    for i in 0..80*25 {
        vga.write(0x0F00);
    }

    // Write message
    for (i, &byte) in msg.iter().enumerate() {
        vga.write(0x0F00 | byte as u16);
    }

    loop {
        core::arch::asm!("hlt", options(nomem, nostack));
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
