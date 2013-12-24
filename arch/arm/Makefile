RUST_ROOT := /usr/local
LLVM_ROOT := /usr
GCC_PREFIX := /usr/bin/arm-none-eabi-

TARGET := arm-linux-noeabi

-include ./config.mk

RUSTC := $(RUST_ROOT)/bin/rustc
RUSTCFLAGS := -O --target $(TARGET) -Z no-landing-pads -Z debug-info

AS := $(GCC_PREFIX)as
LD := $(GCC_PREFIX)ld

LLC := $(LLVM_ROOT)/bin/llc
LLCFLAGS := -march=arm -mcpu=arm926ej-s --float-abi=hard -asm-verbose

GDB := $(GCC_PREFIX)gdb
OBJCOPY := $(GCC_PREFIX)objcopy
QEMU := qemu-system-arm

MODS := $(wildcard ../../*/*.rs) $(wildcard */*.rs)
BDIR := ./boot
LCORE := libcore-2e829c2f-0.0.rlib
LINK := $(BDIR)/linker.ld $(BDIR)/loader.o $(BDIR)/main.o $(BDIR)/core.o

all: boot/floppy.img

.PHONY: run debug

.DELETE_ON_ERROR:

$(BDIR)/$(LCORE):
	$(RUSTC) $(RUSTCFLAGS) ../../rust-core/core/lib.rs --out-dir $(BDIR)

$(BDIR)/core.bc: $(BDIR)/$(LCORE)
	cd $(@D); ar x $(<F) $(@F)

%.ll: %.bc
	$(LLVM_ROOT)/bin/llvm-dis $^
	sed -i 's/fixedstacksegment //g;s/arm-unknown-linux-gnueabihf/arm-none-eabi/g;s/nocapture read[a-z]*/nocapture/g' $@

%.ll: ../../../%.rs $(MODS) $(BDIR)/$(LCORE)
	$(RUSTC) $(RUSTCFLAGS) --emit-llvm -S ../../main.rs --out-dir $(BDIR) -L $(BDIR)
	sed -i 's/fixedstacksegment //g;s/arm-unknown-linux-gnueabihf/arm-none-eabi/g;s/nocapture read[a-z]*/nocapture/g' $@

%.s: %.ll
	$(LLC) $(LLCFLAGS) $^ -o $@
	sed -i 's/.note.rustc,"aw"/.note.rustc,"a"/g' $@

%.o: %.s
	$(AS) -c $< -o $@

$(BDIR)/floppy.elf: $(LINK)
	$(LD) -o $@ -T $^

$(BDIR)/floppy.img: $(BDIR)/floppy.elf
	$(OBJCOPY) -O binary $^ $@

run: $(BDIR)/floppy.img
	$(QEMU) -M versatilepb -m 32M -nographic -kernel $^

debug: $(BDIR)/floppy.img
	tmux new-session -d -s rustboot
	tmux new-window -t rustboot:1 "$(QEMU) -M versatilepb -m 32M -nographic -kernel $^ -s -S"
	tmux split-window -t rustboot "$(GDB) -ex 'target remote localhost:1234' -ex 'symbol-file $(BDIR)/floppy.elf'"
	tmux a -t rustboot
	tmux kill-session -t rustboot

clean:
	rm -f $(BDIR)/*.{o,img,bc,elf,rlib,so,ll}
