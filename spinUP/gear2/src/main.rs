#![no_std]
#![no_main]

use core::panic::PanicInfo;
mod serial;
use serial::SerialPort;
use unstable_matter::UnstableMatter;

// ... (keep all struct definitions as is) ...

// Add back PAGE_TABLES static
static mut PAGE_TABLES: PageTables = PageTables {
    pml4: PageTable { entries: [0; 512] },
    pdpt: PageTable { entries: [0; 512] },
    pd: PageTable { entries: [0; 512] },
};

// ... (keep GDT, STACK, and SERIAL statics as is) ...

#[no_mangle]
pub extern "C" fn _start() -> ! {
    unsafe {
        let serial = UnstableMatter::at(&raw const SERIAL as *const _ as usize);
        serial.init();

        for &b in b"Gear2 starting...\r\n" {
            serial.write_byte(b);
        }

        enter_long_mode();
    }
}

// ... (keep enter_long_mode and disable_interrupts as is) ...

unsafe fn setup_page_tables() {
    let pml4 = UnstableMatter::at(&raw const PAGE_TABLES.pml4 as *const _ as usize);
    let pdpt = UnstableMatter::at(&raw const PAGE_TABLES.pdpt as *const _ as usize);
    let pd = UnstableMatter::at(&raw const PAGE_TABLES.pd as *const _ as usize);

    // Use UnstableMatter for entries
    let pml4_entries = UnstableMatter::at(&mut PAGE_TABLES.pml4.entries[0] as *mut _ as usize);
    let pdpt_entries = UnstableMatter::at(&mut PAGE_TABLES.pdpt.entries[0] as *mut _ as usize);
    let pd_entries = UnstableMatter::at(&mut PAGE_TABLES.pd.entries[0] as *mut _ as usize);

    pml4_entries.write(pdpt.addr() as u64 | 0x3);
    pdpt_entries.write(pd.addr() as u64 | 0x3);
    pd_entries.write(0x83);  // Present + Write + Huge (2MB)
}

unsafe fn setup_gdt() {
    let gdt = UnstableMatter::at(&raw const GDT.entries as *const _ as usize);
    let gdt_ptr = GDTPointer {
        limit: (core::mem::size_of::<[GDTEntry; 3]>() - 1) as u16,
        base: gdt.addr() as u32,
    };
    core::arch::asm!("lgdt ({0})", in(reg) &gdt_ptr);
}

unsafe fn enable_paging() {
    let pml4 = UnstableMatter::at(&raw const PAGE_TABLES.pml4 as *const _ as usize);

    core::arch::asm!(
        "mov %cr4, %eax",
        "bts $5, %eax",
        "mov %eax, %cr4",

        "mov {0}, %eax",
        "mov %eax, %cr3",

        "mov $0xC0000080, %ecx",
        "rdmsr",
        "bts $8, %eax",
        "wrmsr",

        "mov %cr0, %eax",
        "bts $31, %eax",
        "bts $0, %eax",
        "mov %eax, %cr0",
        in(reg) pml4.addr(),
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

        // Jump to rust_main
        "call {1}",

        "hlt",
        in(reg) stack_top,
                     sym rust_main,
                     options(noreturn),
    );
}

#[no_mangle]
pub extern "C" fn rust_main() -> ! {
    let vga = unsafe { UnstableMatter::<u16>::at(0xB8000) };
    let msg = b"Long Mode OK!";

    // Clear screen
    for i in 0..80*25 {
        vga.write(0x0F00);
    }

    // Write message
    for (i, &byte) in msg.iter().enumerate() {
        vga.write(0x0F00 | byte as u16);
    }

    loop {
        core::arch::asm!("hlt", options(nomem, nostack));
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    unsafe {
        let serial = UnstableMatter::at(&raw const SERIAL as *const _ as usize);
        for &b in b"PANIC in gear2!\r\n" {
            serial.write_byte(b);
        }
    }
    loop {}
}
