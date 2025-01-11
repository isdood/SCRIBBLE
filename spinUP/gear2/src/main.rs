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
    pml4: [u64; 512],
    pdpt: [u64; 512],
    pd: [u64; 512],
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
    pml4: [0; 512],
    pdpt: [0; 512],
    pd: [0; 512],
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
    // Identity map first 2MB
    PAGE_TABLES.pml4[0] = &raw const PAGE_TABLES.pdpt as *const _ as u64 | 0x3;
    PAGE_TABLES.pdpt[0] = &raw const PAGE_TABLES.pd as *const _ as u64 | 0x3;
    PAGE_TABLES.pd[0] = 0x83;  // Present + Write + Huge (2MB)

    core::arch::asm!("mfence");
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
    // Set up initial stack
    core::arch::asm!(
        "mov esp, 0x7c00",
        options(nomem, nostack)
    );

    // Disable interrupts
    core::arch::asm!("cli");

    // Check for long mode support
    if !check_long_mode() {
        loop { core::arch::asm!("hlt"); }
    }

    // Set up page tables with proper identity mapping
    setup_page_tables();

    // Initialize GDT before enabling long mode
    setup_gdt();

    // Enable PAE
    core::arch::asm!(
        ".code32",
        "mov eax, cr4",
        "or eax, 1 << 5",     // Set PAE bit
        "mov cr4, eax",
        options(nomem, nostack)
    );

    // Load CR3 with PML4 address
    core::arch::asm!(
        ".code32",
        "mov eax, {pml4:e}",
        "mov cr3, eax",
        pml4 = in(reg) &raw const PAGE_TABLES.pml4 as *const _ as u32,
                     options(nomem, nostack)
    );

    // Enable long mode
    core::arch::asm!(
        ".code32",
        "mov ecx, 0xC0000080", // EFER MSR
        "rdmsr",
        "or eax, 1 << 8",      // Set LME
        "wrmsr",
        options(nomem, nostack)
    );

    // Enable paging and protection
    core::arch::asm!(
        ".code32",
        "mov eax, cr0",
        "or eax, 0x80000001",  // Set PG and PE
        "mov cr0, eax",
        options(nomem, nostack)
    );

    // Set up IDT for interrupt handling
    setup_idt();

    // Jump to long mode
    core::arch::asm!(
        ".code32",
        // Load GDT
        "lgdt [{gdt_ptr:e}]",
        // Far jump to 64-bit code
        "jmp 0x08:2f",        // 0x08 is the code segment selector
        ".align 8",
        "2:",
        ".code64",
        // Set up 64-bit environment
        "mov rsp, 0x7c00",    // Reset stack
        // Clear segment registers
        "mov ax, 0x10",       // Data segment selector
        "mov ss, ax",
        "xor ax, ax",
        "mov ds, ax",
        "mov es, ax",
        "mov fs, ax",
        "mov gs, ax",
        // Enable interrupts
        "sti",
        // Jump to Rust main
        "jmp {target}",
        gdt_ptr = in(reg) &raw const GDT_PTR,
                     target = sym rust_main,
                     options(noreturn)
    );
}

#[repr(C, packed(2))]
struct DescriptorTablePointer {
    limit: u16,
    base: u32,
}

// GDT entries for 64-bit mode
static mut GDT: [u64; 4] = [
    0,                      // Null descriptor
0x00AF9A000000FFFF,    // Code segment (64-bit)
0x00CF92000000FFFF,    // Data segment
0,                      // Task State Segment (reserved)
];

static mut GDT_PTR: DescriptorTablePointer = DescriptorTablePointer {
    limit: (4 * 8 - 1) as u16,
    base: 0,  // Will be set at runtime
};

// IDT setup for basic interrupt handling
#[repr(C, packed)]
struct IDTEntry {
    offset_low: u16,
    segment: u16,
    flags: u16,
    offset_mid: u16,
    offset_high: u32,
    reserved: u32,
}

static mut IDT: [IDTEntry; 256] = [IDTEntry {
    offset_low: 0,
    segment: 0,
    flags: 0,
    offset_mid: 0,
    offset_high: 0,
    reserved: 0,
}; 256];

static mut IDT_PTR: DescriptorTablePointer = DescriptorTablePointer {
    limit: (256 * 16 - 1) as u16,
    base: 0,
};

unsafe fn setup_gdt() {
    GDT_PTR.base = &raw const GDT as *const _ as u32;
}

unsafe fn setup_idt() {
    // Set up basic interrupt handlers
    for i in 0..256 {
        IDT[i] = IDTEntry {
            offset_low: (interrupt_handler as u64 & 0xFFFF) as u16,
            segment: 0x08,  // Code segment
            flags: 0x8E00,  // Present, Ring 0, Interrupt Gate
            offset_mid: ((interrupt_handler as u64 >> 16) & 0xFFFF) as u16,
            offset_high: (interrupt_handler as u64 >> 32) as u32,
            reserved: 0,
        };
    }

    IDT_PTR.base = &raw const IDT as *const _ as u32;

    // Load IDT
    core::arch::asm!(
        "lidt [{0:e}]",
        in(reg) &IDT_PTR,
                     options(readonly, nostack)
    );
}

#[naked]
unsafe extern "C" fn interrupt_handler() {
    core::arch::asm!(
        "push rax",
        "push rcx",
        "push rdx",
        "push r8",
        "push r9",
        "push r10",
        "push r11",
        "mov al, 0x20",
        "out 0x20, al",      // Send EOI to PIC
        "pop r11",
        "pop r10",
        "pop r9",
        "pop r8",
        "pop rdx",
        "pop rcx",
        "pop rax",
        "iretq",
        options(noreturn)
    );
}
