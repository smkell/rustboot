RUSTC=rustc
LD=arm-none-eabi-ld
CC=arm-none-eabi-gcc
LLC=llc -march=arm
OBJCOPY=arm-none-eabi-objcopy
GDB=arm-none-eabi-gdb
QEMU=qemu-system-arm

CFLAGS = -g -ffreestanding -nostdlib -O0 -Wall

all: floppy.img

.PHONY: run debug

%.ll: ../../%.rs
	cd ../..; $(RUSTC) --opt-level=0 --target arm-linux-noeabi --lib -S --emit-llvm -c $*.rs -o arch/arm/$*.ll
	sed -i 's/fixedstacksegment //g' $@
	sed -i 's/arm-unknown-linux-gnueabihf/arm-none-eabi/g' $@

%.s: %.ll
	$(LLC) -mcpu=arm926ej-s --float-abi=hard -asm-verbose $^ -o $@
	sed -i 's/.note.rustc,"aw"/.note.rustc,"a"/g' $@

%.o: %.s
	$(CC) -c $< -o $@

floppy.elf: linker.ld loader.o main.o
	$(CC) $(CFLAGS) -o $@ -T $^

floppy.img: floppy.elf
	$(OBJCOPY) -O binary $^ $@

run: floppy.img
	$(QEMU) -M versatilepb -m 32M -nographic -kernel $^

debug: floppy.img
	$(QEMU) -M versatilepb -m 32M -nographic -kernel $^ -s -S &
	$(GDB) -ex 'target remote localhost:1234' -ex 'symbol-file floppy.elf'
	pkill $(QEMU)
