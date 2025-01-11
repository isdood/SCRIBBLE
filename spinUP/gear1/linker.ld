SECTIONS {
    . = 0x7C00;
    .text : {
        *(.boot.text)
        *(.text)
    }
    . = 0x7DFE;
    .sig : {
        SHORT(0xaa55)
    }
    /DISCARD/ : {
        *(*)
    }
}
