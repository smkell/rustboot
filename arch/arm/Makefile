RUST_ROOT := /usr/local
LLVM_ROOT := /usr
GCC_PREFIX := /usr/bin/arm-none-eabi-

TARGET := arm-linux-noeabi

-include ./config.mk

RUSTC := $(RUST_ROOT)/bin/rustc
RUSTCFLAGS := -O --target $(TARGET) --lib -S --emit-llvm

AS := $(GCC_PREFIX)as
CC := $(GCC_PREFIX)gcc
CFLAGS := -g -ffreestanding -nostdlib -O3 -Wall

LLC := $(LLVM_ROOT)/bin/llc
LLCFLAGS := -march=arm -mcpu=arm926ej-s --float-abi=hard -asm-verbose

GDB := $(GCC_PREFIX)gdb
OBJCOPY := $(GCC_PREFIX)objcopy
QEMU := qemu-system-arm

MODS := $(wildcard ../../*/*.rs) $(wildcard */*.rs)

all: boot/floppy.img

.PHONY: run debug

.DELETE_ON_ERROR:

%.ll: ../../../%.rs $(MODS)
	cd ../..; $(RUSTC) $(RUSTCFLAGS) main.rs -o arch/arm/$*.ll
	sed -i 's/fixedstacksegment //g' $@
	sed -i 's/arm-unknown-linux-gnueabihf/arm-none-eabi/g' $@
	sed -i 's/nocapture read[a-z]*/nocapture/g' $@

%.s: %.ll
	$(LLC) $(LLCFLAGS) $^ -o $@
	sed -i 's/.note.rustc,"aw"/.note.rustc,"a"/g' $@

%.o: %.s
	$(AS) -c $< -o $@

boot/floppy.elf: boot/linker.ld boot/loader.o boot/main.o
	$(CC) $(CFLAGS) -o $@ -T $^

boot/floppy.img: boot/floppy.elf
	$(OBJCOPY) -O binary $^ $@

run: boot/floppy.img
	$(QEMU) -M versatilepb -m 32M -nographic -kernel $^

debug: boot/floppy.img
	tmux new-session -d -s rustboot
	tmux new-window -t rustboot:1 "$(QEMU) -M versatilepb -m 32M -nographic -kernel $^ -s -S"
	tmux split-window -t rustboot "$(GDB) -ex 'target remote localhost:1234' -ex 'symbol-file boot/floppy.elf'"
	tmux a -t rustboot
	tmux kill-session -t rustboot
