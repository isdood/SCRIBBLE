ENTRY(_start)

SECTIONS {
    . = 0x7C00;

    .boot : {
        *(.boot.text);
        *(.boot.text.*);
        . = 510;
        SHORT(0xAA55);
    }

    /DISCARD/ : {
        *(.eh_frame);
        *(.eh_frame_hdr);
        *(.comment);
    }
}
