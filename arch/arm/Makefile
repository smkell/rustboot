ASM=fasmarm
LD=arm-none-eabi-ld
QEMU=qemu-system-arm

all: floppy.img
	$(QEMU) -M versatilepb -m 32M -nographic -kernel $^

floppy.img: linker.ld loader.o ../../main.o
	$(LD) -o $@ -T $^

%.o: %.asm
	$(ASM) $< $@
