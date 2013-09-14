arch=x86

%:
	$(MAKE) $* -C arch/$(arch)/
