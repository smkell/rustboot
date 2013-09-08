format elf
use32

extrn stack_top
extrn main

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


section ".text"
start:
	mov sp, 0x100000
	; call rust
	; symbols not working?!
	bl 0x4e0
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
