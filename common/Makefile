-include ./config.mk
-include $(BDIR)/main.d

# need to create new variable due to semantics of make "recursive variables"
GCC_PREFIX_2   ?= $(GCC_PREFIX)

RUSTC          ?= $(RUST_ROOT)/bin/rustc
RUSTCFLAGS     ?= -O --target $(TARGET) -Z no-landing-pads -Z lto

# CC is probably defined (as GCC)
CC              = $(LLVM_ROOT)/bin/clang
CFLAGS         ?= -O3 -ffreestanding -target $(TARGET)

LLC            ?= $(LLVM_ROOT)/bin/llc

# AS is probably defined
AS              = $(GCC_PREFIX_2)as
LD             ?= $(GCC_PREFIX_2)ld
GDB            ?= $(GCC_PREFIX_2)gdb
OBJCOPY        ?= $(GCC_PREFIX_2)objcopy

BDIR           ?= ./boot

DEP_SCRIPT     ?= 's~\($(DEP_KEEP)\)~\n\1~g;s~ \S*$(DEP_RM)\S*~~g;s~\n\($(DEP_KEEP)\)~\1~g'


# Compile rustboot. Produce dependency info
$(BDIR)/main.bc: ../../common/lib.rs
	$(RUSTC) $(RUSTCFLAGS) -L $(BDIR) --dep-info --emit=bc $< --out-dir $(BDIR)
	@sed -i $(DEP_SCRIPT) $(BDIR)/main.d

# initram

$(BDIR)/initram.elf: $(BDIR)/module.o
	$(LD) $(LDFLAGS) -s $< -o $@

$(BDIR)/%.embed: $(BDIR)/%
	cd $(@D); $(LD) $(LDFLAGS) -r -b binary -o $(@F) $(<F)

.PHONY: clean
clean:
	@cat $(BDIR)/.gitignore | xargs -I{} find $(BDIR) -name {} | xargs rm -f
