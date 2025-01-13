ENTRY(_start)

SECTIONS {
    . = 0x7E00;

    .text : ALIGN(4K) {
        *(.text .text.*)
    }

    .rodata : ALIGN(4K) {
        *(.rodata .rodata.*)
    }

    .data : ALIGN(4K) {
        *(.data .data.*)
        *(.got .got.*)
    }

    .bss : ALIGN(4K) {
        __bss_start = .;
        *(.bss .bss.*)
        *(COMMON)
        __bss_end = .;
    }

    .eh_frame : ALIGN(4K) {
        *(.eh_frame)
    }

    /DISCARD/ : {
        *(.note.gnu.*)
        *(.note.gnu.build-id)
    }
}
