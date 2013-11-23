RUST_ROOT := /usr/local
LLVM_ROOT := /usr
GCC_PREFIX := /usr/bin/

TARGET := i386-intel-linux

-include ./config.mk

RUSTC := $(RUST_ROOT)/bin/rustc
RUSTCFLAGS := -O --target $(TARGET) --lib --emit-llvm

CC := $(LLVM_ROOT)/bin/clang
CFLAGS := -g -ffreestanding

LD := $(GCC_PREFIX)ld
LDFLAGS := -melf_i386

GDB := $(GCC_PREFIX)gdb
ASM := nasm
OBJCOPY := $(GCC_PREFIX)objcopy
QEMU := qemu-system-i386

MODS := $(wildcard ../../*/*.rs) $(wildcard */*.rs)

all: floppy.img

.PHONY: clean run debug

.DELETE_ON_ERROR:

%.bc: ../../%.rs $(MODS)
	cd ../..; $(RUSTC) $(RUSTCFLAGS) $*.rs -o arch/x86/$@

%.o: %.bc
	$(CC) -c $^ -o $@

%.o: %.asm
	$(ASM) -g -f elf32 -o $@ $<

floppy.img: linker.ld loader.o main.o
	$(LD) $(LDFLAGS) -o $@ -T $^

floppy.elf: linker.ld loader.o main.o
	$(LD) $(LDFLAGS) -o $@ -T $^ --oformat=default

run: floppy.img
	$(QEMU) -fda $<

clean:
	rm -f *.bin *.o *.img

debug: floppy.elf floppy.img
	tmux new-session -d -s rustboot "$(QEMU) -fda floppy.img -m 32 -s -S"
	tmux new-window -t rustboot:1 "$(GDB) -ex 'target remote localhost:1234' -ex 'symbol-file floppy.elf'"
	tmux a -t rustboot
	tmux kill-session -t rustboot
