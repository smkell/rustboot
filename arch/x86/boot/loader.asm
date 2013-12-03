use16

global __morestack
global abort
global memcmp
global memcpy
global malloc
global free
global start

extern main

start:
    ; initialize segment registers
    xor ax, ax
    mov ds, ax
    mov es, ax
    mov ss, ax
    ; initialize stack
    mov ax, 0x7bff
    mov sp, ax
    ; load rust code into 0x7e00...0xfe00 so we can jump to it later
    mov ah, 2       ; read
    mov al, 65      ; 65 sectors (32.5 KiB)
    mov ch, 0       ; cylinder & 0xff
    mov cl, 2       ; sector | ((cylinder >> 2) & 0xc0)
    xor dx, dx      ; head
    mov bx, 0x7e00  ; read buffer
    int 0x13
    jc error
    ; load the rest into segments starting at 0x10000
    xor di, di
    mov si, 67
.loop:
    add di, 0x1000  ; destination
    mov es, di
    mov ax, si
    mov bl, 18
    div bl
    xor bx, bx
    mov dh, al
    mov ch, al
    shr ch, 1
    and dh, 1
    mov cl, ah
    mov ax, 128 | (2 << 8) ; 128 sectors (64 KiB)
    int 0x13
    jc error
    add si, 128
    cmp di, 0x3000
    jne .loop
    ; load protected mode GDT and a null IDT (we don't need interrupts)
    cli
    lgdt [gdtr]
    lidt [idtr]
    ; set protected mode bit of cr0
    mov eax, cr0
    or eax, 1
    mov cr0, eax
    ; far jump to load CS with 32 bit segment
    jmp 0x08:protected_mode

error:
    mov bx, ax
    mov si, .msg
.loop:
    lodsb
    or al, al
    jz .done
    mov ah, 0x0e
    int 0x10
    jmp .loop
.done:
    jmp $
    .msg db "could not read disk", 0

protected_mode:
    use32
    ; load all the other segments with 32 bit data segments
    mov eax, 0x10
    mov ds, eax
    mov es, eax
    mov fs, eax
    mov gs, eax
    mov ss, eax
    ; set up stack
    mov eax, 0x7bff
    mov esp, eax
    ; clear the screen
    mov edi, 0xb8000
    mov ecx, 80*25*2
    mov al, 0
    rep stosb
    ; rust functions compare esp against [gs:0x30] as a sort of stack guard thing
    ; as long as we set [gs:0x30] to dword 0, it should be ok
    mov [gs:0x30], dword 0
    ; jump into rust
    call main
abort:
__morestack:
memcmp:
memcpy:
malloc:
free:
    jmp $

gdtr:
    dw (gdt_end - gdt) + 1  ; size
    dd gdt                  ; offset

idtr:
    dw 0
    dd 0

gdt:
    ; null entry
    dq 0
    ; code entry
    dw 0xffff       ; limit 0:15
    dw 0x0000       ; base 0:15
    db 0x00         ; base 16:23
    db 0b10011010   ; access byte - code
    db 0x4f         ; flags/(limit 16:19). flag is set to 32 bit protected mode
    db 0x00         ; base 24:31
    ; data entry
    dw 0xffff       ; limit 0:15
    dw 0x0000       ; base 0:15
    db 0x00         ; base 16:23
    db 0b10010010   ; access byte - data
    db 0x4f         ; flags/(limit 16:19). flag is set to 32 bit protected mode
    db 0x00         ; base 24:31
gdt_end:

times 510-($-$$) db 0
db 0x55
db 0xaa

%include "memset.asm"
