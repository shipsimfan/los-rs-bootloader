#![no_std]
#![no_main]
#![feature(asm)]
#![feature(abi_efiapi)]

use core::ffi::c_void;
use uefi::print;

#[no_mangle]
extern "efiapi" fn efi_main(_: *const c_void, system_table: *const c_void) -> usize {
    match uefi::initialize(system_table, main) {
        Ok(()) => 0,
        Err(err) => err.into(),
    }
}

fn main() -> Result<(), uefi::Error> {
    // Load the kernel
    print!("Loading kernel . . . ");

    loop {}
}

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    uefi::eprintln!("PANIC: {}", info);

    loop {}
}
