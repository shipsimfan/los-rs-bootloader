#![no_std]
#![no_main]
#![feature(asm)]
#![feature(abi_efiapi)]

use core::ffi::c_void;
use uefi::{print, println};

extern crate alloc;

mod elf;

type KernelEntry = extern "C" fn(
    graphics_info: *const GraphicsMode,
    memory_map: *const MemoryMap,
    rdsp: *const c_void,
);

#[repr(packed(1))]
pub struct GraphicsMode {
    pub horizontal_resolution: u32,
    pub vertical_resolution: u32,
    pub pixel_format: u32,
    pub red_mask: u32,
    pub green_mask: u32,
    pub blue_mask: u32,
    pub pixels_per_scanline: u32,
    pub framebuffer: *mut u32,
    pub framebuffer_size: usize,
}

#[repr(packed(1))]
pub struct MemoryMap {
    pub size: usize,
    pub key: usize,
    pub desc_size: usize,
    pub desc_version: u32,
    pub address: *const uefi::memory::MemoryDescriptor,
}

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
    let _entry: KernelEntry = {
        let kernel = uefi::file::load_file("kernel.elf")?;
        unsafe { core::mem::transmute(elf::load_executable(&kernel)?) }
    };
    println!("OK!");

    loop {}
}

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    uefi::println!("{}", info);

    loop {}
}
