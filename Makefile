CC=gcc -m32
LD=ld -melf_i386
RUSTC=rustc
ASM=nasm
CLANG=clang
QEMU=qemu-system-i386
MODS=$(wildcard */*.rs)

all: floppy.img

.PHONY: clean run debug

%.o: %.rs $(MODS)
	$(RUSTC) -O --target i386-intel-linux --lib -o $*.bc --emit-llvm $<
	$(CLANG) -ffreestanding -c $*.bc -o $@ # optimization causes issues!

%.o: %.asm
	$(ASM) -f elf32 -o $@ $<

floppy.img: linker.ld loader.o main.o
	$(LD) -o $@ -T $^

run: floppy.img
	$(QEMU) -fda $<

arm: arch/arm
	$(RUSTC) --opt-level=0 --target arm-linux-noeabi --lib -c main.rs -S -o main.ll --emit-llvm -A unused-imports
	sed -i 's/fixedstacksegment //g' main.ll
	sed -i 's/arm-unknown-linux-gnueabihf/arm-none-eabi/g' main.ll
	llc -march=arm -mcpu=arm926ej-s --float-abi=hard -asm-verbose main.ll -o=main.s
	sed -i 's/.note.rustc,"aw"/.note.rustc,"a"/g' main.s
	cd arch/arm; make

clean:
	rm -f *.bin *.o *.img

debug: linker.ld loader.o main.o
	$(LD) -o debug.o -T $^ --oformat=default
	$(QEMU) -fda floppy.img -m 32 -s -S &
	gdb -ex 'target remote localhost:1234' -ex 'symbol-file debug.o' -ex 'break main' -ex 'c'
