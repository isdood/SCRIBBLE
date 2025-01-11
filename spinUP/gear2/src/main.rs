#![no_std]
#![no_main]

use core::panic::PanicInfo;
mod serial;
use serial::SerialPort;
use unstable_matter::UnstableMatter;

#[repr(C)]  // Remove packed from PageTable since it needs to be aligned
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

unsafe fn setup_page_tables() {
    let pml4 = UnstableMatter::at(&raw const PAGE_TABLES.pml4 as *const _ as usize);
    let pdpt = UnstableMatter::at(&raw const PAGE_TABLES.pdpt as *const _ as usize);
    let pd = UnstableMatter::at(&raw const PAGE_TABLES.pd as *const _ as usize);

    // Identity map first 2MB
    let entries = &mut PAGE_TABLES.pml4.entries;
    entries[0] = pdpt.addr() as u64 | 0x3;

    let entries = &mut PAGE_TABLES.pdpt.entries;
    entries[0] = pd.addr() as u64 | 0x3;

    let entries = &mut PAGE_TABLES.pd.entries;
    entries[0] = 0x83;  // Present + Write + Huge (2MB)
}

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

static mut SERIAL: SerialPort = unsafe { SerialPort::new(0x3F8) };

#[no_mangle]
pub extern "C" fn _start() -> ! {
    unsafe {
        SERIAL.init();
        for byte in b"Gear2 starting...\r\n" {
            SERIAL.write_byte(*byte);
        }
        enter_long_mode();
    }
}

unsafe fn enter_long_mode() -> ! {
    disable_interrupts();
    for byte in b"Disabled interrupts\r\n" {
        SERIAL.write_byte(*byte);
    }

    setup_page_tables();
    for byte in b"Page tables setup\r\n" {
        SERIAL.write_byte(*byte);
    }

    setup_gdt();
    for byte in b"GDT setup\r\n" {
        SERIAL.write_byte(*byte);
    }

    enable_paging();
    for byte in b"Paging enabled\r\n" {
        SERIAL.write_byte(*byte);
    }

    for byte in b"Jumping to long mode...\r\n" {
        SERIAL.write_byte(*byte);
    }

    jump_to_long_mode()
}

unsafe fn disable_interrupts() {
    core::arch::asm!("cli");
}

unsafe fn setup_page_tables() {
    // Use raw pointers to avoid alignment issues
    let pml4_ptr = (&mut PAGE_TABLES.pml4.entries[0] as *mut u64);
    let pdpt_ptr = (&mut PAGE_TABLES.pdpt.entries[0] as *mut u64);
    let pd_ptr = (&mut PAGE_TABLES.pd.entries[0] as *mut u64);

    // Set up page tables using raw pointers
    pml4_ptr.write_volatile((&PAGE_TABLES.pdpt as *const PageTable as u64) | 0x3);
    pdpt_ptr.write_volatile((&PAGE_TABLES.pd as *const PageTable as u64) | 0x3);
    pd_ptr.write_volatile(0x83); // Present + Write + Huge (2MB)
}

unsafe fn setup_gdt() {
    let gdt_ptr = GDTPointer {
        limit: (core::mem::size_of::<[GDTEntry; 3]>() - 1) as u16,
        base: &GDT as *const GDTTable as u32,
    };
    core::arch::asm!("lgdtl ({0:e})", in(reg) &gdt_ptr);
}

unsafe fn enable_paging() {
    let pml4_addr = &PAGE_TABLES.pml4 as *const PageTable as u32;

    core::arch::asm!(
        "movl %cr4, %eax",
        "btsl $5, %eax",
        "movl %eax, %cr4",

        "movl {0:e}, %eax",
        "movl %eax, %cr3",

        "movl $0xC0000080, %ecx",
        "rdmsr",
        "btsl $8, %eax",
        "wrmsr",

        "movl %cr0, %eax",
        "btsl $31, %eax",
        "btsl $0, %eax",
        "movl %eax, %cr0",
        in(reg) pml4_addr,
    );
}

unsafe fn jump_to_long_mode() -> ! {
    let stack_top = &raw const STACK.data as *const u8 as u64 + 4096;

    core::arch::asm!(
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

        "movq $0x3F8, %rdx",
        "movb $'L', %al",
        "outb %al, %dx",

        "hlt",
        in(reg) stack_top,
                     options(noreturn),
    );
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    unsafe {
        for byte in b"PANIC in gear2!\r\n" {
            SERIAL.write_byte(*byte);
        }
    }
    loop {}
}
