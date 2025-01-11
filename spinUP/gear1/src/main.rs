#![no_std]
#![no_main]

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! { loop {} }

#[link_section = ".boot.text"]
#[no_mangle]
pub extern "C" fn _start() -> ! {
    unsafe {
        core::arch::asm!(
            // Setup
            "xor ax, ax",
            "mov ds, ax",
            "mov ss, ax",
            "mov sp, 0x7C00",
            "mov byte ptr [0x7E00], dl",

            // Enable A20
            "in al, 0x92",
            "or al, 2",
            "out 0x92, al",

            // Load sectors using DAP
            "mov si, 2f",          // Point to DAP
            "mov ah, 0x42",
            "int 0x13",
            "jc 1f",              // Jump to error if carry set

            // Jump to gear2
            "push word ptr 0x07E0",
            "push word ptr 0",
            "retf",

            // Error handler
            "1:",
            "mov ax, 0x0E45",     // Print 'E'
        "int 0x10",
        "cli",
        "hlt",

        // Disk Address Packet (DAP)
        "2:",
        ".byte 16",           // size of DAP
        ".byte 0",            // unused
        ".word 16",           // sectors to read
        ".word 0",            // offset
        ".word 0x07E0",       // segment
        ".quad 1",            // LBA
        options(noreturn)
        );
    }
}
