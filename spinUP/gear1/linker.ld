SECTIONS {
    . = 0x7c00;
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
        *(.bss .bss.*)
    }
    .boot_signature : AT(0x7dfe) {
        *(.boot_signature)
    }
}
