// boot/spINIT/src/main.rs
#![no_std]
#![no_main]

use core::sync::atomic::{AtomicUsize, Ordering};

const CELL1_ADDR: usize = 0x100000;  // 1MB mark
const SPINUP_SIGNATURE: u64 = 0x5350494E55505F5F; // "SPINUP__"

static BOOT_TIMESTAMP: AtomicUsize = AtomicUsize::new(1705117333); // 2025-01-13 04:42:13 UTC

#[no_mangle]
pub extern "C" fn _start() -> ! {
    // Load spinUP to cell1
    unsafe {
        load_spinup_to_cell1();
    }

    // Transfer control to spinUP
    jump_to_spinup();
}

unsafe fn load_spinup_to_cell1() {
    // Copy spinUP binary to cell1
    let spinup_binary: &[u8] = include_bytes!("../../spinUP/target/spinup.bin");
    let dest = CELL1_ADDR as *mut u8;

    for (i, &byte) in spinup_binary.iter().enumerate() {
        *dest.add(i) = byte;
    }

    // Verify signature
    let signature = *(CELL1_ADDR as *const u64);
    if signature != SPINUP_SIGNATURE {
        panic!("Invalid spinUP signature");
    }
}

fn jump_to_spinup() -> ! {
    let spinup_entry = CELL1_ADDR as *const ();
    let spinup_fn: fn() -> ! = unsafe { core::mem::transmute(spinup_entry) };
    spinup_fn()
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
