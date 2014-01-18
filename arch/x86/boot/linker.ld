OUTPUT_FORMAT(elf32-i386)
ENTRY(start)

MEMORY {
    ram : org = 0x7c00, l = 225K /* 3*64 + 32 + 0.5 + 0.5 */
}

SECTIONS {
    . = 0x7c00;

    .text : {
        *loader.o(.text)
        *(.text*)
    } >ram

    .data : { *(.data*) } >ram
    .rodata : { *(.rodata*) } >ram
}
