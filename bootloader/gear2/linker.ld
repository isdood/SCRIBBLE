ENTRY(_start)

SECTIONS {
    /* Second stage bootloader starts at 0x7E00 */
    . = 0x7E00;

    .boot : ALIGN(512) {
        KEEP(*(.multiboot_header))
        *(.boot)
    }

    .text : ALIGN(16) {
        *(.text .text.*)
    }

    .rodata : ALIGN(16) {
        *(.rodata .rodata.*)
    }

    .data : ALIGN(16) {
        *(.data .data.*)
    }

    .bss : ALIGN(16) {
        *(COMMON)
        *(.bss .bss.*)
    }

    .bootparams : ALIGN(16) {
        *(.bootparams)
    }

    /* Ensure total size is aligned to sector boundary */
    . = ALIGN(512);

    /DISCARD/ : {
        *(.eh_frame*)
        *(.note*)
        *(.comment)
    }
}
