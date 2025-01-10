ENTRY(_start)

SECTIONS {
    . = 0x7E00;

    .boot : {
        *(.boot)
        *(.boot.*)
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

    /DISCARD/ : {
        *(.eh_frame*)
    }
}
