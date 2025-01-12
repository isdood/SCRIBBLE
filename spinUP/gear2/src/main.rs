// src/main.rs
// Last updated: 2025-01-12 04:40:48 UTC
// Author: isdood

#![no_std]
#![no_main]
#![feature(naked_functions)]
#![feature(abi_x86_interrupt)]

use core::panic::PanicInfo;

#[repr(C, packed)]
#[derive(Copy, Clone)]
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

#[repr(C, packed)]
struct GdtDescriptor {
    limit: u16,
    base: u32,
}

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

impl IdtEntry {
    const fn new_empty() -> Self {
        IdtEntry {
            offset_low: 0,
            segment: 0,
            ist: 0,
            flags: 0,
            offset_mid: 0,
            offset_high: 0,
            reserved: 0,
        }
    }
}

#[repr(C, packed)]
struct IdtDescriptor {
    limit: u16,
    base: u32,
}

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

#[repr(C, align(16))]
struct Stack {
    data: [u8; 4096 * 16],
}

static mut GDT: [GdtEntry; 5] = [
    GdtEntry::new_null(),
    GdtEntry {
        limit_low: 0,
        base_low: 0,
        base_middle: 0,
        access: 0x9A,
        granularity: 0x20,
        base_high: 0,
    },
GdtEntry {
    limit_low: 0,
    base_low: 0,
    base_middle: 0,
    access: 0x92,
    granularity: 0,
    base_high: 0,
},
GdtEntry {
    limit_low: 0,
    base_low: 0,
    base_middle: 0,
    access: 0xFA,
    granularity: 0x20,
    base_high: 0,
},
GdtEntry {
    limit_low: 0,
    base_low: 0,
    base_middle: 0,
    access: 0xF2,
    granularity: 0,
    base_high: 0,
},
];

static mut GDT_PTR: GdtDescriptor = GdtDescriptor {
    limit: (core::mem::size_of::<[GdtEntry; 5]>() - 1) as u16,
    base: 0,
};

static mut IDT: [IdtEntry; 256] = [IdtEntry::new_empty(); 256];

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

#[repr(C)]
struct InterruptStackFrame {
    ip: u64,
    cs: u64,
    flags: u64,
    sp: u64,
    ss: u64,
}

unsafe fn setup_gdt() {
    GDT_PTR.base = &raw const GDT as *const _ as u32;

    core::arch::asm!(
        ".code32",
        "lgdt [{0:e}]",
        in(reg) &raw const GDT_PTR as *const _ as u32,
                     options(readonly),
    );

    core::arch::asm!(
        ".code32",
        "mov ax, 0x10",
        "mov ds, ax",
        "mov es, ax",
        "mov fs, ax",
        "mov gs, ax",
        "mov ss, ax",
    );
}

unsafe fn setup_idt() {
    let handler = timer_interrupt_handler as u64;
    IDT[0x20].offset_low = handler as u16;
    IDT[0x20].segment = 0x08;
    IDT[0x20].ist = 0;
    IDT[0x20].flags = 0x8E;
    IDT[0x20].offset_mid = (handler >> 16) as u16;
    IDT[0x20].offset_high = (handler >> 32) as u32;

    IDT_PTR.base = &raw const IDT as *const _ as u32;

    core::arch::asm!(
        ".code32",
        "lidt [{0:e}]",
        in(reg) &raw const IDT_PTR as *const _ as u32,
                     options(readonly),
    );
}

unsafe fn setup_page_tables() {
    core::ptr::write_bytes(&raw mut PAGE_TABLES as *mut _, 0, 1);

    PAGE_TABLES.pml4.entries[0] = (&raw const PAGE_TABLES.pdpt as *const _ as u64) | 0x3;
    PAGE_TABLES.pdpt.entries[0] = (&raw const PAGE_TABLES.pd as *const _ as u64) | 0x3;

    for i in 0..512 {
        PAGE_TABLES.pd.entries[i] = (i as u64 * 0x200000) | 0x83;
    }

    core::arch::asm!(
        ".code32",
        "mov cr3, {0:e}",
        in(reg) &raw const PAGE_TABLES.pml4 as *const _ as u32,
    );
}

unsafe fn setup_pic() {
    core::arch::asm!(
        ".code32",
        "mov al, 0x11",
        "out 0x20, al",
        "out 0xA0, al",

        "mov al, 0x20",
        "out 0x21, al",
        "mov al, 0x28",
        "out 0xA1, al",

        "mov al, 0x04",
        "out 0x21, al",
        "mov al, 0x02",
        "out 0xA1, al",

        "mov al, 0x01",
        "out 0x21, al",
        "out 0xA1, al",

        "mov al, 0x00",
        "out 0x21, al",
        "out 0xA1, al",
    );
}

#[naked]
unsafe extern "x86-interrupt" fn timer_interrupt_handler(_stack_frame: InterruptStackFrame) {
    core::arch::naked_asm!(
        ".code32",
        "push eax",
        "mov al, 0x20",
        "out 0x20, al",
        "pop eax",
        "iret"
    );
}

#[no_mangle]
pub unsafe extern "C" fn _start() -> ! {
    core::arch::asm!(".code32", "cli");

    setup_gdt();
    setup_page_tables();

    core::arch::asm!(
        ".code32",
        "mov eax, cr4",
        "or eax, 0x20",
        "mov cr4, eax",
    );

    core::arch::asm!(
        ".code32",
        "mov ecx, 0xC0000080",
        "rdmsr",
        "or eax, 0x100",
        "wrmsr",
    );

    core::arch::asm!(
        ".code32",
        "mov eax, cr0",
        "or eax, 0x80000001",
        "mov cr0, eax",
    );

    core::arch::asm!(
        ".code32",
        "push {0}",
        "push {1}",
        "retf",
        ".code64",
        "mov ax, 0x10",
        "mov ds, ax",
        "mov es, ax",
        "mov fs, ax",
        "mov gs, ax",
        "mov ss, ax",
        "mov rsp, {2}",
        "mov rbp, rsp",
        "call {3}",
        "call {4}",
        "sti",
        "jmp {5}",
        const 0x08,
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
