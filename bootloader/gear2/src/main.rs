#![no_std]
#![no_main]

use core::arch::{asm, global_asm};
use core::panic::PanicInfo;
use uart_16550::SerialPort;

mod boot_params;

// Change stack location to match what's actually being used
const STACK_START: u64 = 0xD000;  // Updated to match actual value
const STACK_SIZE: u64 = 0x4000;


// Serial port functions
fn write_serial(serial: &mut SerialPort, bytes: &[u8]) {
    for &byte in bytes {
        serial.send(byte);
    }
}

fn print_boot_info(serial: &mut SerialPort) {
    write_serial(serial, b"[spinUP! Gear 2] Starting...\n");
}

fn enable_a20() -> Result<(), ()> {
    unsafe {
        // Try enable A20 through BIOS
        outb(0x92, inb(0x92) | 2);
        Ok(())
    }
}

fn init_page_tables() {
    // Basic identity mapping for now
    // You'll want to implement proper paging later
}

fn init_gdt() {
    // Basic GDT setup
    // You'll want to implement this properly
}

fn init_idt() {
    // Basic IDT setup
    // You'll want to implement this properly
}

fn load_kernel(load_addr: u64, size: u64) -> Result<u64, ()> {
    // Basic validation
    if load_addr == 0 || size == 0 {
        return Err(());
    }

    // For now, we'll just verify the addresses are valid
    // Later, you'll want to actually load the kernel from disk
    if load_addr >= 0x100000 && // Above 1MB
        load_addr + size <= 0xFFFFFFFF { // Within 4GB limit
            Ok(load_addr) // Return entry point (same as load address for now)
        } else {
            Err(())
        }
}


#[no_mangle]
pub extern "C" fn real_start() -> ! {
    let mut serial_port = unsafe { SerialPort::new(0x3F8) };
    serial_port.init();

    write_serial(&mut serial_port, b"[spinUP! Gear 2] Starting at 0x7E00...\n");
    write_serial(&mut serial_port, b"[spinUP! Gear 2] Stack at 0xD000\n");

    print_boot_info(&mut serial_port);

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

    // We know the types match now
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
}

global_asm!(
    ".section .text._start",
    ".global _start",
    "_start:",
    // Set up stack
    "    mov esp, {0}",
    "    mov ebp, esp",
    "    call {1}",
    "1:  hlt",
    "    jmp 1b",
    // Add boot signature at the end of first sector
    ".org 0x1FE",  // Position at end of first sector
    ".word 0xAA55", // Boot signature
    const(STACK_START + STACK_SIZE),
            sym real_start,
);

#[inline(never)]
fn jump_to_kernel(entry_point: u64) -> ! {
    unsafe {
        asm!(
            "jmp {}",
             in(reg) entry_point,
             options(noreturn)
        );
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {
        unsafe {
            asm!("hlt", options(nomem, nostack));
        }
    }
}

// Port I/O functions
pub unsafe fn inb(port: u16) -> u8 {
    let value: u8;
    asm!("in al, dx", out("al") value, in("dx") port, options(nomem, nostack));
    value
}

pub unsafe fn outb(port: u16, value: u8) {
    asm!("out dx, al", in("dx") port, in("al") value, options(nomem, nostack));
}

fn halt() -> ! {
    loop {
        unsafe {
            asm!("hlt", options(nomem, nostack));
        }
    }
}
