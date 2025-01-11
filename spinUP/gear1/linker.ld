ENTRY(_start)
OUTPUT_FORMAT("elf32-i386")
OUTPUT_ARCH(i386)

SECTIONS {
    . = 0x7C00;

    .boot : {
        *(.boot.text .boot.text.*);
        . = 510;
        SHORT(0xAA55);
    }

    .text : {
        *(.text .text.*);
    }

    .rodata : {
        *(.rodata .rodata.*);
    }

    .data : {
        *(.data .data.*);
    }

    .bss : {
        *(.bss .bss.*);
    }

    .shstrtab : {
        *(.shstrtab);
    }

    /DISCARD/ : {
        *(.eh_frame*);
        *(.comment);
        *(.note*);
        *(.debug*);
    }
}
