ENTRY(_start)

SECTIONS {
    . = 0x7C00;
    .boot : {
        *(.boot.text)
        *(.rodata*)
        . = ALIGN(510);
        BYTE(0x55)
        BYTE(0xAA)
    }

    /DISCARD/ : {
        *(.comment)
        *(.note*)
        *(.eh_frame*)
        *(.debug*)
        *(*)
    }
}
