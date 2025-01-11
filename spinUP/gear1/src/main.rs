#![no_std]
#![no_main]

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! { loop {} }

#[no_mangle]
#[link_section = ".boot.text"]
pub extern "C" fn _start() -> ! {
    unsafe {
        core::arch::asm!(
            // Setup stack
            "xor ax, ax",
            "mov ds, ax",
            "mov ss, ax",
            "mov sp, 0x7C00",

            // Save boot drive
            "mov byte ptr [0x7E00], dl",

            // Enable A20
            "in al, 0x92",
            "or al, 2",
            "out 0x92, al",

            // Build DAP on stack
            "push word ptr 0",         // Upper _pad2
            "push word ptr 0",         // Lower _pad2
            "push word ptr 0",         // Upper LBA
            "push word ptr 1",         // Lower LBA = 1
            "push word ptr 0x07E0",    // segment
            "push word ptr 0",         // offset
            "push word ptr 16",        // count
            "push word ptr 0x10",      // size=16
            "mov si, sp",              // SI points to DAP

            // Load sectors
            "mov ah, 0x42",
            "int 0x13",
            "add sp, 16",              // Clean stack
            "jc 2f",                   // Jump if error

            // Jump to gear2
            "push word ptr 0x07E0",    // segment
            "push word ptr 0",         // offset
            "retf",                    // Far return

            // Error: print 'E' and halt
            "2:",
            "mov ax, 0x0E45",
            "int 0x10",
            "cli",
            "hlt",

            options(noreturn)
        );
    }
}
