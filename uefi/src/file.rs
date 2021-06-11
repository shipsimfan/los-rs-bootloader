use crate::efi::{self, CHAR16};
use alloc::{vec, vec::Vec};
use core::ptr::{null, null_mut};

static mut BOOT_VOLUME: Option<*const efi::FILE_PROTOCOL> = None;
static mut ALLOCATION_TYPE: efi::MEMORY_TYPE = efi::MEMORY_TYPE::ReservedMemoryType;

pub fn initialize(
    boot_services: &efi::BOOT_SERVICES,
    image_handle: efi::HANDLE,
) -> Result<(), crate::Error> {
    // Get our loaded image
    let mut loaded_image: *const efi::LOADED_IMAGE_PROTOCOL = null();
    let status = unsafe {
        (boot_services.handle_protocol)(
            image_handle,
            &efi::LOADED_IMAGE_PROTOCOL_GUID,
            &mut loaded_image as *mut *const _ as *mut *const efi::VOID,
        )
    };
    if status != efi::STATUS::SUCCESS {
        return Err(crate::Error::new(status, "Failed to get loaded image"));
    }

    // Get the file system protocol
    let mut simple_file_system: *const efi::SIMPLE_FILE_SYSTEM_PROTOCOL = null();
    let status = unsafe {
        (boot_services.handle_protocol)(
            (*loaded_image).device_handle,
            &efi::SIMPLE_FILE_SYSTEM_PROTOCOL_GUID,
            &mut simple_file_system as *mut *const _ as *mut *const efi::VOID,
        )
    };
    if status != efi::STATUS::SUCCESS {
        return Err(crate::Error::new(status, "Failed to get file system"));
    }

    let mut bv: *const efi::FILE_PROTOCOL = null();
    let status = unsafe {
        ((*simple_file_system).open_volume)(simple_file_system, &mut bv as *mut *const _)
    };
    if status != efi::STATUS::SUCCESS {
        return Err(crate::Error::new(status, "Failed to get boot volume"));
    }

    unsafe {
        BOOT_VOLUME = Some(crate::from_pointer(bv));
        ALLOCATION_TYPE = (*loaded_image).image_data_type;
    }

    Ok(())
}

pub fn load_file(path: &str) -> Result<Vec<u8>, crate::Error> {
    // Get the boot volume
    let boot_volume = unsafe {
        match BOOT_VOLUME {
            None => {
                return Err(crate::Error::new(
                    efi::STATUS::NOT_READY,
                    "Failed to open file",
                ))
            }
            Some(boot_volume) => boot_volume,
        }
    };

    // Convert the filename
    let mut wpath = Vec::with_capacity(path.len() + 1);
    for c in path.chars() {
        wpath.push(c as CHAR16);
    }
    wpath.push(0);

    // Open the file
    let file_handle = open(boot_volume, &wpath)?;

    // Read the file
    let data = read(file_handle)?;

    // Close the file
    close(boot_volume, file_handle)?;

    Ok(data)
}

fn open(
    boot_volume: *const efi::FILE_PROTOCOL,
    name: &Vec<CHAR16>,
) -> Result<*const efi::FILE_PROTOCOL, crate::Error> {
    let mut ret = null();
    let status = unsafe {
        ((*boot_volume).open)(
            boot_volume,
            &mut ret,
            name.as_ptr(),
            efi::FILE_MODE_READ,
            efi::FILE_READ_ONLY | efi::FILE_HIDDEN | efi::FILE_SYSTEM,
        )
    };
    if status != efi::STATUS::SUCCESS {
        Err(crate::Error::new(status, "Failed to open file"))
    } else {
        Ok(ret)
    }
}

fn close(
    boot_volume: *const efi::FILE_PROTOCOL,
    handle: *const efi::FILE_PROTOCOL,
) -> Result<(), crate::Error> {
    let status = unsafe { ((*boot_volume).close)(handle) };
    if status != efi::STATUS::SUCCESS {
        Err(crate::Error::new(status, "Failed to close file"))
    } else {
        Ok(())
    }
}

fn read(handle: *const efi::FILE_PROTOCOL) -> Result<Vec<u8>, crate::Error> {
    let mut file_info_size: efi::UINTN = 0;
    let status = unsafe {
        ((*handle).get_info)(handle, &efi::FILE_INFO_ID, &mut file_info_size, null_mut())
    };
    if status != efi::STATUS::BUFFER_TOO_SMALL {
        return Err(crate::Error::new(status, "Failed to get file info size"));
    }

    let file_info: Vec<u8> = Vec::with_capacity(file_info_size);
    let status = unsafe {
        ((*handle).get_info)(
            handle,
            &efi::FILE_INFO_ID,
            &mut file_info_size,
            file_info.as_ptr() as *mut efi::VOID,
        )
    };
    if status != efi::STATUS::SUCCESS {
        return Err(crate::Error::new(status, "Failed to get file info"));
    }

    let mut file_size: efi::UINTN =
        unsafe { (*(file_info.as_ptr() as *const efi::FILE_INFO)).file_size } as efi::UINTN;

    let mut data = vec![0; file_size];

    let status =
        unsafe { ((*handle).read)(handle, &mut file_size, data.as_mut_ptr() as *mut efi::VOID) };
    match status {
        efi::STATUS::SUCCESS => Ok(data),
        _ => Err(crate::Error::new(status, "Failed to read file")),
    }
}
