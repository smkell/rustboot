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
