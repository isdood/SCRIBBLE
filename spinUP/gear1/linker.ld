ENTRY(_start)
OUTPUT_FORMAT(elf32-i386)
OUTPUT_ARCH(i386)

SECTIONS {
    /* Start of boot sector */
    . = 0x7C00;

    .boot : {
        /* First include the boot text */
        *(.text.start)
        *(.text*)
        *(.rodata*)
        *(.data*)

        /* Pad up to boot signature location */
        . = 0x7DFE - 0x7C00;

        /* Add boot signature */
        SHORT(0xaa55)
    }

    /* Place metadata sections after boot sector */
    . = 0x7E00;

    /* Keep but move these sections after our boot code */
    .shstrtab : { *(.shstrtab) }
    .symtab : { *(.symtab) }
    .strtab : { *(.strtab) }

    /* These can be safely discarded */
    /DISCARD/ : {
        *(.eh_frame*)
        *(.note*)
        *(.comment*)
        *(.rel.*)
        *(.rela.*)
    }
}

/* Verify boot sector size */
ASSERT(SIZEOF(.boot) <= 512, "Error: Boot sector exceeds 512 bytes")
