.text
.code 32
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
    b .
