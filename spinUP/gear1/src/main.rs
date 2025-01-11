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

        // Store boot drive
        let drive: u8;
        core::arch::asm!("mov {}, dl", out(reg_byte) drive);

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
            "jc 2f",         // Jump to error handler if carry set
            "cmp ah, 0",     // Check status
            "jne 2f",        // Jump if not zero (error)
        "push word ptr 0x07E0",
        "push word ptr 0",
        "mov dl, {1}",
        "retf",
        "2:",           // Error handler (using numeric label)
        "mov ax, 0x0E45",  // Print 'E' on error
        "int 0x10",
        "hlt",
        in(reg) &dap,
                         in(reg_byte) drive,
                         options(noreturn),
        );
    }
}

// Enable A20 line
unsafe fn enable_a20() {
    // BIOS method
    core::arch::asm!(
        "mov ax, 0x2401",
        "int 0x15",
        "jc 1f",          // If failed, try alternate method
        "ret",
        "1:",
        // Keyboard controller method as fallback
        "in al, 0x92",
        "or al, 2",
        "out 0x92, al",
    );
}

// Proper memory detection
unsafe fn detect_memory(buffer: &mut [u8]) -> u32 {
    let mut entries = 0;
    let continuation_id = 0; // Removed mut as it wasn't needed

    loop {
        let mut result: u32;
        core::arch::asm!(
            "int 0x15",
            "jc 1f",
            "mov eax, 1",  // Direct register reference instead of template
            "jmp 2f",
            "1:",
            "mov eax, 0",  // Direct register reference instead of template
            "2:",
            inout("eax") 0xE820 => result,
                         in("ebx") continuation_id,
                         in("ecx") 24,
                         in("edx") 0x534D4150,  // 'SMAP'
        in("di") buffer.as_mut_ptr(),
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
