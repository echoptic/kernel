BUILD_DIR=build
KERNEL=$(BUILD_DIR)/kernel
ISO_IMAGE=$(BUILD_DIR)/disk.iso

.PHONY: all limine kernel clean run

all: $(ISO_IMAGE)

limine:
	make -C limine

kernel:
	cargo build \
		--out-dir=$(BUILD_DIR) -Z unstable-options

$(ISO_IMAGE): limine kernel
	rm -rf iso_root
	mkdir -p iso_root
	cp $(KERNEL) \
		limine.cfg limine/limine.sys limine/limine-cd.bin limine/limine-eltorito-efi.bin iso_root/
	xorriso -as mkisofs -b limine-cd.bin \
		-no-emul-boot -boot-load-size 4 -boot-info-table \
		--efi-boot limine-eltorito-efi.bin \
		-efi-boot-part --efi-boot-image --protective-msdos-label \
		iso_root -o $(ISO_IMAGE)
	limine/limine-install $(ISO_IMAGE)
	rm -rf iso_root

run: all
	qemu-system-x86_64 -serial stdio -m 2G -net none -drive format=raw,file=$(ISO_IMAGE)

clean:
	rm -rf $(BUILD_DIR)
