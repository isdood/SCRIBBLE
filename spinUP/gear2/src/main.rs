#![no_std]
#![no_main]

use core::panic::PanicInfo;

// GDT setup
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

// Page table structures
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

// Static structures
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
            access: 0x9A,     // Present, Ring 0, Code Segment, Executable, Direction 0, Readable
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

#[repr(align(16))]
struct Stack {
    data: [u8; 4096]
}

static mut STACK: Stack = Stack {
    data: [0; 4096]
};

// Entry point - This is where execution begins after bootloader handoff
#[no_mangle]
pub extern "C" fn _start() -> ! {
    unsafe {
        enter_long_mode();
    }
}

unsafe fn enter_long_mode() -> ! {
    // First, disable all interrupts properly
    core::arch::asm!("cli");

    // Disable PIC interrupts
    disable_pic();

    // Prepare GDT
    let gdt_ptr = GDTPointer {
        limit: (core::mem::size_of::<[GDTEntry; 3]>() - 1) as u16,
        base: &GDT.entries as *const _ as u32,
    };

    // Identity map first 2MB
    PAGE_TABLES.pml4.entries[0] = (&PAGE_TABLES.pdpt as *const _ as u64) | 0x3;
    PAGE_TABLES.pdpt.entries[0] = (&PAGE_TABLES.pd as *const _ as u64) | 0x3;
    PAGE_TABLES.pd.entries[0] = 0x83;  // Present + Write + Huge (2MB)

    let stack_top = &STACK.data as *const u8 as u64 + 4096;

    core::arch::asm!(
        ".code32",

        // Load GDT before we do anything else
        "lgdtl ({0:r})",

                     // Enable PAE first
                     "movl %cr4, %eax",
                     "btsl $5, %eax",      // Set PAE bit (bit 5)
    "movl %eax, %cr4",

    // Load CR3 with PML4 table
    "movl {1:r}, %eax",
    "movl %eax, %cr3",

    // Enable long mode
    "movl $0xC0000080, %ecx",
    "rdmsr",
    "btsl $8, %eax",      // Set LME bit (bit 8)
    "wrmsr",

    // Enable paging and protection
    "movl %cr0, %eax",
    "btsl $31, %eax",     // Enable paging (bit 31)
    "btsl $0, %eax",      // Enable protected mode (bit 0)
    "movl %eax, %cr0",

    // Reload code segment
    "pushw $0x08",
    "pushl $1f",
    "lretl",

    ".align 8",
    ".code64",
    "1:",

    // Load data segments
    "movw $0x10, %ax",
    "movw %ax, %ds",
    "movw %ax, %es",
    "movw %ax, %fs",
    "movw %ax, %gs",
    "movw %ax, %ss",

    // Set up stack
    "movq {2}, %rsp",
    "xorq %rbp, %rbp",

    // Jump to Rust
    "jmpq *{3}",

    in(reg) &gdt_ptr,
                     in(reg) &PAGE_TABLES.pml4 as *const _ as u32,
                     in(reg) stack_top,
                     sym rust_main,
                     options(noreturn, att_syntax)
    );
}

// Separate function to disable PIC
unsafe fn disable_pic() {
    // Remap PIC to avoid conflicts
    core::arch::asm!(
        "movb $0x11, %al",
        "outb %al, $0x20",    // Start PIC1 initialization
        "outb %al, $0xA0",    // Start PIC2 initialization
        "movb $0x20, %al",
        "outb %al, $0x21",    // Map PIC1 to 0x20-0x27
        "movb $0x28, %al",
        "outb %al, $0xA1",    // Map PIC2 to 0x28-0x2F
        "movb $0x04, %al",
        "outb %al, $0x21",    // PIC1 is master
        "movb $0x02, %al",
        "outb %al, $0xA1",    // PIC2 is slave
        "movb $0x01, %al",
        "outb %al, $0x21",    // 8086 mode for PIC1
        "outb %al, $0xA1",    // 8086 mode for PIC2
        "movb $0xFF, %al",
        "outb %al, $0x21",    // Mask all interrupts on PIC1
        "outb %al, $0xA1",    // Mask all interrupts on PIC2
        options(att_syntax, nomem, nostack)
    );
}

#[no_mangle]
pub extern "C" fn rust_main() -> ! {
    let vga = 0xb8000 as *mut u16;
    let msg = b"Long Mode OK!";

    unsafe {
        // Clear screen
        for i in 0..80*25 {
            *vga.add(i) = 0x0F00;
        }

        // Write message
        for (i, &byte) in msg.iter().enumerate() {
            *vga.add(i) = 0x0F00 | byte as u16;
        }

        loop {
            core::arch::asm!("hlt", options(nomem, nostack));
        }
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
