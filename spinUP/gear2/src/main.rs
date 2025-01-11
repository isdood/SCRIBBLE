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
    // Directly manipulate the static to avoid any potential UB
    SERIAL_PORT = Some(SerialPort::new(0x3F8));

    // Get a raw pointer to the inner SerialPort
    if let Some(ref mut port) = SERIAL_PORT {
        port.init();
    }
}

unsafe fn write_serial(msg: &[u8]) {
    // Use a raw pointer to access the port
    if let Some(ref mut port) = SERIAL_PORT {
        for &b in msg {
            port.write_byte(b);
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
    // First clear all tables
    PAGE_TABLES.pml4.entries.fill(0);
    PAGE_TABLES.pdpt.entries.fill(0);
    PAGE_TABLES.pd.entries.fill(0);

    // Set up identity mapping for first 2MB
    PAGE_TABLES.pml4.entries[0] = (&PAGE_TABLES.pdpt as *const PageTable as u64) | 0x3;
    PAGE_TABLES.pdpt.entries[0] = (&PAGE_TABLES.pd as *const PageTable as u64) | 0x3;
    PAGE_TABLES.pd.entries[0] = 0x83; // Present + Write + Huge (2MB)
}

unsafe fn setup_gdt() {
    let gdt: UnstableMatter<[GDTEntry; 3]> = UnstableMatter::at(&raw const GDT.entries as *const _ as usize);
    let gdt_ptr = GDTPointer {
        limit: (core::mem::size_of::<[GDTEntry; 3]>() - 1) as u16,
        base: gdt.addr() as u32,
    };

    core::arch::asm!(
        ".code32",
        "sub esp, 6",           // Make space for GDTR
        "mov [{0}], {1:x}",    // Store limit
        "mov [{0} + 2], {2:e}", // Store base
        "lgdt [esp]",          // Load GDT
        "add esp, 6",          // Restore stack
        in(reg) "esp" => _,
                     in(reg) gdt_ptr.limit,
                     in(reg) gdt_ptr.base,
                     options(att_syntax)
    );
}

unsafe fn enable_paging() {
    let pml4_addr = &PAGE_TABLES.pml4 as *const PageTable as u64;

    core::arch::asm!(
        ".code32",
        // Enable PAE
        "mov %cr4, %eax",
        "or $0x20, %eax",
        "mov %eax, %cr4",

        // Load CR3 with PML4 address
        "mov {0:e}, %eax",
        "mov %eax, %cr3",

        // Enable long mode in EFER
        "mov $0xC0000080, %ecx",
        "rdmsr",
        "or $0x100, %eax",
        "wrmsr",

        // Enable paging
        "mov %cr0, %eax",
        "or $0x80000001, %eax",
        "mov %eax, %cr0",
        in(reg) pml4_addr,
                     options(att_syntax)
    );
}

unsafe fn jump_to_long_mode() -> ! {
    let stack_top = &STACK.data as *const u8 as u64 + 4096;

    core::arch::asm!(
        ".code32",
        // Load new code segment
        "push $0x08",         // New CS value (1st GDT entry)
    "lea 1f(%eip), %eax", // Get address of label 1
                     "push %eax",          // Push return address
                     "retf",               // Far return to load CS and jump

                     ".code64",
                     "1:",                 // Long mode entry point
                     // Load data segments
                     "mov $0x10, %ax",
                     "mov %ax, %ds",
                     "mov %ax, %es",
                     "mov %ax, %fs",
                     "mov %ax, %gs",
                     "mov %ax, %ss",

                     // Set up new stack
                     "mov {}, %rsp",

                     // Jump to Rust
                     "call {}",

                     "hlt",
                     in(reg) stack_top,
                     sym rust_main,
                     options(noreturn, att_syntax)
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
