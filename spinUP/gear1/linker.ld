ENTRY(_start)

SECTIONS {
    . = 0x7c00;
    .boot : {
        *(.boot)
    }
    .text : {
        *(.text)
    }

    . = 0x7c00 + 510;
    .signature : {
        SHORT(0xaa55)
    }

    /DISCARD/ : {
        *(.eh_frame)
        *(.note.*)
        *(.comment)
        *(.gnu*)
        *(.data*)
        *(.bss*)
        *(.rodata*)
    }
}
