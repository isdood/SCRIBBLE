#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]
#![feature(naked_functions)]

use core::panic::PanicInfo;
mod serial;
use serial::SerialPort;
use unstable_matter::UnstableMatter;

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

// Create a const default entry
const DEFAULT_IDT_ENTRY: IDTEntry = IDTEntry {
    offset_low: 0,
    segment: 0,
    flags: 0,
    offset_middle: 0,
    offset_high: 0,
    reserved: 0,
};

#[repr(C, packed)]
struct InterruptStackFrame {
    instruction_pointer: u64,
    code_segment: u64,
    cpu_flags: u64,
    stack_pointer: u64,
    stack_segment: u64
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

// Define IDT entry structure
#[repr(C, packed)]
#[derive(Copy, Clone)]
struct IDTEntry {
    offset_low: u16,
    segment: u16,
    flags: u16,
    offset_middle: u16,
    offset_high: u32,
    reserved: u32,
}

#[repr(C, align(16))]
struct IDT {
    entries: [IDTEntry; 256],
}

// Single definition of IDT using const array
#[no_mangle]
#[link_section = ".data"]
static mut IDT: IDT = IDT {
    entries: [DEFAULT_IDT_ENTRY; 256],
};

#[repr(C, packed)]
struct GDTPointer {
    limit: u16,
    base: u64,  // Changed from u32 to u64
}

unsafe fn setup_gdt() {
    let gdt_ptr = GDTPointer {
        limit: (core::mem::size_of::<GDTTable>() - 1) as u16,
        base: &raw const GDT as *const _ as u64,  // Changed from u32 to u64
    };

    core::arch::asm!(
        ".code32",
        "lgdt [{0:e}]",
        in(reg) &gdt_ptr,
                     options(readonly)
    );
}

static mut PAGE_TABLES: PageTables = PageTables {
    pml4: PageTable { entries: [0; 512] },
    pdpt: PageTable { entries: [0; 512] },
    pd: PageTable { entries: [0; 512] },
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
    // Clear tables first
    for entry in &mut PAGE_TABLES.pml4.entries {
        *entry = 0;
    }
    for entry in &mut PAGE_TABLES.pdpt.entries {
        *entry = 0;
    }
    for entry in &mut PAGE_TABLES.pd.entries {
        *entry = 0;
    }

    // PML4 -> PDPT
    PAGE_TABLES.pml4.entries[0] = (&PAGE_TABLES.pdpt as *const _ as u64) | 0x3; // Present + Write

    // PDPT -> PD
    PAGE_TABLES.pdpt.entries[0] = (&PAGE_TABLES.pd as *const _ as u64) | 0x3; // Present + Write

    // Map first 2MB using 2MB pages
    PAGE_TABLES.pd.entries[0] = 0x83; // Present + Write + Huge Page

    // Map second 2MB for good measure
    PAGE_TABLES.pd.entries[1] = 0x200083; // Next 2MB, Present + Write + Huge Page

    // Ensure TLB is flushed
    core::arch::asm!(
        "mov cr3, cr3",
        options(nomem, nostack)
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

unsafe fn setup_pic() {
    // Initialize PIC
    core::arch::asm!(
        // ICW1: start initialization
        "mov al, 0x11",
        "out 0x20, al",  // Master PIC
        "out 0xA0, al",  // Slave PIC
        "nop", "nop",

        // ICW2: interrupt vector offsets
        "mov al, 0x20",  // Master: IRQ0 -> INT 0x20
        "out 0x21, al",
        "mov al, 0x28",  // Slave: IRQ8 -> INT 0x28
        "out 0xA1, al",
        "nop", "nop",

        // ICW3: cascading
        "mov al, 0x04",  // Master: IRQ2 has slave
        "out 0x21, al",
        "mov al, 0x02",  // Slave: cascade on IRQ2
        "out 0xA1, al",
        "nop", "nop",

        // ICW4: 8086 mode
        "mov al, 0x01",
        "out 0x21, al",
        "out 0xA1, al",
        "nop", "nop",

        // OCW1: mask all interrupts except timer
        "mov al, 0xFE",  // Enable only IRQ0 (timer)
    "out 0x21, al",
    "mov al, 0xFF",  // Disable all slave interrupts
    "out 0xA1, al",
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

pub unsafe extern "C" fn _start() -> ! {
    // Disable interrupts first
    core::arch::asm!("cli");

    // Initialize serial for debugging
    init_serial();
    write_serial(b"Starting boot sequence...\r\n");

    // Set up IDT
    setup_idt();
    write_serial(b"IDT setup complete\r\n");

    // Set up page tables
    setup_page_tables();
    write_serial(b"Page tables initialized\r\n");

    // Enable PAE
    core::arch::asm!(
        "mov eax, cr4",
        "or eax, 1 << 5",  // PAE
        "mov cr4, eax",
    );
    write_serial(b"PAE enabled\r\n");

    // Set up long mode
    core::arch::asm!(
        "mov ecx, 0xC0000080", // EFER MSR
        "rdmsr",
        "or eax, 1 << 8",      // LME
        "wrmsr",
    );
    write_serial(b"Long mode set in EFER\r\n");

    // Load GDT
    setup_gdt();
    write_serial(b"GDT loaded\r\n");

    // Enable paging
    core::arch::asm!(
        "mov eax, cr0",
        "or eax, 0x80000001",  // PG + PE
        "mov cr0, eax",
    );
    write_serial(b"Paging enabled\r\n");

    // Jump to long mode
    core::arch::asm!(
        "push 0x08",          // Code segment
        "mov eax, 2f",        // Get address of label 2
        "push eax",
        "retf",               // Far return
        "2:",
        ".code64",
        "mov rax, 0x10",      // Data segment
        "mov ds, ax",
        "mov es, ax",
        "mov fs, ax",
        "mov gs, ax",
        "mov ss, ax",

        // Set up stack
        "mov rsp, {stack}",
        "mov rbp, rsp",

        // Jump to Rust
        "jmp {target}",

        stack = in(reg) &STACK.data as *const u8 as u64 + 4096,
                     target = sym rust_main,
                     options(noreturn)
    );
}

unsafe fn setup_idt() {
    // Set up timer interrupt handler (IRQ0 -> INT 0x20)
    IDT.entries[0x20] = IDTEntry {
        offset_low: (timer_interrupt_handler as usize & 0xFFFF) as u16,
        segment: 0x08,  // Code segment
        flags: 0x8E00,  // Present, Ring 0, Interrupt Gate
        offset_middle: ((timer_interrupt_handler as usize >> 16) & 0xFFFF) as u16,
        offset_high: (timer_interrupt_handler as usize >> 32) as u32,
        reserved: 0
    };

    // Set up IDT pointer
    let idtr = GDTPointer {
        limit: (core::mem::size_of::<IDT>() - 1) as u16,
        base: &raw const IDT as *const _ as u64,  // Changed from u32 to u64
    };

    core::arch::asm!(
        "lidt [{0}]",
        in(reg) &idtr,
                     options(readonly, nostack)
    );

    // Load IDT
    core::arch::asm!(
        "lidt [{0}]",
        in(reg) &idtr,
                     options(readonly, nostack)
    );

    let idtr = GDTPointer {
        limit: (core::mem::size_of::<IDT>() - 1) as u16,
        base: &raw const IDT as *const _ as u64,  // Changed from u32 to u64
    };

    core::arch::asm!(
        "lidt [{0}]",
        in(reg) &idtr,
                     options(readonly, nostack)
    );
}

#[naked]
unsafe extern "x86-interrupt" fn timer_interrupt_handler() {
    core::arch::naked_asm!(
        // Save all registers, not just some
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

        // Send EOI to PIC
        "mov al, 20h",
        "out 20h, al",

        // Restore all registers in reverse order
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
