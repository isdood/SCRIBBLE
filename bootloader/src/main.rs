#![no_std]
#![no_main]
#![feature(naked_functions)]
#![feature(asm_experimental_arch)]

use core::panic::PanicInfo;

// Memory constants
const BOOT_STACK: u32 = 0x7000;
const PAGE_PRESENT: u64 = 1 << 0;
const PAGE_WRITABLE: u64 = 1 << 1;
const PAGE_HUGE: u64 = 1 << 7;

// Page table addresses
const PML4_TABLE: u32 = 0x1000;
const PDPT_TABLE: u32 = 0x2000;
const PD_TABLE: u32 = 0x3000;

#[repr(C, align(4096))]
struct PageTables {
    pml4: [u64; 512],
    pdpt: [u64; 512],
    pd: [u64; 512],
}

#[repr(C, align(16))]
struct GDTEntry {
    limit_low: u16,
    base_low: u16,
    base_middle: u8,
    access: u8,
    granularity: u8,
    base_high: u8,
}

#[repr(C, packed)]
struct GDTDescriptor {
    size: u16,
    offset: u64,
}

#[naked]
#[no_mangle]
#[link_section = ".boot.text"]
unsafe extern "C" fn _start() -> ! {
    core::arch::naked_asm!(
        ".code32",
        "cli",
        "cld",
        "movl ${}, %esp",
        "movl %esp, %ebp",
        "jmp {}",
        const BOOT_STACK,
        sym setup_long_mode,
        options(att_syntax)
    )
}

#[no_mangle]
#[link_section = ".text.boot"]
unsafe fn setup_long_mode() -> ! {
    // Enable PAE
    core::arch::asm!(
        ".code32",
        "movl %cr4, %eax",
        "orl $(1 << 5), %eax",
                     "movl %eax, %cr4",
                     options(att_syntax)
    );

    setup_paging();

    // Enable long mode and paging
    core::arch::asm!(
        ".code32",
        "movl $0xC0000080, %ecx",
        "rdmsr",
        "orl $(1 << 8), %eax",
                     "wrmsr",
                     "movl %cr0, %eax",
                     "orl $(1 << 31 | 1), %eax",
                     "movl %eax, %cr0",
                     "lgdtl ({0})",
                     "pushl $0x08",
                     // Fixed far jump sequence
                     "call 1f",
                     "1:",
                     "popl %eax",
                     "addl $long_mode_start - 1b, %eax",
                     "pushl %eax",
                     "lret",
                     sym GDT_DESCRIPTOR,
                     options(att_syntax)
    );

    loop {}
}

unsafe fn setup_paging() {
    let tables = &mut *(PML4_TABLE as *mut PageTables);

    tables.pml4.fill(0);
    tables.pdpt.fill(0);
    tables.pd.fill(0);

    tables.pml4[0] = PDPT_TABLE as u64 | PAGE_PRESENT | PAGE_WRITABLE;
    tables.pdpt[0] = PD_TABLE as u64 | PAGE_PRESENT | PAGE_WRITABLE;
    tables.pd[0] = PAGE_PRESENT | PAGE_WRITABLE | PAGE_HUGE;

    core::arch::asm!(
        ".code32",
        "movl {0:e}, %cr3",  // Using :e for 32-bit register
        in(reg) PML4_TABLE,
                     options(att_syntax)
    );
}

#[link_section = ".text.long_mode"]
#[no_mangle]
extern "C" fn long_mode_start() -> ! {
    unsafe {
        core::arch::asm!(
            ".code64",
            "movw $0x10, %ax",
            "movw %ax, %ds",
            "movw %ax, %es",
            "movw %ax, %fs",
            "movw %ax, %gs",
            "movw %ax, %ss",
            "xorq %rax, %rax",
            "xorq %rbx, %rbx",
            "xorq %rcx, %rcx",
            "xorq %rdx, %rdx",
            "xorq %rsi, %rsi",
            "xorq %rdi, %rdi",
            "xorq %rbp, %rbp",
            "xorq %r8, %r8",
            "xorq %r9, %r9",
            "xorq %r10, %r10",
            "xorq %r11, %r11",
            "xorq %r12, %r12",
            "xorq %r13, %r13",
            "xorq %r14, %r14",
            "xorq %r15, %r15",
            options(att_syntax)
        );
    }

    init_long_mode();

    loop {
        unsafe { core::arch::asm!("hlt") }
    }
}

fn init_long_mode() {
    // Will be implemented later
}

#[used]
#[link_section = ".rodata.gdt"]
static GDT: [GDTEntry; 3] = [
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
];

#[used]
#[link_section = ".rodata.gdt_descriptor"]
static GDT_DESCRIPTOR: GDTDescriptor = GDTDescriptor {
    size: (core::mem::size_of::<[GDTEntry; 3]>() - 1) as u16,
    offset: 0, // Will be fixed by linker
};

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
