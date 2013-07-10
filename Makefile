CC=gcc -m32
LD=ld -melf_i386
RUSTC=rustc
ASM=nasm
CLANG=clang
QEMU=qemu-system-i386

all: floppy.img

.SUFFIXES:

.SUFFIXES: .o .rs .asm

.PHONY: clean run

.rs.o:
	$(RUSTC) -O --target i386-intel-linux --lib -o main.bc --emit-llvm $<
	$(CLANG) -ffreestanding -fno-builtin -fnostdlib -c main.bc -o $@

.asm.o:
	$(ASM) -f elf32 -o $@ $<

main.rs: zero.rs drivers/keyboard.rs drivers/cga.rs

floppy.img: loader.bin main.bin
	cat $^ > $@

loader.bin: loader.asm
	$(ASM) -o $@ -f bin $<

main.bin: linker.ld runtime.o main.o
	$(LD) -o $@ -T $^

run: floppy.img
	$(QEMU) -fda $<

clean:
	rm -f *.bin *.o *.img

debug: linker.ld runtime.o main.o
	$(LD) -o debug.o -T $^ --oformat=default
	$(QEMU) -fda floppy.img -m 32 -s -S &
	gdb -ex 'target remote localhost:1234' -ex 'symbol-file debug.o' -ex 'break main' -ex 'c'
