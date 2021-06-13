use core::ptr::null;

use crate::efi;

#[repr(C)]
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

static mut LOCATE_PROTOCOL: Option<efi::LOCATE_PROTOCOL> = None;

pub fn initialize(boot_services: &efi::BOOT_SERVICES) {
    unsafe { LOCATE_PROTOCOL = Some(boot_services.locate_protocol) };
}

pub fn get_info() -> Result<GraphicsMode, crate::Error> {
    let mut gop: *const efi::GRAPHICS_OUTPUT_PROTOCOL = null();
    let status = unsafe {
        match LOCATE_PROTOCOL {
            None => {
                return Err(crate::Error::new(
                    efi::STATUS::NOT_READY,
                    "Failed to get graphics information",
                ))
            }
            Some(locate_protocol) => locate_protocol(
                &efi::GRAPHICS_OUTPUT_PROTOCOL_GUID,
                null(),
                &mut gop as *mut *const _ as *mut *const efi::VOID,
            ),
        }
    };
    if status != efi::STATUS::SUCCESS {
        return Err(crate::Error::new(
            status,
            "Failed to get graphics information",
        ));
    }

    let mode = unsafe { &*((*gop).mode) };
    let info = unsafe { &*(mode.info) };

    Ok(GraphicsMode {
        horizontal_resolution: info.horizontal_resolution,
        vertical_resolution: info.vertical_resolution,
        pixel_format: info.pixel_format as u32,
        red_mask: info.pixel_information.red_mask,
        green_mask: info.pixel_information.green_mask,
        blue_mask: info.pixel_information.blue_mask,
        pixels_per_scanline: info.pixels_per_scanline,
        framebuffer: mode.framebuffer_base as *mut u32,
        framebuffer_size: mode.framebuffer_size,
    })
}
