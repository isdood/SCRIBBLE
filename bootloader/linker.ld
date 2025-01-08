OUTPUT_FORMAT("binary")
ENTRY(_start)

SECTIONS {
    . = 0x7C00;
    .boot : {
        /* Entry point first */
        KEEP(*(.text._start))

        /* Then rest of the code and data */
        *(.text*)
        *(.rodata*)
        *(.data*)
        *(.bss*)

        /* Calculate remaining space */
        . = 0x7C00 + 510;

        /* Add boot signature */
        BYTE(0x55)
        BYTE(0xAA)
    }

    /* Ensure we don't exceed boot sector size */
    ASSERT(. <= 0x7E00, "Boot sector too large")

    /* Discard debug sections */
    /DISCARD/ : {
        *(.comment)
        *(.note*)
        *(.eh_frame*)
        *(.debug*)
    }
}
