#![no_std]
#![no_main]

use core::ffi::c_void;
use uefi::{exit_boot_services, print, println};

extern crate alloc;

mod elf;

type KernelEntry = extern "efiapi" fn(
    graphics_info: *const uefi::graphics::GraphicsMode,
    memory_map: *const uefi::memory::MemoryMap,
    rdsp: *const c_void,
);

#[no_mangle]
extern "efiapi" fn efi_main(image_handle: *const c_void, system_table: *const c_void) -> usize {
    match uefi::initialize(system_table, image_handle, main) {
        Ok(()) => 0,
        Err(err) => {
            println!("\r\nFATAL ERROR: {}", err);
            err.into()
        }
    }
}

fn main() -> Result<(), uefi::Error> {
    // Load the kernel
    print!("Loading kernel . . . ");
    let entry: KernelEntry = {
        let kernel = uefi::file::load_file("kernel.elf")?;
        unsafe { core::mem::transmute(elf::load_executable(&kernel)?) }
    };
    println!("OK!");

    // Get the graphics mode info
    print!("Getting video mode information . . . ");
    let graphics_info = uefi::graphics::get_info()?;
    println!("OK!");

    // Get the ACPI RSDP
    print!("Getting the ACPI RSDP . . . ");
    let rsdp = uefi::config_table::get_config_table(uefi::config_table::ACPI_20_RSDP_GUID)?;
    println!("OK!");

    // Get memory info
    print!("Getting memory information . . . ");
    let mmap = uefi::memory::get_memory_map()?;

    exit_boot_services(mmap.key)?;

    entry(&graphics_info, &mmap, rsdp);

    loop {
        unsafe { core::arch::asm!("hlt") };
    }
}

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    uefi::println!("{}", info);

    loop {
        unsafe { core::arch::asm!("hlt") };
    }
}
