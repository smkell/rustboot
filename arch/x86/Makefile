TARGET         := i686-intel-linux

RUSTC          := $(RUST_ROOT)/bin/rustc
RUSTCFLAGS     := -O --target $(TARGET) -Z no-landing-pads -g

CC             := $(LLVM_ROOT)/bin/clang
CFLAGS         := -O3 -ffreestanding -target $(TARGET)

LD             := $(GCC_PREFIX)ld
LDFLAGS        := -melf_i386

ASM            := nasm
ASMFLAGS       := -g -f elf32

GDB            := $(GCC_PREFIX)gdb
OBJCOPY        := $(GCC_PREFIX)objcopy
QEMU           := qemu-system-i386

BDIR           := ./boot

OBJS           := $(BDIR)/loader.o $(BDIR)/main.o
LINK           := $(BDIR)/linker.ld $(OBJS) $(BDIR)/initram.elf.embed
LIBS           :=

SECTIONS       := .text .data .rodata

DEP_RM         := arch\/
DEP_KEEP       := arch\/i686\|arch\/common
DEP_SCRIPT     := 's/\($(DEP_KEEP)\)/\n\1/g;s/ \S*$(DEP_RM)\S*//g;s/\n\($(DEP_KEEP)\)/\1/g'

-include ./config.mk
-include $(BDIR)/loader.d
-include $(BDIR)/main.d

.PHONY: all run debug clean

all: $(BDIR)/floppy.img
	@wc -c $^

# Compile rustboot. Produce dependency info
$(BDIR)/main.bc: ../../lib.rs
	$(RUSTC) $(RUSTCFLAGS) --dep-info -L $(BDIR) --emit bc ../../lib.rs --out-dir $(BDIR)
	@sed -i $(DEP_SCRIPT) $(BDIR)/main.d

%.o: %.bc
	$(CC) $(CFLAGS) -c $^ -o $@

# Assemble loader
%.o: %.asm
	$(ASM) $(ASMFLAGS) -MD $*.d -o $@ $<

# kernel (object)
$(BDIR)/kernel.elf: $(LINK)
	$(LD) $(LDFLAGS) -o $@ -T $^ "-(" $(LIBS) "-)" -Map=./$(BDIR)/linker.map

# initram
$(BDIR)/%.o: %.asm
	$(ASM) $(ASMFLAGS) -MD $(BDIR)/$*.d -o $@ $<

$(BDIR)/initram.elf: $(BDIR)/module.o
	$(LD) $(LDFLAGS) -s $< -o $@

$(BDIR)/%.embed: $(BDIR)/%
	cd $(@D); $(LD) $(LDFLAGS) -r -b binary -o $(@F) $(<F)

# bootloader and kernel separately
$(BDIR)/kernel.bin: $(BDIR)/kernel.elf
	$(OBJCOPY) -O binary $(addprefix -j ,$(SECTIONS)) $^ $@

$(BDIR)/boot.bin: $(BDIR)/kernel.elf
	$(OBJCOPY) -O binary -j .boot $^ $@

# join both
$(BDIR)/floppy.img: $(BDIR)/boot.bin $(BDIR)/kernel.bin
	cat $^ > $@

# running
run: all
	$(QEMU) -fda $(BDIR)/floppy.img

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
	@cat $(BDIR)/.gitignore | xargs -I{} find $(BDIR) -name {} | xargs rm -f
