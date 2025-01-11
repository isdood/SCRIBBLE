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

// In both stages
#[repr(C, packed)]
pub struct StageInfo {
    boot_drive: u8,
    memory_map_addr: u32,
    memory_entries: u16,
    stage2_load_addr: u32,
    flags: u32,
}

// Use this to pass information between stages
static mut STAGE_INFO: StageInfo = StageInfo {
    boot_drive: 0,
    memory_map_addr: 0,
    memory_entries: 0,
    stage2_load_addr: 0x7E00,
    flags: 0,
};

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
            access: 0x9A,      // Present(1) | DPL(00) | S(1) | Type(1010)
            granularity: 0xA0, // G(1) | L(1) | Limit(0000)
            base_high: 0,
        },
        // Data segment
        GDTEntry {
            limit_low: 0xFFFF,
            base_low: 0,
            base_middle: 0,
            access: 0x92,      // Present(1) | DPL(00) | S(1) | Type(0010)
            granularity: 0xC0, // G(1) | D/B(1) | Limit(0000)
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

unsafe fn disable_interrupts() {
    core::arch::asm!("cli");
}
unsafe fn setup_page_tables() {
    // Clear tables first
    for table in &mut PAGE_TABLES.pdpt.entries {
        *table = 0;
    }

    // Identity map first 2MB
    PAGE_TABLES.pml4.entries[0] =
    (&PAGE_TABLES.pdpt as *const _ as u64) | 0x3;
    PAGE_TABLES.pdpt.entries[0] =
    (&PAGE_TABLES.pd as *const _ as u64) | 0x3;
    PAGE_TABLES.pd.entries[0] = 0x83; // PS=1, RW=1, P=1
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

unsafe fn setup_page_tables() {
    // Identity map first 2MB of memory
    PAGE_TABLES.pml4.entries[0] = (&PAGE_TABLES.pdpt as *const _ as u64) | 0x3; // Present + R/W
    PAGE_TABLES.pdpt.entries[0] = (&PAGE_TABLES.pd as *const _ as u64) | 0x3;   // Present + R/W
    PAGE_TABLES.pd.entries[0] = 0x83;  // Present + R/W + PS (2MB page)
}

unsafe fn check_long_mode() -> bool {
    // Check CPUID presence
    let has_cpuid: bool;
    core::arch::asm!(
        "pushfd",
        "pop eax",
        "mov ecx, eax",
        "xor eax, 1 << 21",
        "push eax",
        "popfd",
        "pushfd",
        "pop eax",
        "xor eax, ecx",
        "shr eax, 21",
        "and eax, 1",
        out("eax") has_cpuid,
                     out("ecx") _,
    );

    if !has_cpuid {
        return false;
    }

    // Check for extended processor info
    let mut max_cpuid: u32;
    core::arch::asm!(
        "cpuid",
        inout("eax") 0x80000000 => max_cpuid,
                     out("ebx") _,
                     out("ecx") _,
                     out("edx") _,
    );

    if max_cpuid < 0x80000001 {
        return false;
    }

    // Check for long mode support
    let mut edx: u32;
    core::arch::asm!(
        "cpuid",
        inout("eax") 0x80000001 => _,
                     out("ebx") _,
                     out("ecx") _,
                     out("edx") edx,
    );

    (edx & (1 << 29)) != 0 // LM bit
}

unsafe fn setup_long_mode() {
    // Disable interrupts
    core::arch::asm!("cli");

    // Enable PAE
    core::arch::asm!(
        "mov eax, cr4",
        "or eax, 0x20",       // Set PAE bit (1 << 5)
    "mov cr4, eax",
    options(nomem, nostack)
    );

    // Load PML4 table
    core::arch::asm!(
        "mov eax, {pml4}",
        "mov cr3, eax",
        pml4 = in(reg) &PAGE_TABLES.pml4 as *const _ as u32,
                     options(nomem, nostack)
    );

    // Enable long mode in EFER MSR
    core::arch::asm!(
        "mov ecx, 0xC0000080", // EFER MSR
        "rdmsr",
        "or eax, 0x100",       // Set LME bit (1 << 8)
    "wrmsr",
    options(nomem, nostack)
    );

    // Enable paging and protection
    core::arch::asm!(
        "mov eax, cr0",
        "or eax, 0x80000001",  // Set PG and PE bits
        "mov cr0, eax",
        options(nomem, nostack)
    );
}

fn get_cpuid() -> (u32, u32, u32, u32) {
    let eax: u32;
    let ebx: u32;
    let ecx: u32;
    let edx: u32;

    unsafe {
        core::arch::asm!(
            "cpuid",
            inout("eax") 0 => eax,
                         out("ebx") ebx,
                         out("ecx") ecx,
                         out("edx") edx,
        );
    }

    (eax, ebx, ecx, edx)
}

unsafe fn setup_gdt() {
    let gdt_ptr = GDTPointer {
        limit: (core::mem::size_of::<GDTTable>() - 1) as u16,
        base: &GDT as *const _ as u32,
    };

    core::arch::asm!(
        "lgdt [{0}]",
        in(reg) &gdt_ptr,
                     options(readonly)
    );
}

unsafe fn enter_long_mode() -> ! {
    // Disable interrupts first
    core::arch::asm!("cli");
    write_serial(b"Disabled interrupts\r\n");

    // Setup page tables
    write_serial(b"Setting up page tables...\r\n");
    setup_page_tables();

    // Verify CR0 is in a known state
    core::arch::asm!(
        ".code32",
        "mov eax, cr0",
        "and eax, 0x7fffffff", // Clear PG
        "or eax, 1",           // Set PE
        "mov cr0, eax",
        options(nomem, nostack)
    );
    write_serial(b"Set initial CR0 state\r\n");

    // Enable PAE
    core::arch::asm!(
        ".code32",
        "mov eax, cr4",
        "or eax, 1 << 5",      // Set PAE
        "mov cr4, eax",
        "mov eax, cr4",        // Verify PAE was set
        "test eax, 1 << 5",
        "jz 1f",               // If PAE not set, halt
        "jmp 2f",
        "1: hlt",
        "2:",
        options(nomem, nostack)
    );
    write_serial(b"Enabled PAE\r\n");

    // Load CR3 with PML4
    let pml4_addr = &raw const PAGE_TABLES.pml4 as *const PageTable as u64;
    core::arch::asm!(
        ".code32",
        "mov eax, {0:e}",
        "mov cr3, eax",
        in(reg) pml4_addr as u32,
                     options(nomem, nostack)
    );
    write_serial(b"Loaded CR3\r\n");

    // Enable long mode in EFER MSR
    core::arch::asm!(
        ".code32",
        "mov ecx, 0xC0000080", // EFER MSR
        "rdmsr",
        "or eax, 1 << 8",      // Set LME
        "wrmsr",
        // Verify EFER.LME was set
        "rdmsr",
        "test eax, 1 << 8",
        "jz 1f",               // If LME not set, halt
        "jmp 2f",
        "1: hlt",
        "2:",
        options(nomem, nostack)
    );
    write_serial(b"Enabled long mode in EFER\r\n");

    // Load GDT for long mode
    setup_gdt();
    write_serial(b"Loaded GDT\r\n");

    // Enable paging to activate long mode
    core::arch::asm!(
        ".code32",
        "mov eax, cr0",
        "or eax, 0x80000000",  // Set PG
        "mov cr0, eax",
        options(nomem, nostack)
    );
    write_serial(b"Enabled paging\r\n");

    write_serial(b"Jumping to 64-bit mode...\r\n");

    // Far jump to 64-bit code
    core::arch::asm!(
        ".code32",
        // Ensure stack alignment
        "and esp, -16",
        // Prepare far jump
        "push dword ptr 0x08",  // CS selector
        "lea eax, [2f]",       // Target address
        "push eax",
        "retf",                // Far return to 64-bit code
        ".align 8",
        "2:",
        ".code64",
        // Zero segment registers
        "xor ax, ax",
        "mov ds, ax",
        "mov es, ax",
        "mov fs, ax",
        "mov gs, ax",
        "mov ss, ax",
        // Set up final stack and jump to Rust
        "mov rsp, {stack}",
        "jmp {target}",
        stack = in(reg) (&raw const STACK.data as *const u8 as u64 + 4096),
                     target = sym rust_main,
                     options(noreturn)
    );
}

#[no_mangle]
pub unsafe extern "C" fn _start() -> ! {
    // Initialize serial port for debugging
    init_serial();
    write_serial(b"Serial initialized\r\n");

    // Check if long mode is available
    if !check_long_mode() {
        write_serial(b"Long mode not supported\r\n");
        loop { core::arch::asm!("hlt"); }
    }
    write_serial(b"Long mode supported\r\n");

    // Set up paging structures
    setup_page_tables();
    write_serial(b"Page tables set up\r\n");

    // Enable PAE
    core::arch::asm!(
        "mov eax, cr4",
        "or eax, 1 << 5",     // Set PAE bit
        "mov cr4, eax",
        options(nomem, nostack)
    );
    write_serial(b"PAE enabled\r\n");

    // Load CR3 with PML4
    core::arch::asm!(
        "mov cr3, {0}",
        in(reg) &PAGE_TABLES.pml4 as *const _ as u64,
                     options(nomem, nostack)
    );
    write_serial(b"CR3 loaded\r\n");

    // Enable long mode
    core::arch::asm!(
        "mov ecx, 0xC0000080", // EFER MSR
        "rdmsr",
        "or eax, 1 << 8",      // Set LME bit
        "wrmsr",
        options(nomem, nostack)
    );
    write_serial(b"Long mode enabled in EFER\r\n");

    // Setup GDT for long mode
    setup_gdt();
    write_serial(b"GDT loaded\r\n");

    // Enable paging
    core::arch::asm!(
        "mov eax, cr0",
        "or eax, 1 << 31 | 1", // Set PG and PE bits
        "mov cr0, eax",
        options(nomem, nostack)
    );
    write_serial(b"Paging enabled\r\n");

    // Jump to long mode
    core::arch::asm!(
        "push 0x08",          // Code segment
        "lea {tmp}, [1f]",    // Get address of label
        "push {tmp}",         // Push address
        "retf",              // Far return to 64-bit mode
        "1:",
        ".code64",
        "mov ax, 0x10",      // Data segment
        "mov ds, ax",
        "mov es, ax",
        "mov fs, ax",
        "mov gs, ax",
        "mov ss, ax",
        // Set up stack and jump to Rust
        "mov rsp, {stack}",
        "jmp {target}",
        tmp = out(reg) _,
                     stack = in(reg) (&STACK.data as *const _ as u64 + 4096),
                     target = sym rust_main,
                     options(noreturn)
    );
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {
        unsafe { core::arch::asm!("hlt"); }
    }
}
