OUTPUT_FORMAT("binary")
OUTPUT_ARCH(i386)
ENTRY(_start)

SECTIONS {
    . = 0x7C00;
    .text : {
        *(.text*)
        . = ALIGN(2);  /* Ensure alignment */
    }

    /* Ensure we don't exceed bootloader size */
    ASSERT(. - 0x7C00 <= 510, "Bootloader too large")
}
