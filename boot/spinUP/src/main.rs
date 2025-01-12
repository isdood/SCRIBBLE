// src/main.rs
// Last updated: 2025-01-12 04:43:17 UTC
// Author: isdood

#![no_std]
#![no_main]
#![feature(naked_functions)]
#![feature(abi_x86_interrupt)]

use core::panic::PanicInfo;
use unstable_matter::unstable_vectrix::{UnstableVectrix, VirtAddr, PhysAddr, Dimensions};
use unstable_matter::unstable_vectrix::page_table::{PageTable, PageTableEntry};

const KERNEL_LOAD_ADDR: u64 = 0x100000;  // Load kernel at 1MB
const KERNEL_SECTOR_START: u16 = 33;     // Kernel starts at sector 33
const SECTORS_TO_READ: u16 = 100;        // Adjust based on kernel size

const VGA_BUFFER: *mut u8 = 0xb8000 as *mut u8;
static mut VGA_CURSOR: usize = 0;

// At the top of main.rs
pub const BOOT_START: u64 = 0x7E00;
pub const BOOT_STACK_START: u64 = 0x9000;
pub const BOOT_STACK_SIZE: u64 = 0x4000;

mod debug {
    use super::UnstableChar;
    use core::fmt;

    pub struct DebugWriter;

    impl DebugWriter {
        pub fn new() -> Self {
            DebugWriter
        }
    }

    impl fmt::Write for DebugWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            unsafe {
                super::print(s);
            }
            Ok(())
        }
    }

    #[macro_export]
    macro_rules! debug_print {
        ($($arg:tt)*) => ({
            use core::fmt::Write;
            let mut writer = $crate::debug::DebugWriter::new();
            writer.write_fmt(format_args!($($arg)*)).unwrap();
        });
    }

    #[macro_export]
    macro_rules! debug_println {
        () => ($crate::debug_print!("\n"));
        ($($arg:tt)*) => ($crate::debug_print!("{}\n", format_args!($($arg)*)));
    }
}

#[repr(transparent)]
struct UnstableChar {
    inner: UnstableVectrix<u16>,
}

impl UnstableChar {
    fn new(value: u16) -> Self {
        UnstableChar {
            inner: UnstableVectrix::new(value)
        }
    }

    fn write(&mut self, value: u16) {
        self.inner.write(value)
    }

    fn read(&self) -> u16 {
        self.inner.read()
    }
}

#[repr(C)]
pub struct BootParams {
    pub kernel_load_addr: u32,
    pub kernel_size: u32,
    pub memory_map_addr: u32,
    pub memory_map_entries: u32,
}

#[repr(C, packed)]
#[derive(Copy, Clone)]
struct GdtEntry {
    limit_low: u16,
    base_low: u16,
    base_middle: u8,
    access: u8,
    granularity: u8,
    base_high: u8,
}

impl GdtEntry {
    const fn new_null() -> Self {
        GdtEntry {
            limit_low: 0,
            base_low: 0,
            base_middle: 0,
            access: 0,
            granularity: 0,
            base_high: 0,
        }
    }
}

#[repr(C, packed)]
struct GdtDescriptor {
    limit: u16,
    base: u32,
}

#[repr(C, packed)]
#[derive(Copy, Clone)]
struct IdtEntry {
    offset_low: u16,
    segment: u16,
    ist: u8,
    flags: u8,
    offset_mid: u16,
    offset_high: u32,
    reserved: u32,
}

impl IdtEntry {
    const fn new_empty() -> Self {
        IdtEntry {
            offset_low: 0,
            segment: 0,
            ist: 0,
            flags: 0,
            offset_mid: 0,
            offset_high: 0,
            reserved: 0,
        }
    }
}

#[repr(C, packed)]
struct IdtDescriptor {
    limit: u16,
    base: u32,
}

#[repr(C, align(4096))]
struct PageTable {
    entries: [u64; 512],
}

#[repr(C, align(4096))]
struct PageTables {
    pml4: PageTable,
    pdpt: PageTable,
    pd: PageTable,
}

#[repr(C, align(16))]
struct Stack {
    data: [u8; 4096 * 16],
}

static mut GDT: [GdtEntry; 5] = [
    GdtEntry::new_null(),
    GdtEntry {
        limit_low: 0,
        base_low: 0,
        base_middle: 0,
        access: 0x9A,
        granularity: 0x20,
        base_high: 0,
    },
GdtEntry {
    limit_low: 0,
    base_low: 0,
    base_middle: 0,
    access: 0x92,
    granularity: 0,
    base_high: 0,
},
GdtEntry {
    limit_low: 0,
    base_low: 0,
    base_middle: 0,
    access: 0xFA,
    granularity: 0x20,
    base_high: 0,
},
GdtEntry {
    limit_low: 0,
    base_low: 0,
    base_middle: 0,
    access: 0xF2,
    granularity: 0,
    base_high: 0,
},
];

static mut GDT_PTR: GdtDescriptor = GdtDescriptor {
    limit: (core::mem::size_of::<[GdtEntry; 5]>() - 1) as u16,
    base: 0,
};

static mut IDT: [IdtEntry; 256] = [IdtEntry::new_empty(); 256];

static mut IDT_PTR: IdtDescriptor = IdtDescriptor {
    limit: (core::mem::size_of::<[IdtEntry; 256]>() - 1) as u16,
    base: 0,
};

static mut PAGE_TABLES: PageTables = PageTables {
    pml4: PageTable { entries: [0; 512] },
    pdpt: PageTable { entries: [0; 512] },
    pd: PageTable { entries: [0; 512] },
};

static mut STACK: Stack = Stack {
    data: [0; 4096 * 16],
};

const STACK_SIZE: usize = 4096 * 16;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[repr(C)]
struct InterruptStackFrame {
    ip: u64,
    cs: u64,
    flags: u64,
    sp: u64,
    ss: u64,
}

unsafe fn setup_gdt() {
    GDT_PTR.base = &raw const GDT as *const _ as u32;

    core::arch::asm!(
        ".code32",
        "lgdt [{0:e}]",
        in(reg) &raw const GDT_PTR as *const _ as u32,
                     options(readonly),
    );

    core::arch::asm!(
        ".code32",
        "mov ax, 0x10",
        "mov ds, ax",
        "mov es, ax",
        "mov fs, ax",
        "mov gs, ax",
        "mov ss, ax",
    );
}

unsafe fn setup_idt() {
    let handler = timer_interrupt_handler as u64;
    IDT[0x20].offset_low = handler as u16;
    IDT[0x20].segment = 0x08;
    IDT[0x20].ist = 0;
    IDT[0x20].flags = 0x8E;
    IDT[0x20].offset_mid = (handler >> 16) as u16;
    IDT[0x20].offset_high = (handler >> 32) as u32;

    IDT_PTR.base = &raw const IDT as *const _ as u32;

    core::arch::asm!(
        ".code32",
        "lidt [{0:e}]",
        in(reg) &raw const IDT_PTR as *const _ as u32,
                     options(readonly),
    );
}

unsafe fn setup_page_tables() {
    debug_println!("spinUP: Setting up page tables...");

    // Verify we can access the memory where page tables will be
    if !verify_memory_access((&PAGE_TABLES as *const _) as u64, core::mem::size_of::<PageTables>()) {
        debug_println!("ERROR: Cannot access page table memory!");
        loop {}
    }

    // Initialize page tables with UnstableVectrix
    let pml4_addr = PhysAddr::new(&PAGE_TABLES.pml4 as *const _ as u64);
    let pdpt_addr = PhysAddr::new(&PAGE_TABLES.pdpt as *const _ as u64);
    let pd_addr = PhysAddr::new(&PAGE_TABLES.pd as *const _ as u64);

    let dimensions = Dimensions {
        width: 512,  // Standard x86_64 page table width
        height: 1,
        depth: 1,
    };

    // Create UnstableVectrix instances for each table
    let mut pml4 = UnstableVectrix::from_phys(pml4_addr, 512, 0, dimensions);
    let mut pdpt = UnstableVectrix::from_phys(pdpt_addr, 512, 0, dimensions);
    let mut pd = UnstableVectrix::from_phys(pd_addr, 512, 0, dimensions);

    // Clear tables first
    for i in 0..512 {
        pml4.write(i, 0, 0, 0);
        pdpt.write(i, 0, 0, 0);
        pd.write(i, 0, 0, 0);
    }

    // Set up identity mapping
    // PML4[0] -> PDPT
    pml4.write(0, 0, 0, pdpt_addr.as_u64() | 0x3);  // Present + Writable

    // PDPT[0] -> PD
    pdpt.write(0, 0, 0, pd_addr.as_u64() | 0x3);    // Present + Writable

    // PD entries - Identity map first 1GB with 2MB pages
    for i in 0..512 {
        // Each entry maps 2MB
        let addr = i as u64 * 0x200000;
        // Flags: Present + Writable + Huge Page (2MB)
        pd.write(i, 0, 0, addr | 0x83);
    }

    debug_println!("spinUP: Page tables initialized");
    debug_println!("  PML4 at {:016x}", pml4_addr.as_u64());
    debug_println!("  PDPT at {:016x}", pdpt_addr.as_u64());
    debug_println!("  PD at {:016x}", pd_addr.as_u64());

    // Load PML4 into CR3
    core::arch::asm!(
        "mov cr3, {0:r}",
        in(reg) pml4_addr.as_u64(),
                     options(nostack, preserves_flags)
    );

    debug_println!("spinUP: CR3 updated");

    // Verify the mapping
    let test_addr = VirtAddr::new(0x1000);
    if verify_memory_access(test_addr.as_u64(), 0x1000) {
        debug_println!("spinUP: Page tables verified");
    } else {
        debug_println!("ERROR: Page table verification failed!");
        loop {}
    }
}

// Helper function for page table flag combinations
const fn make_page_entry(addr: u64, present: bool, writable: bool, huge: bool, user: bool) -> u64 {
    let mut flags = 0u64;
    if present { flags |= 1; }
    if writable { flags |= 1 << 1; }
    if user { flags |= 1 << 2; }
    if huge { flags |= 1 << 7; }
    (addr & 0x000f_ffff_ffff_f000) | flags
}

unsafe fn verify_memory_access(addr: u64, size: usize) -> bool {
    let test_value = UnstableVectrix::<u8>::new(0x55);
    let ptr = addr as *mut u8;

    // Try to write and read back a test value
    test_value.write(0x55);
    ptr.write_volatile(0x55);
    let read_back = ptr.read_volatile();

    read_back == 0x55
}

unsafe fn setup_pic() {
    core::arch::asm!(
        ".code32",
        "mov al, 0x11",
        "out 0x20, al",
        "out 0xA0, al",

        "mov al, 0x20",
        "out 0x21, al",
        "mov al, 0x28",
        "out 0xA1, al",

        "mov al, 0x04",
        "out 0x21, al",
        "mov al, 0x02",
        "out 0xA1, al",

        "mov al, 0x01",
        "out 0x21, al",
        "out 0xA1, al",

        "mov al, 0x00",
        "out 0x21, al",
        "out 0xA1, al",
    );
}

#[naked]
unsafe extern "x86-interrupt" fn timer_interrupt_handler(_stack_frame: InterruptStackFrame) {
    core::arch::naked_asm!(
        ".code32",
        "push eax",
        "mov al, 0x20",
        "out 0x20, al",
        "pop eax",
        "iret"
    );
}

#[no_mangle]
pub unsafe extern "C" fn _start() -> ! {
    core::arch::asm!(".code32", "cli");

    setup_gdt();
    setup_page_tables();

    core::arch::asm!(
        ".code32",
        "mov eax, cr4",
        "or eax, 0x20",
        "mov cr4, eax",
    );

    core::arch::asm!(
        ".code32",
        "mov ecx, 0xC0000080",
        "rdmsr",
        "or eax, 0x100",
        "wrmsr",
    );

    core::arch::asm!(
        ".code32",
        "mov eax, cr0",
        "or eax, 0x80000001",
        "mov cr0, eax",
    );

    core::arch::asm!(
        ".code32",
        "push {0}",
        "push {1}",
        "retf",
        ".code64",
        "mov ax, 0x10",
        "mov ds, ax",
        "mov es, ax",
        "mov fs, ax",
        "mov gs, ax",
        "mov ss, ax",
        "mov rsp, {2}",
        "mov rbp, rsp",
        "call {3}",
        "call {4}",
        "sti",
        "jmp {5}",
        const 0x08,
        sym long_mode_start,
        in(reg) (&raw const STACK.data as *const _ as u64) + (STACK_SIZE as u64),
                     sym setup_idt,
                     sym setup_pic,
                     sym rust_main,
                     options(noreturn),
    );
}

unsafe fn read_disk_sector(sector: u16, buffer: *mut u8) {
    let mut disk_packet = [0u8; 16];
    // Disk Address Packet structure
    disk_packet[0] = 16;    // Size of packet
    disk_packet[1] = 0;     // Reserved
    disk_packet[2] = 1;     // Number of sectors to read
    disk_packet[3] = 0;     // Reserved
    disk_packet[4..8].copy_from_slice(&(buffer as u32).to_le_bytes());  // Buffer address
    disk_packet[8..12].copy_from_slice(&(sector as u32).to_le_bytes()); // LBA (low)
    disk_packet[12..16].fill(0);  // LBA (high)

    core::arch::asm!(
        ".code32",
        "mov ah, 0x42",
        "mov dl, 0x00",      // Drive number (0x00 for floppy)
    "int 0x13",
    in("si") disk_packet.as_ptr(),
                     options(preserves_flags)
    );
}

// load the kernel from disk
unsafe fn load_kernel() {
    // Read kernel from disk using BIOS int 13h
    let mut buffer = KERNEL_LOAD_ADDR as *mut u8;

    for sector in 0..SECTORS_TO_READ {
        let lba = KERNEL_SECTOR_START + sector;

        // Use int 13h to read the sector
        core::arch::asm!(
            ".code32",
            "push eax",
            "push ebx",
            "push ecx",
            "push edx",
            "push esi",
            "push edi",

            // Set up disk read
            "mov ah, 0x42",
            "mov dl, 0x00",  // Drive number (0x00 for floppy)
        "mov si, sp",
        "sub sp, 16",    // Make space for disk address packet

        // Build disk address packet
        "mov byte ptr [si-16], 16",   // Size of packet
        "mov byte ptr [si-15], 0",    // Reserved
        "mov word ptr [si-14], 1",    // Number of sectors
        "mov dword ptr [si-12], {0}", // Buffer address
        "mov dword ptr [si-8], {1}",  // LBA low
        "mov dword ptr [si-4], 0",    // LBA high

        "int 0x13",

        "add sp, 16",
        "pop edi",
        "pop esi",
        "pop edx",
        "pop ecx",
        "pop ebx",
        "pop eax",
        in(reg) buffer as u32,
                         in(reg) lba as u32,
                         options(nostack),
        );

        buffer = buffer.add(512);
    }
}

unsafe fn print(s: &str) {
    for byte in s.bytes() {
        let char_ptr = (VGA_BUFFER as *mut u16).add(VGA_CURSOR);
        let vga_char = UnstableChar::new(0);
        vga_char.write((0x0F << 8) | byte as u16); // White on black
        VGA_CURSOR += 1;
    }
}

unsafe fn println(s: &str) {
    print(s);
    VGA_CURSOR = (VGA_CURSOR + 80 - (VGA_CURSOR % 80)); // New line
}

#[no_mangle]
unsafe extern "C" fn long_mode_start() -> ! {
    rust_main()
}

#[no_mangle]
pub extern "C" fn rust_main() -> ! {
    unsafe {

        debug_println!("spinUP: Gear 2 started at {:016x}", BOOT_START);
        debug_println!("spinUP: Stack at {:016x}-{:016x}",
                       BOOT_STACK_START,
                       BOOT_STACK_START + BOOT_STACK_SIZE
        );
        // ... rest of the code
        println("spinUP: Gear 2 started");
        println("spinUP: Loading kernel...");

        // Load kernel
        let mut buffer = KERNEL_LOAD_ADDR as *mut u8;
        for sector in 0..SECTORS_TO_READ {
            read_disk_sector(KERNEL_SECTOR_START + sector, buffer);
            buffer = buffer.add(512);
            if sector % 10 == 0 {
                print(".");  // Progress indicator
            }
        }
        println("\nspinUP: Kernel loaded");

        // Set up argument structure for kernel
        let boot_params = BootParams {
            kernel_load_addr: KERNEL_LOAD_ADDR as u32,
            kernel_size: (SECTORS_TO_READ as u32) * 512,
            memory_map_addr: &PAGE_TABLES as *const _ as u32,
            memory_map_entries: 3, // PML4, PDPT, PD
        };

        println("spinUP: Jumping to kernel");

        // Jump to kernel
        let kernel_entry = KERNEL_LOAD_ADDR as *const fn() -> !;
        (*kernel_entry)();
    }

    loop {
        unsafe {
            core::arch::asm!("hlt");
        }
    }
}
