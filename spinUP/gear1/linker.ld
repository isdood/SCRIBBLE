ENTRY(_start)
SECTIONS {
    . = 0x7C00;
    .boot.text : {
        *(.boot.text)
        . = 510;
        SHORT(0xaa55)
    }
    /DISCARD/ : {
        *(.comment)
        *(.eh_frame)
        *(.note.*)
        *(.debug_*)
    }
}
