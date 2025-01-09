ENTRY(_start)

SECTIONS {
    /* First stage bootloader must start at 0x7C00 (BIOS load address) */
    . = 0x7C00;

    .boot : {
        *(.boot)
        *(.text)
        *(.rodata)
        *(.data)

        /* Ensure boot signature is at the right place */
        . = 510;
        SHORT(0xAA55)
    }
}
