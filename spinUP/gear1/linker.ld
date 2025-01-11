OUTPUT_FORMAT(binary)
OUTPUT_ARCH(i386)

SECTIONS {
    . = 0x7C00;
    .text : {
        *(.boot.text)
        *(.text)
        . = 0x1FE;
        SHORT(0xaa55)
    }
    /DISCARD/ : {
        *(.comment)
        *(.note*)
        *(.eh_frame*)
        *(.debug*)
        *(.shstrtab)
        *(*)
    }
}
