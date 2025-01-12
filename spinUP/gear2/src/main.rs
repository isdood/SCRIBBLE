#![no_std]
#![no_main]
#![feature(naked_functions)]
#![feature(abi_x86_interrupt)]

use core::arch::asm;
use core::panic::PanicInfo;

// GDT Entry structure
#[repr(C, packed)]
struct GdtEntry {
    limit_low: u16,
    base_low: u16,
    base_middle: u8,
    access: u8,
    granularity: u8,
    base_high: u8,
}

impl GdtEntry {
    const fn new_null() -> Self {
        GdtEntry {
            limit_low: 0,
            base_low: 0,
            base_middle: 0,
            access: 0,
            granularity: 0,
            base_high: 0,
        }
    }
}

// GDT Descriptor structure
#[repr(C, packed)]
struct GdtDescriptor {
    limit: u16,
    base: u64,
}

// IDT Entry structure
#[repr(C, packed)]
struct IdtEntry {
    offset_low: u16,
    segment: u16,
    ist: u8,
    flags: u8,
    offset_mid: u16,
    offset_high: u32,
    reserved: u32,
}

// IDT Descriptor structure
#[repr(C, packed)]
struct IdtDescriptor {
    limit: u16,
    base: u64,
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

// Stack
#[repr(C, align(16))]
struct Stack {
    data: [u8; 4096 * 16], // 64KB stack
}

// Static variables
static mut GDT: [GdtEntry; 5] = [
    GdtEntry::new_null(),
    // Kernel code (64-bit)
    GdtEntry {
        limit_low: 0,
        base_low: 0,
        base_middle: 0,
        access: 0x9A,       // Present, Ring 0, Code, Executable, Readable
        granularity: 0x20,  // Long mode
        base_high: 0,
    },
// Kernel data
GdtEntry {
    limit_low: 0,
    base_low: 0,
    base_middle: 0,
    access: 0x92,      // Present, Ring 0, Data, Writable
    granularity: 0,    // Byte granularity
    base_high: 0,
},
// User code (64-bit)
GdtEntry {
    limit_low: 0,
    base_low: 0,
    base_middle: 0,
    access: 0xFA,      // Present, Ring 3, Code, Executable, Readable
    granularity: 0x20, // Long mode
    base_high: 0,
},
// User data
GdtEntry {
    limit_low: 0,
    base_low: 0,
    base_middle: 0,
    access: 0xF2,      // Present, Ring 3, Data, Writable
    granularity: 0,    // Byte granularity
    base_high: 0,
},
];

static mut GDT_PTR: GdtDescriptor = GdtDescriptor {
    limit: (core::mem::size_of::<[GdtEntry; 5]>() - 1) as u16,
    base: 0,
};

static mut IDT: [IdtEntry; 256] = [IdtEntry {
    offset_low: 0,
    segment: 0,
    ist: 0,
    flags: 0,
    offset_mid: 0,
    offset_high: 0,
    reserved: 0,
}; 256];

static mut IDT_PTR: IdtDescriptor = IdtDescriptor {
    limit: (core::mem::size_of::<[IdtEntry; 256]>() - 1) as u16,
    base: 0,
};

static mut PAGE_TABLES: PageTables = PageTables {
    pml4: PageTable { entries: [0; 512] },
    pdpt: PageTable { entries: [0; 512] },
    pd: PageTable { entries: [0; 512] },
};

static mut STACK: Stack = Stack {
    data: [0; 4096 * 16],
};

const STACK_SIZE: usize = 4096 * 16;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

unsafe fn setup_gdt() {
    // Initialize GDT pointer
    GDT_PTR.base = &raw const GDT as *const _ as u64;

    // Load GDT
    core::arch::asm!(
        ".code32",
        "lgdt [{0}]",
        in(reg) &raw const GDT_PTR,
    );

    // Load segment registers
    core::arch::asm!(
        ".code32",
        "mov ax, 0x10",  // Data segment
        "mov ds, ax",
        "mov es, ax",
        "mov fs, ax",
        "mov gs, ax",
        "mov ss, ax",
    );
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
    IDT_PTR.base = &raw const IDT as *const _ as u64;

    // Load IDT
    core::arch::asm!("lidt [{0}]", in(reg) &raw const IDT_PTR);
}

unsafe fn setup_page_tables() {
    // Clear tables first
    core::ptr::write_bytes(&raw mut PAGE_TABLES as *mut _, 0, 1);

    // Set up identity mapping for first 1GB
    // PML4 -> PDPT
    PAGE_TABLES.pml4.entries[0] = (&raw const PAGE_TABLES.pdpt as *const _ as u64) | 0x3;

    // PDPT -> PD
    PAGE_TABLES.pdpt.entries[0] = (&raw const PAGE_TABLES.pd as *const _ as u64) | 0x3;

    // PD -> 2MB pages
    for i in 0..512 {
        PAGE_TABLES.pd.entries[i] = (i as u64 * 0x200000) | 0x83; // Present + Write + Huge
    }

    // Load CR3 with PML4 address
    core::arch::asm!(
        "mov cr3, {0}",
        in(reg) &raw const PAGE_TABLES.pml4 as *const _ as u64
    );
}

unsafe fn setup_pic() {
    // Initialize the PIC
    core::arch::asm!(
        // Start initialization
        "mov al, 0x11",
        "out 0x20, al",  // Master PIC command
        "out 0xA0, al",  // Slave PIC command

        // Set vector offsets
        "mov al, 0x20",
        "out 0x21, al",  // Master PIC vector offset (IRQ 0-7: 0x20-0x27)
    "mov al, 0x28",
    "out 0xA1, al",  // Slave PIC vector offset (IRQ 8-15: 0x28-0x2F)

    // Set up master/slave relationship
    "mov al, 0x04",
    "out 0x21, al",  // Tell master there is a slave at IRQ2
    "mov al, 0x02",
    "out 0xA1, al",  // Tell slave its cascade identity

    // Set 8086 mode
    "mov al, 0x01",
    "out 0x21, al",
    "out 0xA1, al",

    // Clear masks
    "mov al, 0x00",
    "out 0x21, al",  // Enable all IRQs on master
    "out 0xA1, al",  // Enable all IRQs on slave
    );
}

#[naked]
unsafe extern "x86-interrupt" fn timer_interrupt_handler(_stack_frame: InterruptStackFrame) {
    core::arch::naked_asm!(
        "push rax",
        "mov al, 0x20",
        "out 0x20, al",  // Send EOI to PIC
        "pop rax",
        "iretq"
    );
}

#[no_mangle]
pub unsafe extern "C" fn _start() -> ! {
    // Disable interrupts until we're ready
    core::arch::asm!(".code32", "cli");

    // Set up GDT first
    setup_gdt();

    // Set up page tables
    setup_page_tables();

    // Enable PAE
    core::arch::asm!(
        ".code32",
        "mov eax, cr4",
        "or eax, 0x20",     // Set PAE bit
        "mov cr4, eax",
    );

    // Enable long mode
    core::arch::asm!(
        ".code32",
        "mov ecx, 0xC0000080", // EFER MSR
        "rdmsr",
        "or eax, 0x100",       // Set LME bit
        "wrmsr",
    );

    // Enable paging and protected mode
    core::arch::asm!(
        ".code32",
        "mov eax, cr0",
        "or eax, 0x80000001",  // Set PG and PE bits
        "mov cr0, eax",
    );

    // Far jump to 64-bit mode
    core::arch::asm!(
        ".code32",
        // Setup the far jump using push/retf
        "push {0}",     // Push segment selector
        "push {1}",     // Push offset
        "retf",         // Far return acts as far jump
        "2:",          // Target label
        ".code64",
        // Now in 64-bit mode
        "mov ax, 0x10", // Data segment
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

        // Set up PIC
        "call {4}",

        // Enable interrupts
        "sti",

        // Jump to Rust main
        "jmp {5}",
        const 0x08,    // Code segment selector
        sym long_mode_start,
        in(reg) (&raw const STACK.data as *const _ as u64) + (STACK_SIZE as u64),
                     sym setup_idt,
                     sym setup_pic,
                     sym rust_main,
                     options(noreturn),
    );
}

#[no_mangle]
unsafe extern "C" fn long_mode_start() -> ! {
    rust_main()
}

#[no_mangle]
pub extern "C" fn rust_main() -> ! {
    loop {
        unsafe {
            core::arch::asm!("hlt");
        }
    }
}
