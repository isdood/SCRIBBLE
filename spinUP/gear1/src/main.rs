#![no_std]
#![no_main]

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! { loop {} }

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

#[no_mangle]
pub unsafe extern "C" fn _start() -> ! {
    // Setup stack
    core::arch::asm!(
        "mov sp, 0x7c00",
        options(nomem, nostack)
    );

    // Reset segments
    core::arch::asm!(
        "xor ax, ax",
        "mov ds, ax",
        "mov es, ax",
        "mov ss, ax",
        options(nomem, nostack)
    );

    // Load Gear2
    let drive = 0x80;  // Boot drive
    let sectors = 64;  // Number of sectors to load
    let buffer = 0x7e00 as *mut u8;  // Load address

    core::arch::asm!(
        "mov ah, 0x42",      // Extended read
        "int 0x13",
        "jc 1f",             // If error, fail
        "cmp ah, 0",         // Check status
        "jne 1f",            // If not success, fail
        "jmp 2f",            // Success, continue
        "1: mov al, 0x45",   // Error code
        "mov ah, 0x0e",      // Print char
        "int 0x10",
        "cli",               // Halt on error
        "hlt",
        "2:",               // Success
        in("dl") drive,
                     options(nomem, nostack)
    );

    // Jump to Gear2
    core::arch::asm!(
        "push word ptr 0x0000",  // CS
        "push word ptr 0x7e00",  // IP
        "retf",                  // Far return to Gear2
        options(nomem, nostack)
    );

    loop {}
}
