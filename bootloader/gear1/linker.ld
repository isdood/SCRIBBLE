ENTRY(_start)
OUTPUT_FORMAT(binary)

SECTIONS
{
    . = 0x7C00;
    .text : {
        *(.boot*)      /* Boot section first */
        *(.text*)
        *(.rodata*)
        *(.data*)
        . = ALIGN(510);    /* Align to 510 bytes */
        SHORT(0xAA55)      /* Boot signature */
    }

    /DISCARD/ : {
        *(.eh_frame*)
        *(.note*)
        *(.comment*)
        *(.gnu*)
        *(.bss*)          /* No BSS in bootloader */
    }
}
