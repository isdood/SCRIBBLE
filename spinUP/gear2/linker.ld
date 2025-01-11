ENTRY(_start)
OUTPUT_FORMAT(elf32-i386)

SECTIONS {
    . = 0x7e00;    /* Load address where Gear1 jumps to */

    .text : {
        *(.text*)  /* Code section */
    }

    .rodata : {
        *(.rodata*)  /* Read-only data */
    }

    .data : {
        *(.data*)  /* Initialized data */
    }

    .bss : {
        *(.bss*)   /* Uninitialized data */
        *(COMMON)
    }
}
