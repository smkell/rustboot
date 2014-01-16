TARGET := arm-linux-noeabi

GCC_PREFIX := $(GCC_PREFIX)arm-none-eabi-

RUSTC := $(RUST_ROOT)/bin/rustc
RUSTCFLAGS := -O --target $(TARGET) -Z no-landing-pads -Z debug-info

AS := $(GCC_PREFIX)as
LD := $(GCC_PREFIX)ld

LLC := $(LLVM_ROOT)/bin/llc
LLCFLAGS := -march=arm -mcpu=arm926ej-s --float-abi=hard -asm-verbose

GDB := $(GCC_PREFIX)gdb
OBJCOPY := $(GCC_PREFIX)objcopy
QEMU := qemu-system-arm

MODS := $(wildcard */*.rs) $(wildcard ../../*.rs) $(wildcard ../../kernel/*.rs)
BDIR := ./boot
LCORE := libcore-2e829c2f-0.0.rlib
LINK := $(BDIR)/linker.ld $(BDIR)/loader.o $(BDIR)/main.o $(BDIR)/core.o aeabi_ldivmod.o

-include ./config.mk

.PHONY: all run debug clean

all: $(BDIR)/floppy.img

$(BDIR)/$(LCORE): $(wildcard ../../rust-core/core/*.rs)
	$(RUSTC) $(RUSTCFLAGS) ../../rust-core/core/lib.rs --out-dir $(BDIR)

$(BDIR)/core.bc: $(BDIR)/$(LCORE)
	cd $(@D); ar x $(<F) $(@F)

%.bc: ../../../%.rs $(MODS) $(BDIR)/$(LCORE)
	$(RUSTC) $(RUSTCFLAGS) --emit-llvm ../../main.rs --out-dir $(BDIR) -L $(BDIR)

%.s: %.bc
	$(LLC) $(LLCFLAGS) $^ -o $@

%.o: %.s
	$(AS) -c $< -o $@ -g

$(BDIR)/floppy.elf: $(LINK)
	$(LD) -o $@ -T $^

$(BDIR)/floppy.img: $(BDIR)/floppy.elf
	$(OBJCOPY) -O binary $^ $@

run: $(BDIR)/floppy.img
	$(QEMU) -M versatilepb -m 32M -nographic -kernel $^

debug: $(BDIR)/floppy.img
ifeq ($(strip $(TMUX)),)
	tmux new-session -d -s rustboot
	tmux new-window -t rustboot:1 "$(QEMU) -M versatilepb -m 32M -nographic -kernel $^ -s -S"
	tmux split-window -t rustboot "$(GDB) -ex 'target remote localhost:1234' -ex 'symbol-file $(BDIR)/floppy.elf'"
	tmux a -t rustboot
	tmux kill-session -t rustboot
else
	# TODO: debug in current window, can't kill panes
	tmux new-w "$(QEMU) -M versatilepb -m 32M -nographic -kernel $^ -s -S"
	tmux split-w "$(GDB) -ex 'target remote localhost:1234' -ex 'symbol-file $(BDIR)/floppy.elf'; tmux kill-w"
endif

clean:
	rm -f $(BDIR)/*.{o,img,bc,elf,rlib,so,ll}
