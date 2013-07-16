use32

global __morestack
global abort
global memcmp
global memcpy
global memset
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
    ; jump into rust
    call main
abort:
__morestack:
memcmp:
memcpy:
malloc:
free:
    jmp $

memset:
    push ebp
    mov ebp, esp
    push edi

    mov edi, [ebp+8]
    movzx eax, byte[ebp+12]
    mov ecx, [ebp+16]
    cld

    test ecx, 0xFFFFFFF0
    jz .bytes

    mov ah, al
    mov edx, eax
    sal edx, 16
    or eax, edx
    shrd edx, ecx, 2
    shr ecx, 2
    rep stosd
    shld ecx, edx, 2
 .bytes:
    rep stosb

    mov eax, [ebp+8]
    pop edi
    pop ebp
    ret

upcall_call_shim_on_rust_stack:
    push ebp
    mov ebp, esp

    push dword[ebp+8]
    call dword[ebp+12]

    mov esp, ebp
    pop ebp
    ret
