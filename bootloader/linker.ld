OUTPUT_FORMAT("binary")
ENTRY(_start)

SECTIONS {
    . = 0x7C00;
    .boot : {
        /* All sections in one */
        *(.boot)
        *(.text*)
        *(.rodata*)
        *(.data*)
        *(.bss*)

        /* Pad to 510 bytes from start of section */
        . = 0x7C00 + 510;

        /* Boot signature */
        BYTE(0x55)
        BYTE(0xAA)
    }

    /DISCARD/ : {
        *(.eh_frame*)
        *(.note*)
        *(.comment*)
        *(.comment)
        *(.note.GNU-stack)
        *(.note.gnu.build-id)
        *(.got)
        *(.gcc_except_table)
    }
}
