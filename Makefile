-include ./config.mk

arch           ?= x86

# can make the compiler fail. Disable for now
DEBUG          ?=

RUST_ROOT      ?= /usr
LLVM_ROOT      ?= /usr
GCC_PREFIX     ?= /usr/bin/
SHELL          ?= /bin/bash

export DEBUG

export RUST_ROOT
export LLVM_ROOT
export GCC_PREFIX

all:
	@$(MAKE) all -C arch/$(arch)/ SHELL=$(SHELL)

%:
	@$(MAKE) $* -C arch/$(arch)/ SHELL=$(SHELL)
