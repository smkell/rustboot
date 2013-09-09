ASM=fasmarm
LD=arm-none-eabi-ld
CC=arm-none-eabi-gcc
OBJCOPY=arm-none-eabi-objcopy
GDB=arm-none-eabi-gdb
QEMU=qemu-system-arm

CFLAGS = -g -ffreestanding -nostdlib -O0 -Wall

all: floppy.img
	$(QEMU) -M versatilepb -m 32M -nographic -kernel $^

debug: floppy.img
	$(QEMU) -M versatilepb -m 32M -nographic -kernel $^ -s -S &
	$(GDB) -ex 'target remote localhost:1234' -ex 'symbol-file floppy.elf'
	pkill $(QEMU)

floppy.elf: linker.ld loader.o ../../main.s
	$(CC) $(CFLAGS) -o $@ -T $^

floppy.img: floppy.elf
	$(OBJCOPY) -O binary $^ $@

%.o: %.asm
	$(ASM) $< $@
