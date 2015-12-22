LD=ld
RUSTC=rustc
NASM=nasm
QEMU=qemu-system-i386
ARCH=i686
SRC=src

all: floppy.img

.SUFFIXES: .o .rs .asm

.PHONY: clean run test doc

.asm.o:
	$(NASM) -f elf32 -o $@ $<

floppy.img: loader.bin main.bin
	dd if=/dev/zero of=$@ bs=512 count=2 &>/dev/null
	cat $^ | dd if=/dev/stdin of=$@ conv=notrunc &>/dev/null

loader.bin: $(SRC)/arch/$(ARCH)/loader.asm
	$(NASM) -o $@ -f bin $<

main.bin: $(SRC)/arch/$(ARCH)/linker.ld main.o
	$(LD) -m elf_i386 -o $@ -T $^

main.o: src/*.rs
	$(RUSTC) -O --target i686-unknown-linux-gnu --crate-type lib -o $@ --emit obj src/main.rs

run: floppy.img
	$(QEMU) -fda $<

doc: src/*.rs
	rustdoc src/main.rs

clean:
	rm -f *.bin *.o *.img *.png *.ppm

test: floppy.img
	ruby .travis.rb