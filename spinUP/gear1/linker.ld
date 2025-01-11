ENTRY(_start)

SECTIONS {
    . = 0x7C00;

    .text : {
        *(.boot.text)
        *(.text)
        *(.text.*)
    }

    .rodata : {
        *(.rodata)
        *(.rodata.*)
    }

    . = 0x7DFE;    /* Boot signature location */
    .boot_sig : {
        SHORT(0xaa55)
    }

    /DISCARD/ : {
        *(.comment)
        *(.note*)
        *(.eh_frame*)
        *(.debug_*)
        *(.comment)
    }
}
