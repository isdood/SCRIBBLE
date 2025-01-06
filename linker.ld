ENTRY(_start)

SECTIONS
{
    . = 0x100000;

    .text :
    {
        *(.text._start)
        *(.text*)
        *(.rodata*)
        *(.eh_frame*)
    }

    .data :
    {
        *(.data*)
    }

    .bss :
    {
        *(.bss*)
        *(COMMON)
    }

    .stack :
    {
        . = ALIGN(16);
        _stack_start = .;
        . += 8K;
        _stack_end = .;
    }

    /DISCARD/ :
    {
        *(.note.gnu.property)
    }

    _end = .;
}
