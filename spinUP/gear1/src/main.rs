#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[repr(C, packed)]
struct Dap {
    sz: u8,
    _pad: u8,
    cnt: u16,
    off: u16,
    seg: u16,
    lba: u32,
    _pad2: u32,
}

#[repr(C, packed)]
pub struct StageInfo {
    boot_drive: u8,
    memory_map_addr: u32,
    memory_entries: u16,
    stage2_load_addr: u32,
    flags: u32,
}

#[no_mangle]
static mut STAGE_INFO: StageInfo = StageInfo {
    boot_drive: 0,
    memory_map_addr: 0,
    memory_entries: 0,
    stage2_load_addr: 0x7E00,
    flags: 0,
};

// Buffer for memory map (placed at a known location)
#[no_mangle]
static mut MEMORY_MAP_BUFFER: [u8; 2048] = [0; 2048];

#[no_mangle]
#[link_section = ".boot.text"]
pub extern "C" fn _start() -> ! {
    unsafe {
        // Setup segments
        core::arch::asm!(
            "xor ax, ax",
            "mov ds, ax",
            "mov ss, ax",
            "mov sp, 0x7C00",
        );

        // Store boot drive in STAGE_INFO
        core::arch::asm!("mov {}, dl", out(reg_byte) STAGE_INFO.boot_drive);

        // Enable A20 line
        enable_a20();

        // Update STAGE_INFO with memory map information
        let buffer_ptr = (&raw mut MEMORY_MAP_BUFFER) as *mut [u8; 2048];
        STAGE_INFO.memory_map_addr = buffer_ptr as u32;
        STAGE_INFO.memory_entries = detect_memory(buffer_ptr) as u16;

        // Load gear2
        let dap = Dap {
            sz: 16,
            _pad: 0,
            cnt: 32,
            off: 0,
            seg: 0x07E0,
            lba: 1,
            _pad2: 0,
        };

        core::arch::asm!(
            "mov ah, 0x42",
            "mov si, {0:x}",
            "int 0x13",
            "jc 2f",         // Using numeric labels
            "cmp ah, 0",
            "jne 2f",
            "push word ptr 0x07E0",
            "push word ptr 0",
            "mov dl, {1}",
            "retf",
            "2:",           // Error handler using numeric label
            "mov ax, 0x0E45",
            "int 0x10",
            "hlt",
            in(reg) &dap,
                         in(reg_byte) STAGE_INFO.boot_drive,
                         options(noreturn),
        );
    }
}

unsafe fn enable_a20() {
    core::arch::asm!(
        "mov ax, 0x2401",
        "int 0x15",
        "jc 2f",         // Using numeric labels
        "ret",
        "2:",           // Using numeric label >= 2
        "in al, 0x92",
        "or al, 2",
        "out 0x92, al",
        options(nomem, nostack)
    );
}

unsafe fn detect_memory(buffer: *mut [u8; 2048]) -> u32 {
    let mut entries = 0;
    let continuation_id = 0;

    loop {
        let mut result: u32;
        core::arch::asm!(
            "int 0x15",
            "jc 2f",          // Using numeric labels
            "mov eax, 1",
            "jmp 3f",
            "2:",             // Using numeric label >= 2
            "mov eax, 0",
            "3:",             // Using numeric label >= 2
            inout("eax") 0xE820 => result,
                         in("ebx") continuation_id,
                         in("ecx") 24,
                         in("edx") 0x534D4150,
                         in("di") buffer as *mut u8,
                         options(nostack)
        );

        if result == 0 { break; }
        entries += 1;
        if continuation_id == 0 { break; }
    }
    entries
}

#[panic_handler]
#[no_mangle]
#[link_section = ".boot.text.panic"]
fn panic(_: &PanicInfo) -> ! { loop {} }
