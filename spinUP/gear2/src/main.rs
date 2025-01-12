#![no_std]
#![no_main]
#![feature(naked_functions)]
#![feature(abi_x86_interrupt)]

use core::panic::PanicInfo;
use x86_64::structures::idt::InterruptStackFrame;

// Constants
const STACK_SIZE: usize = 4096;

// Stack
#[repr(align(16))]
struct Stack {
    data: [u8; STACK_SIZE]
}

static mut STACK: Stack = Stack {
    data: [0; STACK_SIZE]
};

// Page Tables
#[repr(C, align(4096))]
struct PageTable {
    entries: [u64; 512]
}

#[repr(C, align(4096))]
struct PageTables {
    pml4: PageTable,
    pdpt: PageTable,
    pd: PageTable,
}

static mut PAGE_TABLES: PageTables = PageTables {
    pml4: PageTable { entries: [0; 512] },
    pdpt: PageTable { entries: [0; 512] },
    pd: PageTable { entries: [0; 512] },
};

// GDT structures
#[repr(C, packed)]
struct GdtEntry {
    limit_low: u16,
    base_low: u16,
    base_middle: u8,
    access: u8,
    granularity: u8,
    base_high: u8,
}

#[repr(C, packed)]
struct GdtPointer {
    limit: u16,
    base: u64,
}

static mut GDT: [GdtEntry; 5] = [
    // Null descriptor
    GdtEntry {
        limit_low: 0,
        base_low: 0,
        base_middle: 0,
        access: 0,
        granularity: 0,
        base_high: 0,
    },
// Kernel code (64-bit)
GdtEntry {
    limit_low: 0xFFFF,
    base_low: 0,
    base_middle: 0,
    access: 0x9A,       // Present, Ring 0, Code, Readable
    granularity: 0x20,  // Long mode
    base_high: 0,
},
// Kernel data
GdtEntry {
    limit_low: 0xFFFF,
    base_low: 0,
    base_middle: 0,
    access: 0x92,      // Present, Ring 0, Data, Writable
    granularity: 0,
    base_high: 0,
},
// User code (64-bit)
GdtEntry {
    limit_low: 0xFFFF,
    base_low: 0,
    base_middle: 0,
    access: 0xFA,      // Present, Ring 3, Code, Readable
    granularity: 0x20, // Long mode
    base_high: 0,
},
// User data
GdtEntry {
    limit_low: 0xFFFF,
    base_low: 0,
    base_middle: 0,
    access: 0xF2,      // Present, Ring 3, Data, Writable
    granularity: 0,
    base_high: 0,
},
];

static mut GDT_PTR: GdtPointer = GdtPointer {
    limit: (core::mem::size_of::<[GdtEntry; 5]>() - 1) as u16,
    base: 0,
};

#[repr(C, packed)]
#[derive(Copy, Clone)]
struct IdtEntry {
    offset_low: u16,
    segment: u16,
    ist: u8,
    flags: u8,
    offset_mid: u16,
    offset_high: u32,
    reserved: u32,
}

#[repr(C, packed)]
struct IdtDescriptor {
    limit: u16,
    base: u64,
}

// Initialize the IDT with a const array
static mut IDT: [IdtEntry; 256] = {
    const EMPTY: IdtEntry = IdtEntry {
        offset_low: 0,
        segment: 0,
        ist: 0,
        flags: 0,
        offset_mid: 0,
        offset_high: 0,
        reserved: 0,
    };
    [EMPTY; 256]
};

static mut IDT_PTR: IdtDescriptor = IdtDescriptor {
    limit: (core::mem::size_of::<[IdtEntry; 256]>() - 1) as u16,
    base: 0,
};


// Setup functions
unsafe fn setup_gdt() {
    GDT_PTR.base = &GDT as *const _ as u64;
}

unsafe fn setup_idt() {
    // Set up timer interrupt handler
    let handler = timer_interrupt_handler as u64;
    IDT[0x20].offset_low = handler as u16;
    IDT[0x20].segment = 0x08; // Kernel code segment
    IDT[0x20].ist = 0;
    IDT[0x20].flags = 0x8E;   // Present, Ring 0, Interrupt Gate
    IDT[0x20].offset_mid = (handler >> 16) as u16;
    IDT[0x20].offset_high = (handler >> 32) as u32;

    // Set IDT pointer
    IDT_PTR.base = &IDT as *const _ as u64;

    // Load IDT
    core::arch::asm!("lidt [{0}]", in(reg) &IDT_PTR);
}


unsafe fn setup_page_tables() {

    core::ptr::write_bytes(&mut PAGE_TABLES as *mut _, 0, 1);

    // Similar fixes needed for other raw pointer casts
    PAGE_TABLES.pml4.entries[0] = (&PAGE_TABLES.pdpt as *const _ as u64) | 0x3;
    PAGE_TABLES.pdpt.entries[0] = (&PAGE_TABLES.pd as *const _ as u64) | 0x3;
}

unsafe fn setup_pic() {
    // Remap PIC
    // Start initialization
    core::arch::asm!(
        "mov al, 0x11",
        "out 0x20, al", // Master PIC command
        "out 0xA0, al", // Slave PIC command
        "mov al, 0x20",
        "out 0x21, al", // Master PIC vector offset
        "mov al, 0x28",
        "out 0xA1, al", // Slave PIC vector offset
        "mov al, 0x04",
        "out 0x21, al", // Tell Master PIC about Slave
        "mov al, 0x02",
        "out 0xA1, al", // Tell Slave its cascade identity
        "mov al, 0x01",
        "out 0x21, al", // 8086 mode for Master
        "out 0xA1, al", // 8086 mode for Slave
        // Mask all interrupts except timer (IRQ0)
        "mov al, 0xFE",
        "out 0x21, al",
        "mov al, 0xFF",
        "out 0xA1, al",
    );
}

// Entry point
#[no_mangle]
pub unsafe extern "C" fn _start() -> ! {
    // Disable interrupts until we're ready
    core::arch::asm!(".code32", "cli");

    // Set up GDT first
    setup_gdt();

    // Load GDT with correct 32-bit registers
    core::arch::asm!(
        ".code32",
        "lgdt [{0:e}]",  // Use 32-bit addressing
        in(reg) &GDT_PTR,
    );

    // Set up page tables
    setup_page_tables();

    // Load CR3
    core::arch::asm!(
        ".code32",
        "mov eax, {0:e}",
        "mov cr3, eax",
        in(reg) &PAGE_TABLES.pml4 as *const _ as u32,
    );

    // Enable PAE and PSE
    core::arch::asm!(
        ".code32",
        "mov eax, cr4",
        "or eax, 0x30",  // PAE | PSE
        "mov cr4, eax",
    );

    // Enable long mode
    core::arch::asm!(
        ".code32",
        "mov ecx, 0xC0000080", // EFER MSR
        "rdmsr",
        "or eax, 0x100",       // LME
        "wrmsr",
    );

    // Enable paging
    core::arch::asm!(
        ".code32",
        "mov eax, cr0",
        "or eax, 0x80000001",  // PG | PE
        "mov cr0, eax",
    );

    // Far jump to 64-bit mode with fixed argument handling
    core::arch::asm!(
        ".code32",
        // Move values to registers first
        "mov eax, {0}",
        "push eax",        // Push segment selector
        "mov eax, offset {1}",
        "push eax",        // Push offset
        "retf",           // Far return will act as our far jump
        ".code64",
        "2:",            // Target label
        // Now in 64-bit mode
        "mov ax, 0x10",   // Data segment
        "mov ds, ax",
        "mov es, ax",
        "mov fs, ax",
        "mov gs, ax",
        "mov ss, ax",

        // Set up stack
        "mov rsp, {2}",
        "mov rbp, rsp",

        // Set up IDT
        "call {3}",

        // Remap and initialize PIC
        "call {4}",

        // Enable interrupts
        "sti",

        // Jump to Rust main
        "jmp {5}",
        const 0x08,        // Code segment selector
        sym long_mode_start,
        in(reg) (&STACK.data as *const _ as u64) + (STACK_SIZE as u64),
                     sym setup_idt,
                     sym setup_pic,
                     sym rust_main,
                     options(noreturn),
    );
}

#[no_mangle]
unsafe extern "C" fn long_mode_start() -> ! {
    // This function will never be called directly - it's just a target for the jump
    rust_main()
}

#[naked]
unsafe extern "x86-interrupt" fn timer_interrupt_handler(_stack_frame: InterruptStackFrame) {
    core::arch::naked_asm!(
        "push rax",
        "mov al, 0x20",
        "out 0x20, al",  // Send EOI to PIC
        "pop rax",
        "iretq",
    );
}

#[no_mangle]
pub extern "C" fn rust_main() -> ! {
    loop {
        unsafe {
            core::arch::asm!("hlt");
        }
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {
        unsafe {
            core::arch::asm!("hlt");
        }
    }
}
