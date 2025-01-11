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

static mut SERIAL_PORT: Option<SerialPort> = None;

// Helper function to safely initialize and use the serial port
unsafe fn init_serial() {
    let serial_ptr = &raw mut SERIAL_PORT;
    *serial_ptr = Some(SerialPort::new(0x3F8));
    if let Some(serial) = &raw mut (*serial_ptr) {
        serial.init();
    }
}

unsafe fn write_serial(msg: &[u8]) {
    let serial_ptr = &raw mut SERIAL_PORT;
    if let Some(serial) = &raw mut (*serial_ptr) {
        for &b in msg {
            serial.write_byte(b);
        }
    }
}

#[no_mangle]
pub extern "C" fn rust_main() -> ! {
    unsafe {
        let mut vga = UnstableMatter::<u16>::at(0xB8000);
        let msg = b"Long Mode OK!";

        // Clear screen
        for _ in 0..80*25 {
            vga.write(0x0F00);
        }

        // Write message
        for (_, &byte) in msg.iter().enumerate() {
            vga.write(0x0F00 | byte as u16);
        }

        loop {
            core::arch::asm!("hlt", options(nomem, nostack));
        }
    }
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    unsafe {
        init_serial();
        write_serial(b"Gear2 starting...\r\n");
        enter_long_mode();
    }
}

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

    // Using a similar approach to Gear1's assembly style
    core::arch::asm!(
        ".code32",
        "mov {0:e}, %esi",  // Load pointer to GDTR into ESI
        "lgdt (%esi)",      // Load GDT using indirect addressing
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

unsafe fn enter_long_mode() -> ! {
    disable_interrupts();
    write_serial(b"Disabled interrupts\r\n");

    setup_page_tables();
    write_serial(b"Page tables setup\r\n");

    setup_gdt();
    write_serial(b"GDT setup\r\n");

    enable_paging();
    write_serial(b"Paging enabled\r\n");

    write_serial(b"Jumping to long mode...\r\n");
    jump_to_long_mode()
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    unsafe {
        write_serial(b"PANIC in gear2!\r\n");
    }
    loop {}
}
