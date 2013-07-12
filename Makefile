CC=gcc -m32
LD=ld -melf_i386
RUSTC=rustc
ASM=nasm
CLANG=clang
QEMU=qemu-system-i386
MODS=$(wildcard */*.rs) idt.rs

all: floppy.img

.PHONY: clean run debug

%.o: %.rs $(MODS)
	$(RUSTC) -O --target i386-intel-linux --lib -o $*.bc --emit-llvm $<
	$(CLANG) -ffreestanding -c $*.bc -o $@ # optimization causes issues!

%.o: %.asm
	$(ASM) -f elf32 -o $@ $<

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
