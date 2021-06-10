#![no_std]
#![feature(abi_efiapi)]
#![feature(alloc_error_handler)]

use core::{ffi::c_void, ptr::null};

pub mod console;
mod efi;
mod memory;

extern crate alloc;

pub struct Error(efi::STATUS);

pub fn initialize(
    system_table: *const c_void,
    entry: fn() -> Result<(), Error>,
) -> Result<(), Error> {
    let system_table = from_pointer(system_table as *const efi::SYSTEM_TABLE);
    let boot_services = from_pointer(system_table.boot_services);

    // Disable the watchdog timer
    let status = unsafe { (boot_services.set_watchdog_timer)(0, 0, 0, null()) };
    if status != efi::STATUS::SUCCESS {
        return Err(Error(status));
    }

    // Initialize the memory
    memory::initialize(boot_services);

    // Initialize the console
    console::initialize(system_table)?;

    // Enter the program
    entry()
}

fn from_pointer<T>(ptr: *const T) -> &'static T {
    unsafe { &*ptr }
}

fn _from_pointer_mut<T>(ptr: *mut T) -> &'static mut T {
    unsafe { &mut *ptr }
}

impl From<Error> for usize {
    fn from(error: Error) -> Self {
        error.0 as usize
    }
}
