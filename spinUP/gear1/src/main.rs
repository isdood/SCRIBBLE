#![no_std]
#![no_main]

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! { loop {} }

#[link_section = ".boot.text"]
#[no_mangle]
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

            // Build DAP
            "sub sp, 16",         // Reserve space for DAP
            "mov si, sp",         // SI points to DAP
            "mov byte ptr [si], 16", // size=16
            "mov byte ptr [si + 1], 0", // pad=0
            "mov word ptr [si + 2], 16", // count=16
            "mov word ptr [si + 4], 0", // offset=0
            "mov word ptr [si + 6], 0x07E0", // segment
            "mov dword ptr [si + 8], 1", // LBA=1
            "mov dword ptr [si + 12], 0", // pad2=0

            // Load sectors
            "mov ah, 0x42",
            "int 0x13",
            "add sp, 16",         // Clean stack
            "jc 2f",             // Jump if error

            // Jump to gear2
            "push word ptr 0x07E0", // segment
            "push word ptr 0",    // offset
            "retf",              // Far return

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
