#![no_std]
#![feature(abi_efiapi)]
#![feature(alloc_error_handler)]

use core::{ffi::c_void, ptr::null};

pub mod console;
mod efi;
pub mod file;
pub mod memory;

extern crate alloc;

pub type Status = efi::STATUS;

pub struct Error {
    status: Status,
    message: &'static str,
}

pub fn initialize(
    system_table: *const c_void,
    image_handle: *const c_void,
    entry: fn() -> Result<(), Error>,
) -> Result<(), Error> {
    let system_table = from_pointer(system_table as *const efi::SYSTEM_TABLE);
    let boot_services = from_pointer(system_table.boot_services);

    // Disable the watchdog timer
    let status = unsafe { (boot_services.set_watchdog_timer)(0, 0, 0, null()) };
    if status != Status::SUCCESS {
        return Err(Error::new(status, "Failed to set watchdog timer"));
    }

    // Initialize the memory
    memory::initialize(boot_services);

    // Initialize the console
    console::initialize(system_table)?;

    // Initialize the file interface
    file::initialize(boot_services, image_handle)?;

    // Enter the program
    entry()
}

fn from_pointer<T>(ptr: *const T) -> &'static T {
    unsafe { &*ptr }
}

fn _from_pointer_mut<T>(ptr: *mut T) -> &'static mut T {
    unsafe { &mut *ptr }
}

impl Error {
    pub fn new(status: Status, message: &'static str) -> Self {
        Error {
            status: status,
            message: message,
        }
    }
}

impl core::fmt::Display for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}: {}", self.message, self.status)
    }
}

impl From<Error> for usize {
    fn from(error: Error) -> Self {
        error.status as usize
    }
}

impl From<Error> for Status {
    fn from(error: Error) -> Self {
        error.status
    }
}
