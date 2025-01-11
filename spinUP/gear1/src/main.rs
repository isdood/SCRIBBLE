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
#[link_section = ".boot.text"]
pub extern "C" fn _start() -> ! {
    let dap = Dap {
        sz: 16,
        _pad: 0,
        cnt: 16,        // Reduce from 32 to 16 sectors (8KB)
        off: 0,
        seg: 0x07E0,    // Loading to 0x7E00
        lba: 1,
        _pad2: 0,
    };

    unsafe {
        core::arch::asm!(
            // Initialize segments
            "xor ax, ax",
            "mov ds, ax",
            "mov es, ax",
            "mov ss, ax",
            "mov sp, 0x7C00",

            // Store boot drive
            "mov byte ptr [0x7E00], dl",

            // Enable A20
            "in al, 0x92",
            "or al, 2",
            "out 0x92, al",

            // Load sectors
            "mov si, {0:x}",
            "mov ah, 0x42",
            "int 0x13",
            "jc 2f",         // Jump if carry flag set (error)
        "test ah, ah",   // Check status
        "jnz 2f",        // Jump if not zero (error)

        // Set up segment registers for gear2
        "xor ax, ax",    // Clear AX
        "mov ds, ax",    // Set DS to 0
        "mov es, ax",    // Set ES to 0
        "mov fs, ax",    // Set FS to 0
        "mov gs, ax",    // Set GS to 0
        "mov ss, ax",    // Set SS to 0

        // Jump to gear2
        "mov ax, 0x07E0",
        "push ax",       // Push segment
        "xor ax, ax",
        "push ax",       // Push offset
        "retf",          // Far return to gear2

        "2:",           // Error handler
        "mov al, 'E'",
        "mov ah, 0x0E",
        "int 0x10",
        "jmp 2b",      // Loop on error
        in(reg) &dap,
                         options(noreturn)
        );
    }
}
