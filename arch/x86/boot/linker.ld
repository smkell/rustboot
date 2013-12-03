ENTRY(start)
OUTPUT_FORMAT(binary)

MEMORY {
    ram : org = 0x7c00, l = 225K /* 3*64 + 32 + 1 */
}

SECTIONS {
    . = 0x7c00;

    .text : {
        boot/loader.o(.text)
        *(.text*)
        *(.data*)
        *(.rodata*)
        *(.bss*)
    } >ram

    /DISCARD/ : {
        *(.eh_frame)
    }
}
