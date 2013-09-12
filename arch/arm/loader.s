.syntax unified
.fpu softvfp

.global start
.global memcpy
.global memcmp
.global malloc
.global free
.global abort
.global __aeabi_memcpy
.global __aeabi_memset
.global __modsi3
.global __aeabi_idiv
.global vectors
.global vectors_end
.global irq_handler

.section .text.start
.weak start
.type start, %function

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
    b .

irq_handler:
.word irq

.type vectors, %object
.size vectors, .-vectors

vectors:
    b start
    b .
    b .
    b .
    b .
    b .
    b irq
    b .
vectors_end:
