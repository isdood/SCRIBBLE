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

#[repr(C, align(8))]  // Ensure 8-byte alignment for the GDT
struct GDTTable {
    entries: [GDTEntry; 3]
}

#[repr(C, packed)]
struct GDTPointer {
    limit: u16,
    base: u32,  // Using u32 for 32-bit mode
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
#[allow(dead_code)]
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
    core::arch::asm!(
        ".code32",
        "cli",
        options(nomem, nostack)
    );
}

unsafe fn setup_page_tables() {
    // Need to ensure CR3 is loaded after tables are set up
    PAGE_TABLES.pml4.entries[0] = (&raw const PAGE_TABLES.pdpt as *const _ as u64) | 0x3;
    PAGE_TABLES.pdpt.entries[0] = (&raw const PAGE_TABLES.pd as *const _ as u64) | 0x3;
    PAGE_TABLES.pd.entries[0] = 0x83;

    core::arch::asm!(
        ".code32",
        "mov {tmp:e}, {addr:e}",
        "mov cr3, {tmp:e}",
        addr = in(reg) &raw const PAGE_TABLES.pml4 as *const _ as u32,
                     tmp = out(reg) _,
                     options(nomem, nostack)
    );
}

unsafe fn setup_gdt() {
    let gdt_ptr = GDTPointer {
        limit: (core::mem::size_of::<GDTTable>() - 1) as u16,
        base: &raw const GDT as *const _ as u32,
    };

    core::arch::asm!(
        ".code32",
        "lgdt [{0:e}]",  // Use 32-bit register (eax) with :e suffix
                     in(reg) &gdt_ptr,
                     options(readonly, nostack)
    );
}

#[allow(dead_code)]
fn get_cpuid() -> (u32, u32, u32, u32) {
    let eax: u32;
    let ecx: u32;
    let edx: u32;

    unsafe {
        core::arch::asm!(
            ".code32",
            "mov edi, ebx",    // Save ebx
            "cpuid",
            "xchg edi, ebx",   // Restore ebx and get its value
            inout("eax") 0 => eax,
                         out("ecx") ecx,
                         out("edx") edx,
                         out("edi") _,      // Use edi instead of ebx
        );
    }

    // Since we can't directly use ebx, we'll return 0 for that value
    // as it's not critical for our long mode check
    (eax, 0, ecx, edx)
}

unsafe fn check_long_mode() -> bool {
    // Check CPUID presence
    let mut flags: u32;
    core::arch::asm!(
        ".code32",
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
        out("eax") flags,
                     out("ecx") _,
    );

    if flags == 0 {
        return false;
    }

    // Check for extended processor info
    let max_cpuid: u32;
    core::arch::asm!(
        ".code32",
        "cpuid",
        inlateout("eax") 0x80000000u32 => max_cpuid,
                     lateout("ecx") _,
                     lateout("edx") _,
    );

    if max_cpuid < 0x80000001 {
        return false;
    }

    // Check for long mode support
    let edx: u32;
    core::arch::asm!(
        ".code32",
        "cpuid",
        inlateout("eax") 0x80000001u32 => _,
                     lateout("ecx") _,
                     lateout("edx") edx,
    );

    (edx & (1 << 29)) != 0 // LM bit
}

// Fix other CR register operations
#[allow(dead_code)]
unsafe fn enable_pae() {
    core::arch::asm!(
        ".code32",
        "mov eax, cr4",
        "or eax, 1 << 5",     // Set PAE bit
        "mov cr4, eax",
        options(nomem, nostack)
    );
}

#[allow(dead_code)]
unsafe fn enable_paging() {
    core::arch::asm!(
        ".code32",
        "mov eax, cr0",
        "or eax, 0x80000001", // Set PG and PE bits
        "mov cr0, eax",
        options(nomem, nostack)
    );
}

#[allow(dead_code)]
unsafe fn setup_long_mode() {
    // Disable interrupts
    core::arch::asm!(
        ".code32",
        "cli",
        options(nomem, nostack)
    );

    // Load PML4 table
    core::arch::asm!(
        ".code32",
        "mov {tmp:e}, {pml4:e}",
        "mov cr3, {tmp:e}",
        pml4 = in(reg) &raw const PAGE_TABLES.pml4 as *const _ as u32,
                     tmp = out(reg) _,
                     options(nomem, nostack)
    );

    // Enable long mode in EFER MSR
    core::arch::asm!(
        ".code32",
        "mov ecx, 0xC0000080", // EFER MSR
        "rdmsr",
        "or eax, 0x100",       // Set LME bit (1 << 8)
    "wrmsr",
    options(nomem, nostack)
    );
}

#[allow(dead_code)]
unsafe fn enter_long_mode() -> ! {
    // Disable interrupts first
    core::arch::asm!(
        ".code32",
        "cli",
        options(nomem, nostack)
    );
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
        "jz 3f",               // If PAE not set, halt
        "jmp 4f",
        "3: hlt",
        "4:",
        options(nomem, nostack)
    );
    write_serial(b"Enabled PAE\r\n");

    // Load CR3 with PML4
    core::arch::asm!(
        ".code32",
        "mov {tmp:e}, {addr:e}",
        "mov cr3, {tmp:e}",
        addr = in(reg) &raw const PAGE_TABLES.pml4 as *const _ as u32,
                     tmp = out(reg) _,
                     options(nomem, nostack)
    );

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
        "jz 5f",               // If LME not set, halt
        "jmp 6f",
        "5: hlt",
        "6:",
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
        "lea eax, [4f]",       // Target address
        "push eax",
        "retf",                // Far return to 64-bit code
        ".align 8",
        "4:",
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
        stack = in(reg) &raw const STACK.data as *const u8 as u64 + 4096,
                     target = sym rust_main,
                     options(noreturn)
    );
}

#[allow(dead_code)]
unsafe fn check_paging_enabled() -> bool {
    let cr0: u32;
    core::arch::asm!(
        ".code32",
        "mov {0:e}, cr0",
        out(reg) cr0,
                     options(nomem, nostack)
    );
    (cr0 & (1 << 31)) != 0
}

#[no_mangle]
pub unsafe extern "C" fn _start() -> ! {
    init_serial();
    write_serial(b"Serial initialized\r\n");

    disable_interrupts();
    write_serial(b"Interrupts disabled\r\n");

    // Check for long mode support first
    if !check_long_mode() {
        write_serial(b"Long mode not supported\r\n");
        loop { core::arch::asm!("hlt", options(nomem, nostack)); }
    }
    write_serial(b"Long mode supported\r\n");

    // Setup page tables before enabling any CPU features
    setup_page_tables();
    write_serial(b"Page tables set up\r\n");

    // Enable PAE (Physical Address Extension)
    core::arch::asm!(
        ".code32",
        "mov eax, cr4",
        "or eax, 1 << 5",      // Set PAE bit
        "mov cr4, eax",
        // Verify PAE was set
        "mov eax, cr4",
        "test eax, 1 << 5",
        "jnz 2f",
        "hlt",                 // Halt if PAE not set
        "2:",
        options(nomem, nostack)
    );
    write_serial(b"PAE enabled\r\n");

    // Setup GDT for long mode
    setup_gdt();
    write_serial(b"GDT loaded\r\n");

    // Enable long mode in EFER MSR
    core::arch::asm!(
        ".code32",
        "mov ecx, 0xC0000080", // EFER MSR
        "rdmsr",
        "or eax, 1 << 8",      // Set LME bit
        "wrmsr",
        // Verify LME was set
        "rdmsr",
        "test eax, 1 << 8",
        "jnz 3f",
        "hlt",                 // Halt if LME not set
        "3:",
        options(nomem, nostack)
    );
    write_serial(b"Long mode enabled in EFER\r\n");

    // Enable paging and protected mode
    core::arch::asm!(
        ".code32",
        "mov eax, cr0",
        "or eax, 1 << 31 | 1", // Set PG and PE bits
        "mov cr0, eax",
        // Verify paging was enabled
        "mov eax, cr0",
        "test eax, 1 << 31",
        "jnz 4f",
        "hlt",                 // Halt if paging not set
        "4:",
        options(nomem, nostack)
    );
    write_serial(b"Paging enabled\r\n");

    write_serial(b"Jumping to 64-bit mode...\r\n");

    // Far jump to 64-bit mode
    core::arch::asm!(
        ".code32",
        // Ensure stack alignment
        "and esp, -16",
        // Far jump preparation
        "push dword ptr 0x08", // Long mode code segment
        "lea eax, [5f]",      // Get address of 64-bit code
        "push eax",
        "retf",               // Far return to load CS with 64-bit segment
        ".align 8",
        "5:",                 // 64-bit code starts here
        ".code64",
        // Initialize segment registers
        "mov ax, 0x10",       // Data segment selector
        "mov ds, ax",
        "mov es, ax",
        "mov fs, ax",
        "mov gs, ax",
        "mov ss, ax",
        // Set up stack and jump to Rust main
        "mov rsp, {stack}",
        "jmp {target}",
        stack = in(reg) &raw const STACK.data as *const u8 as u64 + 4096,
                     target = sym rust_main,
                     options(noreturn)
    );
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {
        unsafe {
            core::arch::asm!(
                ".code32",
                "hlt",
                options(nomem, nostack)
            );
        }
    }
}
