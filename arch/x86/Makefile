TARGET := i686-intel-linux

RUSTC := $(RUST_ROOT)/bin/rustc
RUSTCFLAGS := -O --target $(TARGET) -Z no-landing-pads -Z debug-info

CC := $(LLVM_ROOT)/bin/clang
CFLAGS := -O3 -ffreestanding -target $(TARGET)

LD := $(GCC_PREFIX)ld
LDFLAGS := -melf_i386

GDB := $(GCC_PREFIX)gdb
ASM := nasm
OBJCOPY := $(GCC_PREFIX)objcopy
QEMU := qemu-system-i386

MODS := $(wildcard */*.rs) $(wildcard ../../*.rs) $(wildcard ../../kernel/*.rs)
BDIR := ./boot
LCORE := libcore-2e829c2f-0.0.rlib
OBJS := $(BDIR)/loader.o $(BDIR)/main.o $(BDIR)/runtime.o
LINK := $(BDIR)/linker.ld $(OBJS) "-(" $(BDIR)/$(LCORE) "-)"

SECTIONS := .text .data .rodata

-include ./config.mk

.PHONY: all run debug clean

all: $(BDIR)/floppy.img
	@wc -c $^

$(BDIR)/$(LCORE): $(wildcard ../../rust-core/core/*.rs)
	$(RUSTC) $(RUSTCFLAGS) ../../rust-core/core/lib.rs --out-dir $(BDIR)

%.bc: ../../../%.rs $(MODS) $(BDIR)/$(LCORE)
	$(RUSTC) $(RUSTCFLAGS) --emit-llvm ../../main.rs --out-dir $(BDIR) -L $(BDIR)

%.o: %.bc
	$(CC) $(CFLAGS) -c $^ -o $@

%.o: %.asm
	$(ASM) -g -f elf32 -o $@ $<

$(BDIR)/floppy.elf: $(OBJS)
	$(LD) $(LDFLAGS) -o $@ -T $(LINK) -Map=./$(BDIR)/linker.map

$(BDIR)/kernel.bin: $(BDIR)/floppy.elf
	$(OBJCOPY) -O binary $(addprefix -j ,$(SECTIONS)) $^ $@

$(BDIR)/boot.bin: $(BDIR)/floppy.elf
	$(OBJCOPY) -O binary -j .boot $^ $@

$(BDIR)/%.o: %.asm
	$(ASM) -f elf32 -o $@ $<

$(BDIR)/initram.elf: $(BDIR)/module.o
	$(LD) $(LDFLAGS) -s $< -o $@

$(BDIR)/floppy.img: $(BDIR)/boot.bin $(BDIR)/kernel.bin $(BDIR)/initram.elf
	cat $^ > $@

run: boot/floppy.img
	$(QEMU) -fda $<

debug: $(BDIR)/floppy.elf $(BDIR)/floppy.img
ifeq ($(strip $(TMUX)),)
	tmux new-session -d -s rustboot "$(QEMU) -fda $(BDIR)/floppy.img -m 32 -s -S"
	tmux new-window -t rustboot:1 "$(GDB) -ex 'target remote localhost:1234' -ex 'symbol-file $(BDIR)/floppy.elf'"
	tmux a -t rustboot
	tmux kill-session -t rustboot
else
	tmux split-w "$(GDB) -ex 'target remote localhost:1234' -ex 'symbol-file $(BDIR)/floppy.elf'; tmux kill-p"
	$(QEMU) -fda $(BDIR)/floppy.img -m 32 -s -S
endif

clean:
	rm -f $(BDIR)/*.{o,img,bc,elf,rlib,so}
