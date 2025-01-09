ENTRY(_start)
OUTPUT_FORMAT(elf32-i386)

SECTIONS {
    . = 0x7E00;

    .start : {
        *(.start)
    }

    .text : {
        *(.text*)
    }

    .rodata : {
        *(.rodata*)
    }

    .data : {
        *(.data*)
    }

    .bss : {
        *(.bss*)
    }
}
