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
        // 64-bit code segment
        GDTEntry {
            limit_low: 0xFFFF,
            base_low: 0,
            base_middle: 0,
            access: 0x9A,      // Present | Ring 0 | Code | Readable
            granularity: 0x20, // Long mode bit
            base_high: 0,
        },
        // Data segment
        GDTEntry {
            limit_low: 0xFFFF,
            base_low: 0,
            base_middle: 0,
            access: 0x92,      // Present | Ring 0 | Data | Writable
            granularity: 0x00, // No long mode bit needed
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
    let tables_ptr = &raw mut PAGE_TABLES as *mut PageTables;

    // Clear all tables
    (*tables_ptr).pml4.entries.fill(0);
    (*tables_ptr).pdpt.entries.fill(0);
    (*tables_ptr).pd.entries.fill(0);

    // Set up identity mapping
    (*tables_ptr).pml4.entries[0] = (&(*tables_ptr).pdpt as *const PageTable as u64) | 0x3;
    (*tables_ptr).pdpt.entries[0] = (&(*tables_ptr).pd as *const PageTable as u64) | 0x3;
    (*tables_ptr).pd.entries[0] = 0x83; // Present + Write + Huge (2MB)
}

unsafe fn setup_gdt() {
    let gdt_ptr = GDTPointer {
        limit: (core::mem::size_of::<GDTTable>() - 1) as u16,
        base: (&raw const GDT as *const GDTTable) as u32,
    };

    core::arch::asm!(
        ".code32",
        "subl $8, %esp",
        "andl $-8, %esp",
        "movw {limit:x}, (%esp)",
                     "movl {base:e}, 2(%esp)",
                     "lgdt (%esp)",
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
        "movl %cr4, %eax",
        "orl $0x20, %eax",
        "movl %eax, %cr4",
        "movl {addr:e}, %eax",
        "movl %eax, %cr3",
        "movl $0xC0000080, %ecx",
        "rdmsr",
        "orl $0x100, %eax",
        "wrmsr",
        "movl %cr0, %eax",
        "orl $0x80000001, %eax",
        "movl %eax, %cr0",
        addr = in(reg) pml4_addr as u32,
                     options(att_syntax, nomem, nostack)
    );
}

unsafe fn setup_long_mode() {
    // Disable interrupts
    core::arch::asm!("cli");

    // Enable PAE
    core::arch::asm!(
        ".code32",            // Explicitly set 32-bit mode
        "mov eax, cr4",
        "or eax, 1 << 5",     // Set PAE bit
        "mov cr4, eax",
        options(nomem, nostack)
    );

    // Load PML4 table
    let pml4_addr = &raw const PAGE_TABLES.pml4 as *const PageTable as u64;
    core::arch::asm!(
        ".code32",
        "mov eax, {0:e}",
        "mov cr3, eax",
        in(reg) pml4_addr as u32,
                     options(nomem, nostack)
    );

    // Enable long mode in EFER MSR
    core::arch::asm!(
        ".code32",
        "mov ecx, 0xC0000080", // EFER MSR
        "rdmsr",
        "or eax, 1 << 8",      // Set LME bit
        "wrmsr",
        options(nomem, nostack)
    );

    // Enable paging and protection
    core::arch::asm!(
        ".code32",
        "mov eax, cr0",
        "or eax, 1 << 31 | 1", // Set PG and PE bits
        "mov cr0, eax",
        options(nomem, nostack)
    );
}

unsafe fn jump_to_long_mode() -> ! {
    // Load GDT before jumping
    setup_gdt();

    core::arch::asm!(
        ".code32",
        // Ensure stack alignment
        "and esp, -16",
        // Far jump to 64-bit code
        "push dword ptr 0x08", // Code segment selector
        "lea eax, [2f]",       // Get address of label
        "push eax",            // Push target address
        "retf",                // Far return to 64-bit code
        ".align 8",
        "2:",
        ".code64",
        // Load data segment registers
        "mov ax, 0x10",        // Data segment selector
        "mov ds, ax",
        "mov es, ax",
        "mov fs, ax",
        "mov gs, ax",
        "mov ss, ax",
        // Set up stack and jump to Rust
        "mov rsp, {stack}",    // Load stack pointer
        "jmp {target}",        // Jump to rust_main
        stack = in(reg) (&raw const STACK.data as *const u8 as u64 + 4096),
                     target = sym rust_main,
                     options(noreturn)
    );
}

unsafe fn enter_long_mode() -> ! {
    write_serial(b"Setting up page tables...\r\n");
    setup_page_tables();

    write_serial(b"Enabling long mode...\r\n");
    setup_long_mode();

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
