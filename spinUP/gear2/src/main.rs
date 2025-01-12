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
struct GDTTable {
    entries: [GDTEntry; 3]
}

#[repr(C, packed(4))]  // Ensure 4-byte alignment for 32-bit mode
struct GDTPointer {
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
    // Identity map first 2MB
    PAGE_TABLES.pml4.entries[0] = (&PAGE_TABLES.pdpt as *const _ as u64) | 0x3;
    PAGE_TABLES.pdpt.entries[0] = (&PAGE_TABLES.pd as *const _ as u64) | 0x3;
    PAGE_TABLES.pd.entries[0] = (&PAGE_TABLES.pt as *const _ as u64) | 0x3;

    // Map first 2MB with 4KB pages
    for i in 0..512 {
        PAGE_TABLES.pt.entries[i] = (i as u64 * 0x1000) | 0x3; // Present + writable
    }

    // Ensure CR3 points to PML4
    core::arch::asm!(
        ".code32",
        "mov eax, {pml4:e}",
        "mov cr3, eax",
        pml4 = in(reg) &PAGE_TABLES.pml4 as *const _ as u32,
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

// Static definitions
static mut GDT: GDTTable = GDTTable {
    entries: [
        // Null descriptor
        GDTEntry {
            limit_low: 0,
            base_low: 0,
            base_middle: 0,
            access: 0,
            granularity: 0,
            base_high: 0,
        },
        // 64-bit code segment
        GDTEntry {
            limit_low: 0xFFFF,
            base_low: 0,
            base_middle: 0,
            access: 0x9A,       // Present + Ring 0 + Code Segment + Readable
            granularity: 0xAF,  // 4K pages + Long mode + Limit bits
            base_high: 0,
        },
        // Data segment
        GDTEntry {
            limit_low: 0xFFFF,
            base_low: 0,
            base_middle: 0,
            access: 0x92,       // Present + Ring 0 + Data Segment + Writable
            granularity: 0xCF,  // 4K pages + 32-bit + Limit bits
            base_high: 0,
        },
    ]
};

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

unsafe fn setup_gdt() {
    let gdt_ptr = GDTPointer {
        limit: (core::mem::size_of::<GDTTable>() - 1) as u16,
        base: &raw const GDT as *const _ as u32,
    };

    core::arch::asm!(
        ".code32",
        // Use explicit operand size prefix and 32-bit register
        "lgdt [{0:e}]",  // :e specifies 32-bit register
        in(reg) &gdt_ptr
    );
}

unsafe fn setup_page_tables() {
    // Clear tables
    let pml4_ptr = &raw mut PAGE_TABLES.pml4.entries[0] as *mut u64;
    let pdpt_ptr = &raw mut PAGE_TABLES.pdpt.entries[0] as *mut u64;
    let pd_ptr = &raw mut PAGE_TABLES.pd.entries[0] as *mut u64;
    let pt_ptr = &raw mut PAGE_TABLES.pt.entries[0] as *mut u64;

    core::ptr::write_bytes(pml4_ptr, 0, 512);
    core::ptr::write_bytes(pdpt_ptr, 0, 512);
    core::ptr::write_bytes(pd_ptr, 0, 512);
    core::ptr::write_bytes(pt_ptr, 0, 512);

    // Set up identity mapping for first 2MB
    // Each entry contains the physical address of the next table and flags
    PAGE_TABLES.pml4.entries[0] = (&raw const PAGE_TABLES.pdpt as *const _ as u64) | 0x3;
    PAGE_TABLES.pdpt.entries[0] = (&raw const PAGE_TABLES.pd as *const _ as u64) | 0x3;
    PAGE_TABLES.pd.entries[0] = (&raw const PAGE_TABLES.pt as *const _ as u64) | 0x3;

    // Map first 2MB with 4KB pages
    for i in 0..512 {
        PAGE_TABLES.pt.entries[i] = (i as u64 * 0x1000) | 0x3; // Present + writable
    }
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
        core::arch::asm!(
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

            // Get faulting address from CR2
            "mov rdi, cr2",
            "mov rsi, rsp",    // Save original stack pointer
            "and rsp, ~0xF",   // Align stack to 16 bytes
            "call handle_page_fault",
            "mov rsp, rsi",    // Restore original stack pointer

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

            "add rsp, 8",     // Pop error code
            "iretq",
        );
    }
}

#[no_mangle]
extern "C" fn handle_page_fault(fault_addr: u64) {
    unsafe {
        let pt_idx = (fault_addr >> 12) & 0x1FF;
        PAGE_TABLES.pt.entries[pt_idx as usize] = (fault_addr & !0xFFF) | 0x3;

        // Invalidate TLB for this address
        core::arch::asm!("invlpg [{}]", in(reg) fault_addr);
    }
}

#[naked]
extern "x86-interrupt" fn timer_interrupt_handler() -> ! {
    unsafe {
        core::arch::asm!(
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
    core::arch::asm!("cli");

    // Clear all segment registers
    core::arch::asm!(
        ".code32",
        "xor ax, ax",
        "mov ds, ax",
        "mov es, ax",
        "mov fs, ax",
        "mov gs, ax",
        "mov ss, ax",
    );

    setup_page_tables();

    // Load CR3
    core::arch::asm!(
        ".code32",
        "mov eax, {pml4:e}",
        "mov cr3, eax",
        pml4 = in(reg) &raw const PAGE_TABLES.pml4 as *const _ as u32,
    );

    // Enable PAE
    core::arch::asm!(
        ".code32",
        "mov eax, cr4",
        "or eax, 1 << 5",  // PAE
        "mov cr4, eax"
    );

    // Enable Long Mode
    core::arch::asm!(
        ".code32",
        "mov ecx, 0xC0000080", // EFER MSR
        "rdmsr",
        "or eax, 1 << 8",      // LME
        "wrmsr"
    );

    setup_gdt();
    setup_idt();

    // Enable paging and protection
    core::arch::asm!(
        ".code32",
        "mov eax, cr0",
        "or eax, 1 << 31 | 1", // PG | PE
        "mov cr0, eax"
    );

    setup_pic();

    // Switch to long mode
    core::arch::asm!(
        ".code32",
        "push 0x08",         // Code segment
        "lea eax, [2f]",     // Target address
        "push eax",
        "retf",              // Far return to 64-bit mode

        "2:",
        ".code64",
        "mov ax, 0x10",      // Data segment
        "mov ds, ax",
        "mov es, ax",
        "mov fs, ax",
        "mov gs, ax",
        "mov ss, ax",

        "mov rsp, {stack}",
        "mov rbp, rsp",

        "sti",               // Enable interrupts

        "jmp {target}",

        stack = in(reg) &raw const STACK.data as *const _ as u64 + 4096,
                     target = sym rust_main,
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
