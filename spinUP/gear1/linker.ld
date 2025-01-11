ENTRY(_start)

SECTIONS {
    . = 0x7C00;

    .text : {
        *(.boot.text)
        *(.text*)
    }

    .rodata : {
        *(.rodata*)
    }

    .data : {
        *(.data*)
    }

    .bss : {
        *(.bss*)
        *(COMMON)
    }

    . = 0x7DFE;
    .boot_signature : {
        SHORT(0xaa55)
    }

    /* Ensure we don't exceed the 512 byte limit */
    . = 0x7E00;

    /* Keep .shstrtab but discard other unnecessary sections */
    .shstrtab : {
        *(.shstrtab)
    }
}

/* Ensure the binary doesn't exceed 512 bytes */
ASSERT(. <= 0x7E00, "Error: Boot sector exceeds 512 bytes")
