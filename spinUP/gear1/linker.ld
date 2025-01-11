ENTRY(_start)
OUTPUT_FORMAT(elf32-i386)
OUTPUT_ARCH(i386)

SECTIONS {
    /* Force location counter to start of boot sector */
    . = 0x7C00;

    .boottext : AT(0x7C00) {
        *(.text.start)
        *(.text)
    }

    .bootdata : {
        *(.rodata)
        *(.data)
        . = ALIGN(16);
    }

    /* Ensure we have room for boot signature */
    .sig : AT(0x7DFE) {
        SHORT(0xaa55)
    }

    /* Everything else can be discarded */
    /DISCARD/ : {
        *(.comment)
        *(.eh_frame)
        *(.note*)
        *(*)
    }
}

/* Ensure total size doesn't exceed one sector */
ASSERT(SIZEOF(.boottext) + SIZEOF(.bootdata) + 2 <= 512, "Error: Boot sector exceeds 512 bytes")
