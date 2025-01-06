/* linker.ld */

ENTRY(_start) /* Entry point of the kernel */

SECTIONS
{
    /* Define the start of the code segment */
    . = 0x100000;

    /* Read-only code and data */
    .text :
    {
        *(.text._start) /* Ensure the entry point is first */
        *(.text*)       /* All .text sections */
        *(.rodata*)     /* All read-only data (rodata) sections */
        *(.eh_frame*)   /* Unwind information */
    }

    /* Read-write data */
    .data :
    {
        *(.data*)       /* All .data sections */
    }

    /* BSS: Zero-initialized data */
    .bss :
    {
        *(.bss*)        /* All .bss sections */
        *(COMMON)       /* Common symbols */
    }

    /* Stack area: ensures it is placed at the end of the .bss section */
    .stack :
    {
        . = ALIGN(16);  /* Align to 16 bytes */
        _stack_start = .; /* Mark the start of the stack */
        . += 8K;        /* Allocate 8KB for the stack */
        _stack_end = .; /* Mark the end of the stack */
    }

    /* Provide the end of the kernel image for heap allocation */
    /DISCARD/ :
    {
        *(.note.gnu.property)
    }

    _end = .; /* Mark the end of the kernel image */
}
