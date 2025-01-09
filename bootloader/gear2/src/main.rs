#[repr(align(16))]
#[repr(C)]
pub struct Registers {
    eax: u32,
    ebx: u32,
    ecx: u32,
    edx: u32,    // Boot drive number passed from Gear 1
    esi: u32,
    edi: u32,
    ebp: u32,
    esp: u32,
}

static mut REGISTERS: Registers = Registers {
    eax: 0, ebx: 0, ecx: 0, edx: 0,
    esi: 0, edi: 0, ebp: 0, esp: 0,
};

#[no_mangle]
#[link_section = ".start"]
pub extern "C" fn _start() -> ! {
    // Initial assembly stub to save registers
    unsafe {
        core::arch::asm!(
            // Save all registers
            "mov [{0} + 0], eax",
            "mov [{0} + 4], ebx",
            "mov [{0} + 8], ecx",
            "mov [{0} + 12], edx",  // Contains boot drive
            "mov [{0} + 16], esi",
            "mov [{0} + 20], edi",
            "mov [{0} + 24], ebp",
            "mov [{0} + 28], esp",
            // Set up stack
            "mov esp, 0xD000",
            // Jump to Rust code
            "call {1}",
            sym REGISTERS,
            sym real_start,
            options(noreturn),
        );
    }
    unreachable!()
}

#[no_mangle]
pub extern "C" fn real_start() -> ! {
    // First, write directly to VGA buffer as a sanity check
    unsafe {
        let vga = 0xB8000 as *mut u16;
        *vga = 0x0F47;       // White 'G' on black
        *(vga.add(1)) = 0x0F32; // White '2' on black
    }

    // Initialize serial port for debugging
    let mut serial_port = unsafe { SerialPort::new(0x3F8) };
    serial_port.init();

    write_serial(&mut serial_port, b"[spinUP! Gear 2] Starting...\n");

    // Get boot drive number passed from Gear 1
    let boot_drive = unsafe { REGISTERS.edx & 0xFF };
    write_serial(&mut serial_port, b"[spinUP! Gear 2] Boot drive: ");
    write_serial(&mut serial_port, &[b'0' + (boot_drive / 10) as u8, b'0' + (boot_drive % 10) as u8, b'\n']);

    if let Err(()) = enable_a20() {
        write_serial(&mut serial_port, b"[spinUP! Gear 2] Failed to enable A20 line\n");
        halt();
    }

    init_page_tables();

    let boot_params = unsafe {
        &*(&boot_params::BOOT_PARAMS as *const boot_params::BootParams)
    };

    init_gdt();
    init_idt();

    match load_kernel(boot_params.kernel_load_addr, boot_params.kernel_size) {
        Ok(entry_point) => {
            write_serial(&mut serial_port, b"[spinUP! Gear 2] Kernel loaded successfully. Shifting into high gear...\n");
            jump_to_kernel(entry_point);
        }
        Err(_) => {
            write_serial(&mut serial_port, b"[spinUP! Gear 2] Failed to load kernel\n");
            halt();
        }
    }

    halt();
}

fn halt() -> ! {
    unsafe {
        core::arch::asm!("cli; hlt", options(noreturn));
    }
}
