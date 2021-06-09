TARGET := ./target/x86_64-unknown-uefi/debug/bootloader.efi
SYSROOT_DIR := ../sysroot

all: build

build:
	@cargo build

install: all
	@mkdir -p $(SYSROOT_DIR)/EFI/BOOT
	@cp $(TARGET) $(SYSROOT_DIR)/EFI/BOOT/BOOTX64.EFI
	@echo "[ BOOTLOADER ] Installed!"

clean:
	@cargo clean
	@echo "[ BOOTLOADER ] Cleaned!"

.PHONY: all build install

