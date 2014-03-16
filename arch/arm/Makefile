TARGET := arm-linux-noeabi

GCC_PREFIX := $(GCC_PREFIX)arm-none-eabi-

RUSTC := $(RUST_ROOT)/bin/rustc
RUSTCFLAGS := -O --target $(TARGET) -Z no-landing-pads -g

AS := $(GCC_PREFIX)as
LD := $(GCC_PREFIX)ld

LLC := $(LLVM_ROOT)/bin/llc
LLCFLAGS := -mtriple=$(TARGET) -march=arm -mcpu=arm926ej-s --float-abi=hard -asm-verbose

GDB := $(GCC_PREFIX)gdb
OBJCOPY := $(GCC_PREFIX)objcopy
QEMU := qemu-system-arm

BDIR := ./boot
CORE_LIB := ../../rust-core/core/lib.rs
LCORE := $(BDIR)/$(shell $(RUSTC) --crate-file-name $(CORE_LIB))

OBJS := $(BDIR)/loader.o $(BDIR)/aeabi_runtime.o $(BDIR)/main.o $(BDIR)/core.o
LINK := $(BDIR)/linker.ld $(OBJS) $(BDIR)/initram.elf.embed

DEP_RM := arch\/
DEP_KEEP := arch\/arm
DEP_SCRIPT := 's/\($(DEP_KEEP)\)/\n\1/g;s/ \S*$(DEP_RM)\S*//g;s/\n\($(DEP_KEEP)\)/\1/g'

-include ./config.mk
-include $(BDIR)/core.d
-include $(BDIR)/main.d

.PHONY: all run debug clean

all: $(BDIR)/kernel.bin
	@wc -c $^

# Library rust-core
$(LCORE) $(BDIR)/core.bc:
	$(RUSTC) $(RUSTCFLAGS) $(CORE_LIB) --dep-info --emit=bc,link --out-dir $(BDIR)

# Compile rustboot. Produce dependency info
$(BDIR)/main.bc: $(LCORE)
	$(RUSTC) $(RUSTCFLAGS) -L $(BDIR) --dep-info --emit=bc ../../lib.rs --out-dir $(BDIR)
	@sed -i $(DEP_SCRIPT) $(BDIR)/main.d

%.s: %.bc
	$(LLC) $(LLCFLAGS) $^ -o $@

# Assemble object files. note: do not overwrite deps with -MD $*.d
%.o: %.s
	$(AS) -g $< -o $@

# initram
$(BDIR)/%.o: %.s
	$(AS) $< -o $@

$(BDIR)/initram.elf: $(BDIR)/module.o
	$(LD) -s -o $@ $^

$(BDIR)/%.embed: $(BDIR)/%
	cd $(@D); $(LD) $(LDFLAGS) -r -b binary -o $(@F) $(<F)

# kernel (object)
$(BDIR)/kernel.elf: $(LINK)
	$(LD) -o $@ -T $^

# kernel (binary image)
$(BDIR)/kernel.bin: $(BDIR)/kernel.elf
	$(OBJCOPY) -O binary $^ $@

# running
run: all
	$(QEMU) -M versatilepb -m 32M -nographic -kernel $(BDIR)/kernel.bin

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
	@cat $(BDIR)/.gitignore | xargs -I{} find $(BDIR) -name {} | xargs rm -f
