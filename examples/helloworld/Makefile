CXBE=$(NXDK_DIR)/tools/cxbe/cxbe
XISO=$(NXDK_DIR)/tools/extract-xiso/build/extract-xiso

CRATE=helloworld

all:
	$(MAKE) -f $(NXDK_DIR)/Makefile NXDK_ONLY=y
	cargo +nightly build --release
	mkdir -p bin
	@$(CXBE) target/i386-xbox-win32/release/$(CRATE).exe -OUT:bin/default.xbe -TITLE:$(CRATE)
	@$(XISO) -c bin 

clean:
	cargo clean
	rm -r bin
	rm bin.iso

.PHONY: clean
