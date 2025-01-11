SECTIONS {
    . = 0x7c00;
    .text.start : { *(.text.start) }
    . = 0x7dfe;
    .boot_sig : { *(.boot_sig) }
}
