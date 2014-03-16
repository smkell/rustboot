arch=x86
RUST_ROOT := /usr
LLVM_ROOT := /usr
GCC_PREFIX := /usr/bin/
SHELL := /bin/bash

export RUST_ROOT
export LLVM_ROOT
export GCC_PREFIX

all:
	@$(MAKE) all -C arch/$(arch)/ SHELL=$(SHELL)

%:
	@$(MAKE) $* -C arch/$(arch)/ SHELL=$(SHELL)
