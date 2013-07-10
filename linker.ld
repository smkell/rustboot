ENTRY(start)
OUTPUT_FORMAT(binary)

SECTIONS {
    . = 0x7e00;

    .text : {
        runtime.o(.text)
        *(.text)
        *(.gnu.linkonce.t*)
    }

    /DISCARD/ : {
        *(.comment)
        *(.eh_frame)
        *(.rel.eh_frame)
    }
}
