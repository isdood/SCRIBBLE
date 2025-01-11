ENTRY(_start)
OUTPUT_FORMAT(elf32-i386)

SECTIONS {
    . = 0x7C00;

    /* Combine everything into a single bootable section */
    .boot : {
        KEEP(*(.boot.text))  /* Our entry point */
        *(.text*)            /* Other code */
        *(.rodata*)          /* Read-only data */
        *(.data*)            /* Data */
        . = 510 - (. - 0x7C00);  /* Pad to boot signature */
        SHORT(0xaa55)            /* Boot signature */
    } =0

    /* These sections must exist for the linker but won't be in final binary */
    .shstrtab 0 : { *(.shstrtab) }
    .symtab 0 : { *(.symtab) }
    .strtab 0 : { *(.strtab) }

    /DISCARD/ : {
        *(.eh_frame*)
        *(.note*)
        *(.comment*)
        *(.rel*)
        *(.rela*)
        *(.bss*)
    }
}

ASSERT(SIZEOF(.boot) <= 512, "Error: Boot sector exceeds 512 bytes")
