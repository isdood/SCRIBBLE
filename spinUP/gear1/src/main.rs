#![no_std]
#![no_main]

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! { loop {} }

#[no_mangle]
#[link_section = ".boot.text"]
pub extern "C" fn _start() -> ! {
    unsafe {
        core::arch::asm!(
            // Initialize segments and stack
            "xor ax, ax",
            "mov ds, ax",
            "mov es, ax",
            "mov ss, ax",
            "mov sp, 0x7C00",

            // Save boot drive
            "mov [0x7E00], dl",

            // Enable A20
            "in al, 0x92",
            "or al, 2",
            "out 0x92, al",

            // Set up disk read
            "push ax",         // Reserve space for DAP
            "push ax",
            "push ax",
            "push ax",
            "mov si, sp",      // SI points to DAP
            "mov byte ptr [si], 16",     // DAP size
            "mov byte ptr [si + 1], 0",  // Padding
            "mov word ptr [si + 2], 16", // Sector count
            "mov word ptr [si + 4], 0",  // Offset
            "mov word ptr [si + 6], 0x07E0", // Segment
            "mov dword ptr [si + 8], 1",  // LBA

            // Read sectors
            "mov ah, 0x42",
            "int 0x13",
            "add sp, 8",       // Clean up DAP
            "jc 2f",          // Jump if error

            // Jump to gear2
            "push word ptr 0x07E0",
            "push word ptr 0",
            "retf",           // Far return to gear2

            // Error handler
            "2:",
            "mov ax, 0x0E45", // Print 'E'
        "int 0x10",
        "cli",
        "hlt",

        options(noreturn)
        );
    }
}
