#[repr(packed)]
struct GDTEntry {
    limit_low: u16,
    base_low: u16,
    base_middle: u8,
    access: u8,
    granularity: u8,
    base_high: u8,
}

#[repr(packed)]
struct GDTDescriptor {
    size: u16,
    offset: u32,
}

static mut GDT: [GDTEntry; 3] = [
    // Null descriptor
    GDTEntry {
        limit_low: 0,
        base_low: 0,
        base_middle: 0,
        access: 0,
        granularity: 0,
        base_high: 0,
    },
// Code segment
GDTEntry {
    limit_low: 0xFFFF,
    base_low: 0,
    base_middle: 0,
    access: 0x9A,       // Present, Ring 0, Code segment, Execute/Read
    granularity: 0xCF,  // 4KB granularity, 32-bit protected mode
    base_high: 0,
},
// Data segment
GDTEntry {
    limit_low: 0xFFFF,
    base_low: 0,
    base_middle: 0,
    access: 0x92,       // Present, Ring 0, Data segment, Read/Write
    granularity: 0xCF,  // 4KB granularity, 32-bit protected mode
    base_high: 0,
},
];

#[no_mangle]
#[link_section = ".start"]
pub extern "C" fn _start() -> ! {
    unsafe {
        // Save registers first
        core::arch::asm!(
            // Save all registers
            "mov [{0} + 0], eax",
            "mov [{0} + 4], ebx",
            "mov [{0} + 8], ecx",
            "mov [{0} + 12], edx",
            "mov [{0} + 16], esi",
            "mov [{0} + 20], edi",
            "mov [{0} + 24], ebp",
            "mov [{0} + 28], esp",

            // Reload segments with known good values
            "mov ax, 0x10",
            "mov ds, ax",
            "mov es, ax",
            "mov fs, ax",
            "mov gs, ax",
            "mov ss, ax",
            "mov esp, 0xD000",

            // Clear direction flag
            "cld",

            // Jump to Rust code
            "call {1}",
            sym REGISTERS,
            sym real_start,
            options(noreturn),
        );
    }
    unreachable!()
}

fn init_gdt() {
    unsafe {
        let gdt_desc = GDTDescriptor {
            size: (core::mem::size_of::<[GDTEntry; 3]>() - 1) as u16,
            offset: &GDT as *const _ as u32,
        };

        core::arch::asm!(
            "lgdt [{0}]",
            // Reload CS via far return
            "push 0x08",           // Code segment selector
            "push 1f",            // Offset
            "retf",              // Far return
            "1:",
            // Reload data segments
            "mov ax, 0x10",
            "mov ds, ax",
            "mov es, ax",
            "mov fs, ax",
            "mov gs, ax",
            "mov ss, ax",
            in(reg) &gdt_desc,
                         options(readonly, nostack)
        );
    }
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
