OUTPUT_FORMAT("binary")
ENTRY(_start)

SECTIONS {
    . = 0x7C00;
    .text : {
        *(.boot.text)
        *(.text)
        *(.text.*)
        . = ALIGN(2);
    }
    .rodata : {
        *(.rodata)
        *(.rodata.*)
        . = ALIGN(2);
    }
    .boot_sig : AT(0x7DFE) {
        SHORT(0xaa55)
    }

    .bss : {
        *(COMMON)
        *(.bss)
        *(.bss.*)
    }

    /DISCARD/ : {
        *(.eh_frame*)
        *(.note*)
        *(.comment*)
    }
}
