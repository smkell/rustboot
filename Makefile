LD=ld
RUSTC=rustc
NASM=nasm
QEMU=qemu-system-i386
ARCH=i686
SRC=src

RUSTC_FLAGS := -O 								# Optimize to opt-level=2
RUSTC_FLAGS += --target i686-unknown-linux-gnu 	# Set the target triple
RUSTC_FLAGS += --crate-type lib					# Output a library crate (don't link)
RUSTC_FLAGS += --emit obj 						# Emit object files 
RUSTC_FLAGS += -C relocation-model=static		# Don't add a global offset table

all: floppy.img doc

.SUFFIXES: .o .rs .asm

.PHONY: clean run test doc

.asm.o:
	$(NASM) -f elf32 -o $@ $<

floppy.img: loader.bin main.bin
	dd if=/dev/zero of=$@ bs=512 count=2 &>/dev/null
	cat $^ | dd if=/dev/stdin of=$@ conv=notrunc &>/dev/null

loader.bin: $(SRC)/arch/$(ARCH)/loader.asm
	$(NASM) -o $@ -f bin $<

main.bin: $(SRC)/arch/$(ARCH)/linker.ld nepheliad.o
	$(LD) -m elf_i386 -o $@ -T $^

nepheliad.o: src/*.rs
	$(RUSTC) $(RUSTC_FLAGS) -o $@ src/nepheliad.rs

run: floppy.img
	$(QEMU) -fda $<

doc/nepheliad/index.html: src/*.rs
	rustdoc src/nepheliad.rs
	
doc: doc/nepheliad/index.html
	

clean:
	rm -f *.bin *.o *.img *.png *.ppm
	rm -rf doc

test: floppy.img
	ruby .travis.rb