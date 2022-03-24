CXBE=$(NXDK_DIR)/tools/cxbe/cxbe
XISO=$(NXDK_DIR)/tools/extract-xiso/build/extract-xiso

all:
	cargo +nightly build --release
	mkdir -p bin
	@$(CXBE) target/i386-xbox-win32/release/helloworld.exe -OUT:bin/default.xbe
	@$(XISO) -c bin 

clean:
	cargo clean
	rm -r bin
	rm bin.iso

.PHONY: clean
