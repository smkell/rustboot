CC=i386-elf-gcc
LD=i386-elf-ld
RUSTC=rustc
NASM=nasm
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
	$(NASM) -f elf32 -o $@ $<

main.rs: zero.rs

floppy.img: loader.bin main.bin
	cat $^ > $@

loader.bin: loader.asm
	$(NASM) -o $@ -f bin $<

main.bin: linker.ld runtime.o main.o
	$(LD) -o $@ -T $^

run: floppy.img
	$(QEMU) -fda $<

clean:
	rm -f *.bin *.o *.img
