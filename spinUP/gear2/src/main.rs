#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]
#![feature(naked_functions)]

use core::panic::PanicInfo;
use unstable_matter::UnstableMatter;

// Struct definitions
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

static mut IDT: [IdtEntry; 256] = [IdtEntry {
    offset_low: 0,
    segment: 0,
    ist: 0,
    flags: 0,
    offset_mid: 0,
    offset_high: 0,
    reserved: 0,
}; 256];

#[repr(C, packed)]
struct IdtPointer {
    limit: u16,
    base: u64,
}

static mut IDT_PTR: IdtPointer = IdtPointer {
    limit: (core::mem::size_of::<[IdtEntry; 256]>() - 1) as u16,
    base: 0,
};

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

unsafe fn setup_idt() {
    // Set up timer interrupt handler
    let handler = timer_interrupt_handler as u64;
    IDT[0x20].offset_low = handler as u16;
    IDT[0x20].segment = 0x08; // Code segment
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
    // Zero out all tables first
    core::ptr::write_bytes(&raw mut PAGE_TABLES as *mut _, 0, 1);

    // Set up identity mapping for first 1GB using 2MB pages
    PAGE_TABLES.pml4.entries[0] = (&raw const PAGE_TABLES.pdpt as *const _ as u64) | 0x3;
    PAGE_TABLES.pdpt.entries[0] = (&raw const PAGE_TABLES.pd as *const _ as u64) | 0x3;

    // Map first 1GB with 2MB pages
    for i in 0..512 {
        PAGE_TABLES.pd.entries[i] = (i as u64 * 0x200000) | 0x83; // Present + writable + huge
    }
}

#[repr(C, align(4096))]
struct Stack {
    data: [u8; 4096]
}

#[repr(C, packed)]
pub struct StageInfo {
    boot_drive: u8,
    memory_map_addr: u32,
    memory_entries: u16,
    stage2_load_addr: u32,
    flags: u32,
}

static mut STACK: Stack = Stack {
    data: [0; 4096]
};

// Function implementations
unsafe fn setup_idt() {
    // Set up timer interrupt handler
    IDT.entries[32] = IdtEntry {
        offset_low: (timer_interrupt_handler as u64 & 0xFFFF) as u16,
        segment_selector: 0x08,  // Kernel code segment
        ist: 0,                  // No interrupt stack table
        flags: 0x8E,            // Present, Ring 0, Interrupt Gate
        offset_middle: ((timer_interrupt_handler as u64 >> 16) & 0xFFFF) as u16,
        offset_high: (timer_interrupt_handler as u64 >> 32) as u32,
        reserved: 0,
    };

    // Set up page fault handler
    IDT.entries[14] = IdtEntry {
        offset_low: (page_fault_handler as u64 & 0xFFFF) as u16,
        segment_selector: 0x08,  // Kernel code segment
        ist: 0,                  // No interrupt stack table
        flags: 0x8E,            // Present, Ring 0, Interrupt Gate
        offset_middle: ((page_fault_handler as u64 >> 16) & 0xFFFF) as u16,
        offset_high: (page_fault_handler as u64 >> 32) as u32,
        reserved: 0,
    };

    let idt_ptr = IdtPointer {
        limit: (core::mem::size_of::<Idt>() - 1) as u16,
        base: &raw const IDT as *const _ as u32,
    };

    core::arch::asm!(
        ".code32",
        "lidt [{0:e}]",
        in(reg) &idt_ptr
    );
}

#[repr(C, packed)]
struct GdtDescriptor {
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

static mut GDT: [GdtDescriptor; 5] = [
    // Null descriptor
    GdtDescriptor {
        limit_low: 0,
        base_low: 0,
        base_middle: 0,
        access: 0,
        granularity: 0,
        base_high: 0,
    },
// Kernel code segment (64-bit)
GdtDescriptor {
    limit_low: 0xFFFF,
    base_low: 0,
    base_middle: 0,
    access: 0x9A,      // Present, Ring 0, Code, Readable
    granularity: 0x2F, // 4KB granularity, 64-bit code
    base_high: 0,
},
// Kernel data segment
GdtDescriptor {
    limit_low: 0xFFFF,
    base_low: 0,
    base_middle: 0,
    access: 0x92,      // Present, Ring 0, Data, Writable
    granularity: 0xCF, // 4KB granularity, 32-bit
    base_high: 0,
},
// User code segment (64-bit)
GdtDescriptor {
    limit_low: 0xFFFF,
    base_low: 0,
    base_middle: 0,
    access: 0xFA,      // Present, Ring 3, Code, Readable
    granularity: 0x2F, // 4KB granularity, 64-bit code
    base_high: 0,
},
// User data segment
GdtDescriptor {
    limit_low: 0xFFFF,
    base_low: 0,
    base_middle: 0,
    access: 0xF2,      // Present, Ring 3, Data, Writable
    granularity: 0xCF, // 4KB granularity, 32-bit
    base_high: 0,
},
];

static mut GDT_PTR: GdtPointer = GdtPointer {
    limit: (core::mem::size_of::<[GdtDescriptor; 5]>() - 1) as u16,
    base: 0, // Will be initialized at runtime
};

unsafe fn setup_gdt() {
    // Set the GDT base address
    GDT_PTR.base = &raw const GDT as *const _ as u64;
}

unsafe fn setup_pic() {
    // ICW1: start initialization
    core::arch::asm!(
        "mov al, 0x11",
        "out 0x20, al",
        "out 0xA0, al",
        "out 0x80, al"
    );

    // ICW2: vector offset
    core::arch::asm!(
        "mov al, 32",
        "out 0x21, al",
        "mov al, 40",
        "out 0xA1, al",
        "out 0x80, al"
    );

    // ICW3: cascade configuration
    core::arch::asm!(
        "mov al, 4",
        "out 0x21, al",
        "mov al, 2",
        "out 0xA1, al",
        "out 0x80, al"
    );

    // ICW4: x86 mode
    core::arch::asm!(
        "mov al, 1",
        "out 0x21, al",
        "out 0xA1, al",
        "out 0x80, al"
    );

    // OCW1: mask interrupts
    core::arch::asm!(
        "mov al, 0xfe",
        "out 0x21, al",
        "mov al, 0xff",
        "out 0xA1, al",
        "out 0x80, al"
    );
}

#[naked]
extern "x86-interrupt" fn page_fault_handler() -> ! {
    unsafe {
        core::arch::naked_asm!(
            "cli",              // Disable interrupts
            "push rax",
            "push rcx",
            "push rdx",
            "push rbx",
            "push rbp",
            "push rsi",
            "push rdi",
            "push r8",
            "push r9",
            "push r10",
            "push r11",
            "push r12",
            "push r13",
            "push r14",
            "push r15",

            "mov rdi, cr2",     // Get fault address
            "mov rsi, [rsp+120]", // Get error code
            "call {handle_fault}",

            "pop r15",
            "pop r14",
            "pop r13",
            "pop r12",
            "pop r11",
            "pop r10",
            "pop r9",
            "pop r8",
            "pop rdi",
            "pop rsi",
            "pop rbp",
            "pop rbx",
            "pop rdx",
            "pop rcx",
            "pop rax",

            "add rsp, 8",      // Remove error code
            "sti",             // Re-enable interrupts
            "iretq",
            handle_fault = sym handle_page_fault,
        );
    }
}

#[no_mangle]
extern "C" fn handle_page_fault(fault_addr: u64, error_code: u64) {
    unsafe {
        // Only handle page-not-present faults
        if error_code & 1 == 0 {
            let pd_idx = (fault_addr >> 21) & 0x1FF;
            PAGE_TABLES.pd.entries[pd_idx as usize] = (fault_addr & !0x1FFFFF) | 0x83;
            core::arch::asm!("invlpg [{}]", in(reg) fault_addr);
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn _start() -> ! {
    // Disable interrupts until we're ready
    core::arch::asm!(".code32", "cli");

    // Set up GDT first
    setup_gdt();

    // Load GDT
    core::arch::asm!(
        ".code32",
        "lgdt [{0:e}]",
        in(reg) &raw const GDT_PTR,
    );

    // Set up page tables
    setup_page_tables();

    // Load CR3
    core::arch::asm!(
        ".code32",
        "mov eax, {0:e}",
        "mov cr3, eax",
        in(reg) &raw const PAGE_TABLES.pml4 as *const _ as u32,
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

    // Jump to 64-bit mode
    core::arch::asm!(
        ".code32",
        "jmp 0x08:2f",    // Far jump with new code segment
        "2:",
        ".code64",
        "mov ax, 0x10",   // Data segment
        "mov ds, ax",
        "mov es, ax",
        "mov fs, ax",
        "mov gs, ax",
        "mov ss, ax",

        // Set up stack
        "mov rsp, {0}",
        "mov rbp, rsp",

        // Set up IDT
        "call {1}",

        // Remap and initialize PIC
        "call {2}",

        // Enable interrupts
        "sti",

        // Jump to Rust main
        "jmp {3}",
        in(reg) &raw const STACK.data as *const _ as u64 + 4096,
                     sym setup_idt,
                     sym setup_pic,
                     sym rust_main,
                     options(noreturn),
    );
}

#[naked]
extern "x86-interrupt" fn timer_interrupt_handler() -> ! {
    unsafe {
        core::arch::naked_asm!(
            ".code64",
            "push rax",
            "mov al, 0x20",
            "out 0x20, al",
            "pop rax",
            "iretq",
        );
    }
}

#[no_mangle]
pub extern "C" fn rust_main() -> ! {
    unsafe {
        let mut vga = UnstableMatter::<u16>::at(0xB8000);
        let msg = b"Long Mode OK!";

        for _ in 0..80*25 {
            vga.write(0x0F00);
        }

        for (_, &byte) in msg.iter().enumerate() {
            vga.write(0x0F00 | byte as u16);
        }

        loop {
            core::arch::asm!("hlt", options(nomem, nostack));
        }
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {
        unsafe {
            core::arch::asm!("hlt", options(nomem, nostack));
        }
    }
}
