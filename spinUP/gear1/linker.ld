ENTRY(_start)

SECTIONS
{
    . = 0x7C00;

    .start : {
        *(.boot.text)    /* Entry point */
    }

    .text : {
        *(.text*)        /* Code */
    }

    .rodata : {
        *(.rodata*)      /* Constants */
    }

    .data : {
        *(.data*)        /* Data */
    }

    /* Ensure we don't exceed available space */
    . = 0x7C00 + 510;

    /* Add boot signature */
    .signature : {
        SHORT(0xaa55)
    }

    /* These are required by the toolchain */
    .metadata : {
        *(.shstrtab)
        *(.strtab)
        *(.symtab)
    }
}

/* Sanity check */
ASSERT(SIZEOF(.start) + SIZEOF(.text) + SIZEOF(.rodata) + SIZEOF(.data) + 2 <= 512, "Boot sector exceeds 512 bytes")
