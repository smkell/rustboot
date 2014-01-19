global memset
global memcpy

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

memcpy:
    push ebp
    mov ebp, esp
    push esi
    push edi

    mov edi, [ebp+8]
    mov esi, [ebp+12]
    mov ecx, [ebp+16]

    cld
    rep movsb

    mov eax, [ebp+8]
    pop edi
    pop esi
    pop ebp
    ret
