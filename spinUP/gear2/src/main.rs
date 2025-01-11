#![no_std]
#![no_main]

use core::panic::PanicInfo;
mod serial;
use serial::SerialPort;
use unstable_matter::UnstableMatter;

#[repr(C)]
struct PageTable {
    entries: [u64; 512]
}

#[repr(C, align(4096))]
struct PageTables {
    pml4: PageTable,
    pdpt: PageTable,
    pd: PageTable,
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

#[repr(C, packed)]
struct GDTPointer {
    limit: u16,
    base: u32,
}

#[repr(C, align(4096))]
struct Stack {
    data: [u8; 4096]
}

static mut PAGE_TABLES: PageTables = PageTables {
    pml4: PageTable { entries: [0; 512] },
    pdpt: PageTable { entries: [0; 512] },
    pd: PageTable { entries: [0; 512] },
};

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
        // Code segment
        GDTEntry {
            limit_low: 0xFFFF,
            base_low: 0,
            base_middle: 0,
            access: 0x9A,
            granularity: 0xAF,
            base_high: 0,
        },
        // Data segment
        GDTEntry {
            limit_low: 0xFFFF,
            base_low: 0,
            base_middle: 0,
            access: 0x92,
            granularity: 0xCF,
            base_high: 0,
        },
    ]
};

static mut STACK: Stack = Stack {
    data: [0; 4096]
};

static mut SERIAL: SerialPort = unsafe { SerialPort::new(0x3F8) };

unsafe fn disable_interrupts() {
    core::arch::asm!("cli");
}

unsafe fn setup_page_tables() {
    let mut pml4_entries = UnstableMatter::at(&mut PAGE_TABLES.pml4.entries[0] as *mut _ as usize);
    let mut pdpt_entries = UnstableMatter::at(&mut PAGE_TABLES.pdpt.entries[0] as *mut _ as usize);
    let mut pd_entries = UnstableMatter::at(&mut PAGE_TABLES.pd.entries[0] as *mut _ as usize);

    let pdpt_addr = &raw const PAGE_TABLES.pdpt as *const PageTable as u64;
    let pd_addr = &raw const PAGE_TABLES.pd as *const PageTable as u64;

    pml4_entries.write(pdpt_addr | 0x3);
    pdpt_entries.write(pd_addr | 0x3);
    pd_entries.write(0x83);  // Present + Write + Huge (2MB)
}

unsafe fn setup_gdt() {
    let gdt: UnstableMatter<[GDTEntry; 3]> = UnstableMatter::at(&raw const GDT.entries as *const _ as usize);
    let gdt_ptr = GDTPointer {
        limit: (core::mem::size_of::<[GDTEntry; 3]>() - 1) as u16,
        base: gdt.addr() as u32,
    };
    core::arch::asm!(
        ".code32",
        "lgdt [{0:e}]",  // Use 32-bit addressing
        in(reg) &gdt_ptr,
                     options(att_syntax)
    );
}

unsafe fn enable_paging() {
    let pml4_addr = &raw const PAGE_TABLES.pml4 as *const PageTable as u32;

    core::arch::asm!(
        ".code32",
        "mov %cr4, %eax",
        "or $0x20, %eax",     // Set PAE flag (bit 5)
    "mov %eax, %cr4",

    "mov {0:e}, %eax",
    "mov %eax, %cr3",

    "mov $0xC0000080, %ecx",
    "rdmsr",
    "or $0x100, %eax",    // Set LME flag (bit 8)
    "wrmsr",

    "mov %cr0, %eax",
    "or $0x80000001, %eax", // Set PG and PE flags
    "mov %eax, %cr0",
    in(reg) pml4_addr,
                     options(att_syntax)
    );
}

unsafe fn jump_to_long_mode() -> ! {
    let stack_top = &raw const STACK.data as *const u8 as u64 + 4096;

    core::arch::asm!(
        ".code32",
        "pushl $0x08",
        "pushl $2f",
        "lretl",

        ".align 8",
        ".code64",
        "2:",

        "movw $0x10, %ax",
        "movw %ax, %ds",
        "movw %ax, %es",
        "movw %ax, %fs",
        "movw %ax, %gs",
        "movw %ax, %ss",

        "movq {}, %rsp",

        "call {1}",

        "hlt",
        in(reg) stack_top,
                     sym rust_main,
                     options(noreturn, att_syntax),
    );
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    unsafe {
        SERIAL.init();
        for &b in b"Gear2 starting...\r\n" {
            SERIAL.write_byte(b);
        }
        enter_long_mode();
    }
}

unsafe fn enter_long_mode() -> ! {
    disable_interrupts();
    {
        for &b in b"Disabled interrupts\r\n" {
            SERIAL.write_byte(b);
        }
    }

    setup_page_tables();
    {
        for &b in b"Page tables setup\r\n" {
            SERIAL.write_byte(b);
        }
    }

    setup_gdt();
    {
        for &b in b"GDT setup\r\n" {
            SERIAL.write_byte(b);
        }
    }

    enable_paging();
    {
        for &b in b"Paging enabled\r\n" {
            SERIAL.write_byte(b);
        }
    }

    {
        for &b in b"Jumping to long mode...\r\n" {
            SERIAL.write_byte(b);
        }
    }

    jump_to_long_mode()
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    unsafe {
        for &b in b"PANIC in gear2!\r\n" {
            SERIAL.write_byte(b);
        }
    }
    loop {}
}
