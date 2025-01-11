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
        // 64-bit code segment
        GDTEntry {
            limit_low: 0,      // Limit ignored in 64-bit mode
            base_low: 0,       // Base ignored in 64-bit mode
            base_middle: 0,    // Base ignored in 64-bit mode
            access: 0x9A,      // Present(1) | DPL(00) | S(1) | Type(1010)
            granularity: 0x20, // Long mode (1) | Default Op Size (0) | Granularity (0)
            base_high: 0,      // Base ignored in 64-bit mode
        },
        // Data segment
        GDTEntry {
            limit_low: 0,      // Limit ignored in 64-bit mode
            base_low: 0,       // Base ignored in 64-bit mode
            base_middle: 0,    // Base ignored in 64-bit mode
            access: 0x92,      // Present(1) | DPL(00) | S(1) | Type(0010)
            granularity: 0,    // Long mode (0) | Default Op Size (0) | Granularity (0)
            base_high: 0,      // Base ignored in 64-bit mode
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
    // Get pointers to the whole tables, not just first entries
    let tables_ptr = &raw mut PAGE_TABLES as *mut PageTables;

    // Clear all tables
    (*tables_ptr).pml4.entries.fill(0);
    (*tables_ptr).pdpt.entries.fill(0);
    (*tables_ptr).pd.entries.fill(0);

    // Set up identity mapping
    (*tables_ptr).pml4.entries[0] = (&raw const (*tables_ptr).pdpt as *const PageTable as u64) | 0x3;
    (*tables_ptr).pdpt.entries[0] = (&raw const (*tables_ptr).pd as *const PageTable as u64) | 0x3;
    (*tables_ptr).pd.entries[0] = 0x83; // Present + Write + Huge (2MB)
}

unsafe fn setup_gdt() {
    let gdt_ptr = GDTPointer {
        limit: (core::mem::size_of::<GDTTable>() - 1) as u16,
        base: (&GDT as *const GDTTable) as u32,
    };

    core::arch::asm!(
        ".code32",
        // Create space on stack and ensure alignment
        "subl $8, %esp",
        "andl $-8, %esp",

        // Store GDTR data
        "movw {limit:x}, (%esp)",     // Store limit
                     "movl {base:e}, 2(%esp)",     // Store base
                     "lgdt (%esp)",                // Load GDT

                     // Restore stack
                     "addl $8, %esp",

                     limit = in(reg) gdt_ptr.limit,
                     base = in(reg) gdt_ptr.base,
                     options(att_syntax)
    );
}

unsafe fn enable_paging() {
    let pml4_addr = &raw const PAGE_TABLES.pml4 as *const PageTable as u64;

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
    let stack_top = &raw const STACK.data as *const u8 as u64 + 4096;

    core::arch::asm!(
        ".code32",
        // Set up far jump to 64-bit code
        "movl $0x08, -8(%esp)",   // Push CS selector
                     "movl $1f, -4(%esp)",     // Push target address
                     "ljmp *-8(%esp)",         // Far jump using memory operand

                     ".align 8",               // Ensure 64-bit alignment
                     "1:",                     // Long mode entry point
                     ".code64",
                     // Set up segment registers
                     "movw $0x10, %ax",
                     "movw %ax, %ds",
                     "movw %ax, %es",
                     "movw %ax, %fs",
                     "movw %ax, %gs",
                     "movw %ax, %ss",

                     // Set up stack and call main
                     "movq {}, %rsp",
                     "callq *{}",
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
