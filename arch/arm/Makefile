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
OBJS := $(BDIR)/loader.o $(BDIR)/aeabi_runtime.o $(BDIR)/main.o $(BDIR)/core.o
LINK := $(BDIR)/linker.ld $(OBJS) $(BDIR)/initram.elf.embed

-include ./config.mk

.PHONY: all run debug clean

all: $(BDIR)/kernel.bin
	@wc -c $^

$(BDIR)/$(LCORE): $(wildcard ../../rust-core/core/*.rs)
	$(RUSTC) $(RUSTCFLAGS) ../../rust-core/core/lib.rs --out-dir $(BDIR)

$(BDIR)/core.bc: $(BDIR)/$(LCORE)
	cd $(@D); ar x $(<F) $(@F)

$(BDIR)/main.bc: ../../lib.rs $(MODS) $(BDIR)/$(LCORE)
	$(RUSTC) $(RUSTCFLAGS) -L $(BDIR) --emit-llvm ../../lib.rs --out-dir $(BDIR)

%.s: %.bc
	$(LLC) $(LLCFLAGS) $^ -o $@

%.o: %.s
	$(AS) -c $< -o $@ -g

$(BDIR)/kernel.elf: $(LINK)
	$(LD) -o $@ -T $^

$(BDIR)/kernel.bin: $(BDIR)/kernel.elf
	$(OBJCOPY) -O binary $^ $@

$(BDIR)/initram.elf: module.s
	$(AS) $< -o $@

%.embed: %
	$(LD) -r -b binary -o $@ $^

run: $(BDIR)/kernel.bin
	$(QEMU) -M versatilepb -m 32M -nographic -kernel $^

debug: $(BDIR)/kernel.elf
ifeq ($(strip $(TMUX)),)
	tmux new-session -d -s rustboot
	tmux new-window -t rustboot:1 "$(QEMU) -M versatilepb -m 32M -nographic -kernel $^ -s -S"
	tmux split-window -t rustboot "$(GDB)"
	tmux a -t rustboot
	tmux kill-session -t rustboot
else
	# TODO: debug in current window, can't kill panes
	tmux new-w "$(QEMU) -M versatilepb -m 32M -nographic -kernel $^ -s -S"
	tmux split-w "$(GDB); tmux kill-w"
endif

clean:
	rm -f $(BDIR)/*.{o,img,bc,elf,rlib,so,ll}
