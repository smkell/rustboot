TARGET := i686-intel-linux

RUSTC := $(RUST_ROOT)/bin/rustc
RUSTCFLAGS := -O --target $(TARGET) -Z no-landing-pads -g

CC := $(LLVM_ROOT)/bin/clang
CFLAGS := -O3 -ffreestanding -target $(TARGET)

LD := $(GCC_PREFIX)ld
LDFLAGS := -melf_i386

ASM := nasm
ASMFLAGS := -g -f elf32

GDB := $(GCC_PREFIX)gdb
OBJCOPY := $(GCC_PREFIX)objcopy
QEMU := qemu-system-i386

BDIR := ./boot
CORE_LIB := ../../rust-core/core/lib.rs
LCORE := $(BDIR)/$(shell $(RUSTC) --crate-file-name $(CORE_LIB))
OBJS := $(BDIR)/loader.o $(BDIR)/main.o
DEPS := $(BDIR)/linker.ld $(OBJS) $(BDIR)/initram.elf.embed
LINK := "-(" $(LCORE) "-)"

# TODO: fix dep-info target
MODS := $(wildcard *.rs) $(wildcard */*.rs) $(wildcard ../../kernel/*.rs) $(wildcard ../../kernel/*/*.rs) $(wildcard ../../kernel/*/*/*.rs)

SECTIONS := .text .data .rodata

-include ./config.mk
-include $(BDIR)/core.d
-include $(BDIR)/loader.d

.PHONY: all run debug clean

all: $(BDIR)/floppy.img
	@wc -c $^

# Library rust-core
$(LCORE): $(CORE_LIB)
	$(RUSTC) $(RUSTCFLAGS) --dep-info $(@D)/core.d $(CORE_LIB) --out-dir $(BDIR)

# Compile rustboot
$(BDIR)/main.bc: ../../lib.rs $(MODS) $(LCORE)
	$(RUSTC) $(RUSTCFLAGS) --dep-info $(@D)/main.d -L $(BDIR) --emit bc ../../lib.rs --out-dir $(BDIR)

%.o: %.bc
	$(CC) $(CFLAGS) -c $^ -o $@

# Assemble loader
%.o: %.asm
	$(ASM) $(ASMFLAGS) -MD $*.d -o $@ $<

# kernel (object)
$(BDIR)/kernel.elf: $(DEPS)
	$(LD) $(LDFLAGS) -o $@ -T $^ $(LINK) -Map=./$(BDIR)/linker.map

# initram
$(BDIR)/%.o: %.asm
	$(ASM) $(ASMFLAGS) -MD $(BDIR)/$*.d -o $@ $<

$(BDIR)/initram.elf: $(BDIR)/module.o
	$(LD) $(LDFLAGS) -s $< -o $@

%.embed: %
	$(LD) $(LDFLAGS) -r -b binary -o $@ $^

# bootloader and kernel separately
$(BDIR)/kernel.bin: $(BDIR)/kernel.elf
	$(OBJCOPY) -O binary $(addprefix -j ,$(SECTIONS)) $^ $@

$(BDIR)/boot.bin: $(BDIR)/kernel.elf
	$(OBJCOPY) -O binary -j .boot $^ $@

# join both
$(BDIR)/floppy.img: $(BDIR)/boot.bin $(BDIR)/kernel.bin
	cat $^ > $@

# running
run: boot/floppy.img
	$(QEMU) -fda $<

debug: $(BDIR)/kernel.elf $(BDIR)/floppy.img
ifeq ($(strip $(TMUX)),)
	tmux new-session -d -s rustboot "$(QEMU) -fda $(BDIR)/floppy.img -m 32 -s -S"
	tmux new-window -t rustboot:1 "$(GDB)"
	tmux a -t rustboot
	tmux kill-session -t rustboot
else
	tmux split-w "$(GDB); tmux kill-p"
	$(QEMU) -fda $(BDIR)/floppy.img -m 32 -s -S
endif

clean:
	rm -f $(BDIR)/*.{d,o,bc,rlib,so,ll,embed,elf,bin,img,map}
