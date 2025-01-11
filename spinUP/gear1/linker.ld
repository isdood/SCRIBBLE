OUTPUT_FORMAT(binary)
OUTPUT_ARCH(i386)

SECTIONS {
    . = 0x7C00;

    .text : {
        *(.boot.text)
        *(.text)
    }

    .data : {
        *(.rodata)
        *(.data)
    }

    /* Pad to boot signature */
    . = 0x7DFE;

    /* Add boot signature */
    .signature : {
        SHORT(0xaa55)
    }

    /* Discard everything else */
    /DISCARD/ : {
        *(.eh_frame*)
        *(.comment*)
        *(.note*)
        *(.debug*)
        *(.gcc_except_table*)
        *(.gnu.attributes*)
        *(.jcr*)
        *(.got*)
        *(.got.plt*)
        *(.data.rel.ro*)
        *(.bss*)
    }
}
