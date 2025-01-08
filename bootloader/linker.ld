ENTRY(_start)

SECTIONS {
    /* Load the second stage bootloader right after the MBR */
    . = 0x7E00;

    .boot : {
        KEEP(*(.multiboot_header))
        *(.boot)
    }

    .text : {
        *(.text .text.*)
    }

    .rodata : {
        *(.rodata .rodata.*)
    }

    .data : {
        *(.data .data.*)
    }

    .bss : {
        *(COMMON)
        *(.bss .bss.*)
    }

    .bootparams : {
        *(.bootparams)
    }
}
