ENTRY(_start)
OUTPUT_FORMAT(binary)

SECTIONS
{
    . = 0x7C00;
    .text : {
        *(.text*)
        *(.rodata*)
        *(.data*)
    }

    /DISCARD/ : {
        *(.eh_frame*)
        *(.note*)
        *(.comment*)
        *(.gnu*)
    }
}
