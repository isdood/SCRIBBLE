OUTPUT_FORMAT("binary")
SECTIONS
{
    . = 0x7C00;
    .text : {
        *(.boot.text);
        *(.boot.text.*);

        /* Fill to 510 bytes, then add boot signature */
        . = 510;
        SHORT(0xAA55)
    }

    /DISCARD/ : {
        *(.*);
    }
}
