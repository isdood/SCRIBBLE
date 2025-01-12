#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]
#![feature(naked_functions)]

use core::panic::PanicInfo;
mod serial;
use serial::SerialPort;
use unstable_matter::UnstableMatter;
use core::arch::asm;

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
struct DescriptorTablePointer {
    limit: u16,
    base: u64,
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
#[derive(Clone, Copy)]
#[repr(C, packed)]
struct IdtEntry {
    offset_low: u16,
    segment_selector: u16,
    ist: u8,
    flags: u8,
    offset_middle: u16,
    offset_high: u32,
    reserved: u32,
}

impl IdtEntry {
    const fn new(handler: u64) -> Self {
        IdtEntry {
            offset_low: (handler & 0xFFFF) as u16,
            segment_selector: 0x08, // Code segment
            ist: 0,
            flags: 0x8E, // Present, Ring 0, Interrupt Gate
            offset_middle: ((handler >> 16) & 0xFFFF) as u16,
            offset_high: (handler >> 32) as u32,
            reserved: 0,
        }
    }

    const fn missing() -> Self {
        Self::new(0)
    }
}

// Define the IDT structure
#[repr(C, packed)]
struct Idt {
    entries: [IdtEntry; 256],
}

// Static IDT instance
static mut IDT: Idt = Idt {
    entries: [IdtEntry::missing(); 256],
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

// Update page table setup to use raw pointers
unsafe fn setup_page_tables() {
    // Clear tables using raw pointers
    let pml4_ptr = &raw mut PAGE_TABLES.pml4.entries[0] as *mut u64;
    let pdpt_ptr = &raw mut PAGE_TABLES.pdpt.entries[0] as *mut u64;
    let pd_ptr = &raw mut PAGE_TABLES.pd.entries[0] as *mut u64;

    // Zero out tables
    core::ptr::write_bytes(pml4_ptr, 0, 512);
    core::ptr::write_bytes(pdpt_ptr, 0, 512);
    core::ptr::write_bytes(pd_ptr, 0, 512);

    // Set up mappings using raw pointers
    PAGE_TABLES.pml4.entries[0] = (&raw const PAGE_TABLES.pdpt as *const _ as u64) | 0x3;
    PAGE_TABLES.pdpt.entries[0] = (&raw const PAGE_TABLES.pd as *const _ as u64) | 0x3;
    PAGE_TABLES.pd.entries[0] = 0x83;  // Map first 2MB with huge pages

    // Flush TLB using 64-bit register
    core::arch::asm!("mov rax, cr3", "mov cr3, rax");
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

// Function to set up the IDT
unsafe fn setup_idt() {
    // Set up timer interrupt handler
    IDT.entries[32] = IdtEntry::new(timer_interrupt_handler as u64);

    // Load IDT
    let idt_descriptor = DescriptorTablePointer {
        limit: (core::mem::size_of::<Idt>() - 1) as u16,
        base: &IDT as *const Idt as u64,
    };

    asm!("lidt [{}]", in(reg) &idt_descriptor);
}


unsafe fn setup_pic() {
    // ICW1: start initialization
    core::arch::asm!(
        "mov al, 0x11",
        "out 0x20, al", // Master PIC
        "out 0xA0, al", // Slave PIC
        "out 0x80, al"  // Delay
    );

    // ICW2: vector offset
    core::arch::asm!(
        "mov al, 0x20",
        "out 0x21, al", // Master: IRQ 0-7 → INT 0x20-0x27
        "mov al, 0x28",
        "out 0xA1, al", // Slave: IRQ 8-15 → INT 0x28-0x2F
        "out 0x80, al"  // Delay
    );

    // ICW3: cascading
    core::arch::asm!(
        "mov al, 0x04",
        "out 0x21, al", // Master: Slave on IRQ2
        "mov al, 0x02",
        "out 0xA1, al", // Slave: Cascade identity
        "out 0x80, al"  // Delay
    );

    // ICW4: 8086 mode
    core::arch::asm!(
        "mov al, 0x01",
        "out 0x21, al",
        "out 0xA1, al",
        "out 0x80, al"  // Delay
    );

    // OCW1: enable only timer interrupt
    core::arch::asm!(
        "mov al, 0xFE", // Enable IRQ0 (timer), mask others
                     "out 0x21, al",
                     "mov al, 0xFF", // Mask all slave interrupts
                     "out 0xA1, al",
                     "out 0x80, al"  // Delay
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

#[no_mangle]
pub unsafe extern "C" fn _start() -> ! {
    // Disable interrupts during setup
    core::arch::asm!("cli");

    // First set up page tables before enabling paging
    setup_page_tables();

    // Set up IDT
    setup_idt();

    // Set CR3 to point to page tables
    core::arch::asm!(
        "mov rax, {0}",
        "mov cr3, rax",
        in(reg) &PAGE_TABLES.pml4 as *const _ as u64
    );

    // Enable PAE and other required CR4 bits
    core::arch::asm!(
        "mov rax, cr4",
        "or eax, (1 << 5) | (1 << 7)",  // PAE + PGE
                     "mov cr4, rax",
    );

    // Set EFER.LME to enable long mode
    core::arch::asm!(
        "mov ecx, 0xC0000080", // EFER MSR
        "rdmsr",
        "or eax, 0x100",       // Set LME bit
        "wrmsr",
    );

    // Load GDT with 64-bit segments
    setup_gdt();

    // Enable protected mode and paging
    core::arch::asm!(
        "mov rax, cr0",
        "or eax, 0x80000001",  // Enable paging (PG) + protection (PE)
    "mov cr0, rax",
    );

    // Jump to long mode
    core::arch::asm!(
        // Push code segment and target address
        "lea rax, [rip + 2f]",  // Get address of label 2
        "push 0x08",            // Code segment
        "push rax",             // Target address
        "retfq",                // Far return to load CS and jump
        "2:",
        // Load data segments
        "mov ax, 0x10",
        "mov ds, ax",
        "mov es, ax",
        "mov fs, ax",
        "mov gs, ax",
        "mov ss, ax",

        // Set up stack
        "mov rsp, {stack}",
        "mov rbp, rsp",

        // Initialize PIC
        "call {init_pic}",

        // Enable interrupts
        "sti",

        // Jump to Rust main
        "jmp {target}",

        stack = in(reg) &raw const STACK.data as *const u8 as u64 + 4096,
                     init_pic = sym setup_pic,
                     target = sym rust_main,
                     options(noreturn)
    );
}

#[naked]
extern "x86-interrupt" fn timer_interrupt_handler() -> ! {
    unsafe {
        asm!(
            // Save registers
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

             // Send EOI
             "mov al, 0x20",
             "out 0x20, al",

             // Restore registers
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
             options(noreturn)
        );
    }
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
