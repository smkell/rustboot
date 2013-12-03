RUST_ROOT := /usr/local
LLVM_ROOT := /usr
GCC_PREFIX := /usr/bin/

TARGET := i686-intel-linux

-include ./config.mk

RUSTC := $(RUST_ROOT)/bin/rustc
RUSTCFLAGS := -O --target $(TARGET) --lib --emit-llvm

CC := $(LLVM_ROOT)/bin/clang
CFLAGS := -g -O3 -ffreestanding -target $(TARGET)

LD := $(GCC_PREFIX)ld
LDFLAGS := -melf_i386

GDB := $(GCC_PREFIX)gdb
ASM := nasm
OBJCOPY := $(GCC_PREFIX)objcopy
QEMU := qemu-system-i386

MODS := $(wildcard ../../*/*.rs) $(wildcard */*.rs)

all: boot/floppy.img

.PHONY: clean run debug

.DELETE_ON_ERROR:

%.bc: ../../../%.rs $(MODS)
	cd ../..; $(RUSTC) $(RUSTCFLAGS) main.rs -o arch/x86/$@

%.o: %.bc
	$(CC) $(CFLAGS) -c $^ -o $@

%.o: %.asm
	$(ASM) -g -f elf32 -o $@ $<

boot/floppy.img: boot/linker.ld boot/loader.o boot/main.o
	$(LD) $(LDFLAGS) -o $@ -T $^

boot/floppy.elf: boot/linker.ld boot/loader.o boot/main.o
	$(LD) $(LDFLAGS) -o $@ -T $^ --oformat=default

run: boot/floppy.img
	$(QEMU) -fda $<

clean:
	rm -f boot/*.{o,img}

debug: boot/floppy.elf boot/floppy.img
	tmux new-session -d -s rustboot "$(QEMU) -fda boot/floppy.img -m 32 -s -S"
	tmux new-window -t rustboot:1 "$(GDB) -ex 'target remote localhost:1234' -ex 'symbol-file boot/floppy.elf'"
	tmux a -t rustboot
	tmux kill-session -t rustboot
