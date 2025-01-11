#![no_std]
#![no_main]

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! { loop {} }

#[link_section = ".boot.text"]
#[no_mangle]
pub extern "C" fn _start() -> ! {
    unsafe {
        core::arch::asm!(
            "xor ax, ax",
            "mov ds, ax",
            "mov ss, ax",
            "mov sp, 0x7C00",
            "mov byte ptr [0x7E00], dl",
            "in al, 0x92",
            "or al, 2",
            "out 0x92, al",
            "mov si, dap",
            "mov ah, 0x42",
            "int 0x13",
            "jc error",
            "push word ptr 0x07E0",
            "push word ptr 0",
            "retf",
            "error:",
            "mov ax, 0x0E45",
            "int 0x10",
            "cli",
            "hlt",
            dap:
            .byte 16, 0",
            .word 16, 0, 0x07E0",
            .long 1, 0",
            options(noreturn)
        );
    }
    }
