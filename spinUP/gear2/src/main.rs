#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]
#![feature(naked_functions)]

use core::panic::PanicInfo;
use unstable_matter::UnstableMatter;

// Struct definitions
#[derive(Clone, Copy)]
#[repr(C)]
struct IdtEntry {
    offset_low: u16,
    segment_selector: u16,
    ist: u8,
    flags: u8,
    offset_middle: u16,
    offset_high: u32,
    reserved: u32,
}

#[repr(align(16))]
struct Idt {
    entries: [IdtEntry; 256]
}

#[repr(C, packed(4))]  // Ensure 4-byte alignment for 32-bit mode
struct IdtPointer {
    limit: u16,
    base: u32,
}

#[repr(C)]
struct PageTable {
    entries: [u64; 512]
}

#[repr(C, align(4096))]
struct PageTables {
    pml4: PageTable,
    pdpt: PageTable,
    pd: PageTable,
    pt: PageTable,
}

static mut PAGE_TABLES: PageTables = PageTables {
    pml4: PageTable { entries: [0; 512] },
    pdpt: PageTable { entries: [0; 512] },
    pd: PageTable { entries: [0; 512] },
    pt: PageTable { entries: [0; 512] },
};

unsafe fn setup_page_tables() {
    // First clear all tables
    core::ptr::write_bytes(&raw mut PAGE_TABLES as *mut _, 0, 1);

    // Identity map first 2MB with a single 2MB page
    PAGE_TABLES.pml4.entries[0] = (&raw const PAGE_TABLES.pdpt as *const _ as u64) | 0x3;
    PAGE_TABLES.pdpt.entries[0] = (&raw const PAGE_TABLES.pd as *const _ as u64) | 0x3;
    PAGE_TABLES.pd.entries[0] = 0x83; // Present + writable + huge page (2MB)

    // Load CR3 with PML4 address
    core::arch::asm!(
        ".code32",
        "mov eax, {pml4:e}",
        "mov cr3, eax",
        pml4 = in(reg) &raw const PAGE_TABLES.pml4 as *const _ as u32,
    );
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



static mut IDT: Idt = Idt {
    entries: {
        const EMPTY_ENTRY: IdtEntry = IdtEntry {
            offset_low: 0,
            segment_selector: 0x08,
            ist: 0,
            flags: 0x8E,
            offset_middle: 0,
            offset_high: 0,
            reserved: 0,
        };
        [EMPTY_ENTRY; 256]
    }
};

static mut STACK: Stack = Stack {
    data: [0; 4096]
};

#[allow(dead_code)]
static mut STAGE_INFO: StageInfo = StageInfo {
    boot_drive: 0,
    memory_map_addr: 0,
    memory_entries: 0,
    stage2_load_addr: 0x7E00,
    flags: 0,
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

#[naked]
extern "x86-interrupt" fn timer_interrupt_handler() -> ! {
    unsafe {
        core::arch::naked_asm!(
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

            "mov al, 0x20",  // EOI
            "out 0x20, al",

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

            "iretq",
        );
    }
}

#[no_mangle]
pub unsafe extern "C" fn _start() -> ! {
    // Disable interrupts
    core::arch::asm!("cli");

    // Clear segment registers
    core::arch::asm!(
        ".code32",
        "xor ax, ax",
        "mov ds, ax",
        "mov es, ax",
        "mov fs, ax",
        "mov gs, ax",
        "mov ss, ax",
    );

    // Set up paging
    setup_page_tables();

    // Enable PAE
    core::arch::asm!(
        ".code32",
        "mov eax, cr4",
        "or eax, 1 << 5",  // PAE
        "mov cr4, eax"
    );

    // Enable long mode in EFER
    core::arch::asm!(
        ".code32",
        "mov ecx, 0xC0000080", // EFER MSR
        "rdmsr",
        "or eax, 1 << 8",      // LME
        "wrmsr"
    );

    // Set up GDT for long mode
    setup_gdt();

    // Enable paging and protected mode
    core::arch::asm!(
        ".code32",
        "mov eax, cr0",
        "or eax, 1 << 31 | 1", // PG | PE
        "mov cr0, eax"
    );

    // Set up IDT after paging is enabled
    setup_idt();

    // Set up PIC
    setup_pic();

    // Jump to long mode
    core::arch::asm!(
        ".code32",
        "lgdt [{gdt}]",        // Load GDT
        "push 0x08",           // Code segment
        "lea eax, [lm_entry]", // Target address
        "push eax",
        "retf",                // Far return to 64-bit mode
        "lm_entry:",           // Changed from "1:" to "lm_entry:"
        ".code64",
        // Set up segment registers
        "mov ax, 0x10",        // Data segment
        "mov ds, ax",
        "mov es, ax",
        "mov fs, ax",
        "mov gs, ax",
        "mov ss, ax",
        // Set up stack
        "mov rsp, {stack}",
        "mov rbp, rsp",
        // Enable interrupts
        "sti",
        // Jump to Rust main
        "jmp {main}",
        gdt = in(reg) &raw const GDT_PTR,
                     stack = in(reg) &raw const STACK.data as *const _ as u64 + 4096,
                     main = sym rust_main,
                     options(noreturn)
    );
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
