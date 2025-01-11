OUTPUT_FORMAT(binary)
OUTPUT_ARCH(i386)

SECTIONS {
    . = 0x7C00;

    .text : {
        *(.text.start)
        *(.text*)
    }

    .rodata : {
        *(.rodata*)
    }

    .data : {
        *(.data*)
    }

    . = 0x7DFE;

    .boot_sig : {
        SHORT(0xaa55)
    }

    /DISCARD/ : {
        *(.eh_frame*)
        *(.note.*)
        *(.comment)
        *(.shstrtab)
        *(.symtab)
        *(.strtab)
    }
}

ASSERT(. <= 0x7E00, "Error: Binary exceeds 512 bytes")
