#![no_std]
#![no_main]
#![feature(abi_efiapi)]

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
extern "efiapi" fn efi_main() {
    loop {}
}
