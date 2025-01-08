#![no_std]
#![no_main]
#![feature(naked_functions)]

use core::{
    arch::{asm, global_asm},
    panic::PanicInfo,
};
use uart_16550::SerialPort;
use x86_64::{
    structures::paging::{PageTable, PageTableFlags},
    PhysAddr,
};

mod boot_params {
    #[derive(Debug)]
    pub struct BootParams {
        pub kernel_load_addr: u32,
        pub kernel_size: u32,
    }

    #[no_mangle]
    pub static mut BOOT_PARAMS: BootParams = BootParams {
        kernel_load_addr: 0,
        kernel_size: 0,
    };
}

// Include the generated bootloader info
include!(concat!(env!("OUT_DIR"), "/bootloader_info.rs"));

// Constants for memory management
const PAGE_TABLE_START: u64 = 0x1000;
const STACK_START: u64 = 0x9000;
const STACK_SIZE: u64 = 0x4000;

// Stack with proper alignment
#[link_section = ".bss"]
#[used]
static mut STACK: [u8; STACK_SIZE as usize] = [0; STACK_SIZE as usize];

// Helper function to write strings to serial port
fn write_serial(port: &mut SerialPort, s: &[u8]) {
    for &byte in s {
        port.send(byte);
    }
}

#[repr(C, packed)]
struct GDTEntry {
    limit_low: u16,
    base_low: u16,
    base_middle: u8,
    access: u8,
    granularity: u8,
    base_high: u8,
}

#[repr(C, packed)]
struct GDTPointer {
    limit: u16,
    base: u32,
}

global_asm!(
    ".section .text._start",
    ".global _start",
    "_start:",
    "    mov esp, {stack_top}",
    "    mov ebp, esp",
    "    call {real_start}",
    "1:  hlt",
    "    jmp 1b",
    stack_top = const(STACK_START + STACK_SIZE),
            real_start = sym real_start,
);

const VGA_BUFFER: *mut u8 = 0xb8000 as *mut u8;

fn print_at_boot(message: &str) {
    let color_byte = 0x0f; // White foreground (0x0f) on black background (0x00)
    for (i, byte) in message.bytes().enumerate() {
        unsafe {
            *VGA_BUFFER.add(i * 2) = byte;           // Character byte
            *VGA_BUFFER.add(i * 2 + 1) = color_byte; // Color byte
        }
    }
}

#[no_mangle]
pub extern "C" fn real_start() -> ! {

    // Display welcome message
    print_at_boot("Welcome Kleb!");

    // Initialize serial port for debugging
    let mut serial_port = unsafe { SerialPort::new(0x3F8) };
    serial_port.init();

    // Print bootloader information
    print_boot_info(&mut serial_port);

    // Enable A20 line
    if let Err(()) = enable_a20() {
        write_serial(&mut serial_port, b"[spinUP! Gear 2] Failed to enable A20 line\n");
        halt();
    }

    // Initialize basic page tables
    init_page_tables();

    // Read boot parameters with proper type annotation
    let boot_params: &boot_params::BootParams = unsafe {
        &*(&raw const boot_params::BOOT_PARAMS as *const boot_params::BootParams)
    };

    // Initialize essential services
    init_gdt();
    init_idt();

    // Prepare to load kernel
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

fn print_boot_info(serial_port: &mut SerialPort) {
    write_serial(serial_port, b"\nspinUP! Gear 2 Bootloader\n");
    write_serial(serial_port, b"Version: ");
    write_serial(serial_port, b"0.1.0");  // Hardcoded version for now
    write_serial(serial_port, b"\n");
}

fn enable_a20() -> Result<(), ()> {
    // Try enabling A20 through BIOS
    let success: u8;
    unsafe {
        asm!(
            "mov ax, 0x2401",
             "int 0x15",
             "setc {0}",
             out(reg_byte) success
        );
    }

    if success == 0 {
        Ok(())
    } else {
        // If BIOS method failed, try keyboard controller
        enable_a20_via_kbd()
    }
}

fn enable_a20_via_kbd() -> Result<(), ()> {
    let mut port_a = unsafe { Port::new(0x60) };
    let mut port_b = unsafe { Port::new(0x64) };

    // Disable keyboard
    wait_kbd();
    port_b.write(0xAD_u8);

    // Read controller output port
    wait_kbd();
    port_b.write(0xD0_u8);
    wait_kbd_out();
    let value: u8 = port_a.read();

    // Write controller output port
    wait_kbd();
    port_b.write(0xD1_u8);
    wait_kbd();
    port_a.write(value | 2);

    // Enable keyboard
    wait_kbd();
    port_b.write(0xAE_u8);
    wait_kbd();

    Ok(())
}

fn init_page_tables() {
    let page_table = unsafe { &mut *(PAGE_TABLE_START as *mut PageTable) };

    // Identity map first 1MB
    for i in 0..256 {
        page_table[i].set_addr(
            PhysAddr::new(i as u64 * 0x1000),
                               PageTableFlags::PRESENT | PageTableFlags::WRITABLE,
        );
    }
}

fn init_gdt() {
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
            access: 0x9A,
            granularity: 0xCF,
            base_high: 0,
        },
        // Data segment
        GDTEntry {
            limit_low: 0xFFFF,
            base_low: 0,
            base_middle: 0,
            access: 0x92,
            granularity: 0xCF,
            base_high: 0,
        },
    ];

    let gdt_ptr = GDTPointer {
        limit: (core::mem::size_of::<[GDTEntry; 3]>() - 1) as u16,
        base: (&raw const GDT as *const _ as u32),  // Remove unsafe block
    };

    unsafe {
        asm!(
            "lgdt [{0}]",
             in(reg) &gdt_ptr as *const _,
             options(readonly, nostack)
        );
    }
}

fn init_idt() {
    // Initialize with default handlers
    // This is a minimal implementation for the bootloader
}

fn load_kernel(load_addr: u32, _size: u32) -> Result<u32, ()> {
    // TODO: Implement actual kernel loading from disk
    // For now, just return the load address as entry point
    Ok(load_addr)
}

#[inline(never)]
fn jump_to_kernel(entry_point: u32) -> ! {
    unsafe {
        asm!(
            "mov [{0}], {1:e}",    // Store jump target in memory
             "jmp [{0}]",           // Indirect jump through memory
             in(reg) &entry_point as *const _,
             in(reg) entry_point,
             options(noreturn)
        );
    }
}

fn halt() -> ! {
    loop {
        unsafe {
            asm!("hlt", options(nomem, nostack));
        }
    }
}

#[inline]
fn wait_kbd() {
    while unsafe { Port::new(0x64).read::<u8>() } & 2 != 0 {}
}

#[inline]
fn wait_kbd_out() {
    while unsafe { Port::new(0x64).read::<u8>() } & 1 == 0 {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    halt()
}

// Port I/O helper
struct Port {
    port: u16,
}

impl Port {
    unsafe fn new(port: u16) -> Port {
        Port { port }
    }

    fn read<T>(&mut self) -> T where T: PortRead {
        unsafe { T::read_from_port(self.port) }
    }

    fn write<T>(&mut self, value: T) where T: PortWrite {
        unsafe { T::write_to_port(self.port, value) }
    }
}

trait PortRead {
    unsafe fn read_from_port(port: u16) -> Self;
}

trait PortWrite {
    unsafe fn write_to_port(port: u16, value: Self);
}

impl PortRead for u8 {
    unsafe fn read_from_port(port: u16) -> u8 {
        let value: u8;
        asm!("in al, dx", out("al") value, in("dx") port, options(nomem, nostack));
        value
    }
}

impl PortWrite for u8 {
    unsafe fn write_to_port(port: u16, value: u8) {
        asm!("out dx, al", in("dx") port, in("al") value, options(nomem, nostack));
    }
}
