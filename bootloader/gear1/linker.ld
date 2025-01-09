OUTPUT_FORMAT(binary)
ENTRY(_start)

SECTIONS
{
    . = 0x7c00;
    .text : {
        *(.boot)
        *(.text*)
        *(.rodata*)
        *(.data*)
        . = 510;
        SHORT(0xaa55)
    }

    /DISCARD/ : {
        *(.eh_frame*)
        *(.note*)
        *(.comment*)
    }
}
