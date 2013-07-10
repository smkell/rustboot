use32

global __morestack
global abort
global memcmp
global memcpy
global malloc
global free
global start
global upcall_call_shim_on_rust_stack

_GLOBAL_OFFSET_TABLE_ equ 0

extern main

start:
    ; rust functions compare esp against [gs:0x30] as a sort of stack guard thing
    ; as long as we set [gs:0x30] to dword 0, it should be ok
    mov [gs:0x30], dword 0
    ; clear the screen a slightly different colour
    mov edi, 0xb8000
    mov ecx, 80*25*2
    mov al, 1
    rep stosb
    ; jump into rust
    call main
abort:
__morestack:
memcmp:
memcpy:
malloc:
free:
    jmp $

upcall_call_shim_on_rust_stack:
    push ebp
    mov ebp, esp

    push dword[ebp+8]
    call dword[ebp+12]

    mov esp, ebp
    pop ebp
    ret
