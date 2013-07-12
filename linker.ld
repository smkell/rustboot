ENTRY(start)
OUTPUT_FORMAT(binary)

MEMORY {
    ram : org = 0x7e00, l = 32K
}

SECTIONS {
    . = 0x7e00;

    .text : {
        runtime.o(.text)
        *(.text)
        *(.gnu.linkonce.t*)
    } >ram

    /DISCARD/ : {
        *(.comment)
        *(.eh_frame)
        *(.rel.eh_frame)
    }
}
