format elf
use32

extrn stack_top

public start

section ".text"
start:
	ldr sp, [stack_top]
	b $
