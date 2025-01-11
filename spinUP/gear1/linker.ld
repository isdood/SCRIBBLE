OUTPUT_FORMAT(binary)
OUTPUT_ARCH(i386)

SECTIONS {
    /* Bootloader starts at 0x7C00 */
    . = 0x7C00;

    .boot : {
        /* Place all sections marked as .boot.text here */
        *(.boot.text .boot.text.*);
        /* Pad to 510 bytes and add boot signature */
        . = 0x7C00 + 510;
        SHORT(0xAA55)
    }

    /* Discard everything else */
    /DISCARD/ : {
        *(.text*)
        *(.data*)
        *(.bss*)
        *(.rodata*)
        *(.comment*)
        *(.eh_frame*)
        *(.note*)
        *(.riscv.attributes*)
        *(.debug*)
    }
}
