ENTRY(start)
OUTPUT_FORMAT(binary)

MEMORY {
    ram : org = 0x7c00, l = 161K
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
