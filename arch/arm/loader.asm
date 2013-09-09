format elf
use32

extrn stack_top
extrn main
extrn copy_vectors
extrn irq

public start
public memcpy
public memcmp
public malloc
public free
public abort
public __aeabi_memcpy
public __aeabi_memset
public __modsi3
public __aeabi_idiv
public vectors
public vectors_end
public irq_handler


section ".text"
start:
    mov sp, 0x18000
    bl copy_vectors
    mrs r0, cpsr
    bic r1, r0, #0x1F
    orr r1, r1, #0x12
    msr cpsr, r1
    mov sp, 0x19000
    bic r0, r0, #0x80
    msr cpsr, r0
    bl main
memcpy:
memcmp:
malloc:
free:
abort:
__aeabi_memcpy:
__aeabi_memset:
__modsi3:
__aeabi_idiv:
    b $

irq_handler dw irq

vectors:
    b start
    b $
    b $
    b $
    b $
    b $
    b irq
    b $
vectors_end:
