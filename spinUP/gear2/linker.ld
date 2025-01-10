ENTRY(_start)

SECTIONS {
    . = 0x7E00;

    .start : ALIGN(4K) {
        KEEP(*(.start))
        *(.text._start)
        *(.text.enter_protected_mode)
        *(.text*)
    }

    .rodata : ALIGN(4K) {
        *(.rodata*)
    }

    .gdt : ALIGN(8) {
        KEEP(*(.gdt))
    }

    .page_tables : ALIGN(4K) {
        KEEP(*(.page_tables))
    }

    .data : ALIGN(4K) {
        *(.data*)
    }

    .bss : ALIGN(4K) {
        *(COMMON)
        *(.bss*)
    }

    .stack : ALIGN(16) {
        KEEP(*(.stack))
    }

    /DISCARD/ : {
        *(.eh_frame*)
        *(.note*)
        *(.comment*)
    }
}
