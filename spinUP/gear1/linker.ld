SECTIONS {
    . = 0x7C00;

    .text : {
        *(.boot.text)
        *(.text*)
        . = 510 - (. - 0x7C00);  /* Pad to 510 bytes */
        SHORT(0xaa55)            /* Boot signature */
    }

    /DISCARD/ : {
        *(.comment)
        *(.note*)
        *(.eh_frame*)
        *(.debug_*)
        *(.shstrtab)
        *(.symtab)
        *(.strtab)
    }
}
