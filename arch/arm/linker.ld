ENTRY(start)

MEMORY {
    ram : org = 0x8000, l = 32K
}

SECTIONS {
    . = 0x8000;

    .text : {
        loader.o(.text)
        *(.text)
        *(.data)
        *(.rodata)
        *(.bss)
    } >ram

    stack_top = 0x100000;
}
