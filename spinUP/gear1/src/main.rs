#![no_std]
#![no_main]

use core::arch::global_asm;
use core::panic::PanicInfo;

#[panic_handler]
fn panic(_: &PanicInfo) -> ! {
    loop {}
}

global_asm!(r#"
.section .boot, "ax"
.code16
.globl _start
_start:
# Jump over BPB
jmp short start
nop

# BPB (BIOS Parameter Block)
.space 59, 0

start:
# Set up segments
xor ax, ax
mov ds, ax
mov es, ax
mov ss, ax
mov sp, 0x7c00

# Load sector 2 to 0x7E00
mov ax, 0x0201     # AH=read(2), AL=1 sector
mov cx, 0x0002     # CH=track 0, CL=sector 2
xor dh, dh         # DH=head 0
mov bx, 0x7e00     # ES:BX = 0:0x7E00
int 0x13

# Jump to loaded sector
.byte 0xea         # Far jump opcode
.word 0x0000       # Offset
.word 0x07e0       # Segment
"#);
