arch=x86

all:
	$(MAKE) all -C arch/$(arch)/

%:
	$(MAKE) $* -C arch/$(arch)/
