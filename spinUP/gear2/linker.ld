ENTRY(_start)

SECTIONS {
    . = 0x7E00;

    .start ALIGN(4K) : AT(ADDR(.start)) {
        KEEP(*(.start))
        *(.text._start)
        *(.text.enter_protected_mode)
        *(.text*)
    }

    .rodata ALIGN(4K) : AT(ADDR(.rodata)) {
        *(.rodata*)
    }

    /* GDT needs to be accessible in both real and protected mode */
    .gdt ALIGN(8) : AT(ADDR(.gdt)) {
        _gdt_start = .;
        KEEP(*(.gdt))
        _gdt_end = .;
    }

    /* Page tables need 4K alignment and we want to track their addresses */
    .page_tables ALIGN(4K) : AT(ADDR(.page_tables)) {
        _page_tables_start = .;
        KEEP(*(.page_tables))
        _page_tables_end = .;
    }

    .data ALIGN(4K) : AT(ADDR(.data)) {
        _data_start = .;
        *(.data*)
        _data_end = .;
    }

    .bss ALIGN(4K) : AT(ADDR(.bss)) {
        _bss_start = .;
        *(COMMON)
        *(.bss*)
        _bss_end = .;
    }

    /* Stack needs to be properly aligned and we track its boundaries */
    .stack ALIGN(16) : AT(ADDR(.stack)) {
        _stack_bottom = .;
        . += 0x4000;
        _stack_top = .;
        PROVIDE(_stack_start = .);
    }

    /* Add these symbols for debugging */
    _end = .;

    /DISCARD/ : {
        *(.eh_frame*)
        *(.note*)
        *(.comment*)
    }
}
