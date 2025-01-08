ENTRY(_start)
OUTPUT_FORMAT("binary")

SECTIONS {
    . = 0x7C00;
    .text : {
        *(.text)
        . = 510;
        BYTE(0x55)
        BYTE(0xAA)
    }

    /DISCARD/ : {
        *(.eh_frame*)
        *(.note*)
        *(.comment*)
        *(.got*)
        *(.gnu*)
    }
}
