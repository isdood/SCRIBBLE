ENTRY(_start)
OUTPUT_FORMAT("binary")
OUTPUT_ARCH(i386)

SECTIONS {
    . = 0x7C00;

    .boot : ALIGN(2) {
        /* Boot code */
        *(.boot.text*)
        *(.text*)

        /* Read-only data */
        *(.rodata*)

        /* Writable data */
        *(.data*)

        /* BSS */
        *(.bss*)

        /* Pad to 510 bytes */
        . = ALIGN(2);
        . = 0x7C00 + 510;
    }

    .signature : {
        SHORT(0xAA55)
    }

    /DISCARD/ : {
        *(.eh_frame*)
        *(.note*)
        *(.comment*)
        *(.gnu*)
    }
}
