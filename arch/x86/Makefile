RUSTC=rustc
ASM=nasm
CLANG=clang
CC=gcc -m32
LD=ld -melf_i386
GDB=gdb
OBJCOPY=objcopy
QEMU=qemu-system-i386
TRIPLE=i386-intel-linux

MODS=$(wildcard */*.rs)

all: floppy.img

.PHONY: clean run debug

%.bc: ../../%.rs
	cd ../..; $(RUSTC) -O --target $(TRIPLE) --lib --emit-llvm $*.rs -o arch/x86/$@

%.o: %.bc
	$(CLANG) -ffreestanding -c $^ -o $@ # optimization causes issues!

%.o: %.asm
	$(ASM) -f elf32 -o $@ $<

floppy.img: linker.ld loader.o main.o
	$(LD) -o $@ -T $^

floppy.elf: linker.ld loader.o main.o
	$(LD) -o $@ -T $^ --oformat=default

run: floppy.img
	$(QEMU) -fda $<

clean:
	rm -f *.bin *.o *.img

debug: floppy.elf floppy.img
	$(QEMU) -fda floppy.img -m 32 -s -S &
	$(GDB) -ex 'target remote localhost:1234' -ex 'symbol-file floppy.elf'
