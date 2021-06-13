use crate::{efi, println};
use core::{
    alloc::{GlobalAlloc, Layout},
    ffi::c_void,
    ptr::null_mut,
};

pub type MemoryDescriptor = efi::MEMORY_DESCRIPTOR;

#[repr(C)]
pub struct MemoryMap {
    pub size: usize,
    pub key: usize,
    pub desc_size: usize,
    pub desc_version: u32,
    pub address: *const MemoryDescriptor,
}

struct UEFIAllocator {
    allocate: Option<efi::ALLOCATE_POOL>,
    free: Option<efi::FREE_POOL>,
    allocate_pages: Option<efi::ALLOCATE_PAGES>,
    copy_mem: Option<efi::COPY_MEM>,
    get_memory_map: Option<efi::GET_MEMORY_MAP>,
}

#[global_allocator]
static mut ALLOCATOR: UEFIAllocator = UEFIAllocator {
    allocate: None,
    free: None,
    allocate_pages: None,
    copy_mem: None,
    get_memory_map: None,
};

pub fn initialize(boot_services: &efi::BOOT_SERVICES) {
    unsafe {
        ALLOCATOR.allocate = Some(boot_services.allocate_pool);
        ALLOCATOR.free = Some(boot_services.free_pool);
        ALLOCATOR.allocate_pages = Some(boot_services.allocate_pages);
        ALLOCATOR.copy_mem = Some(boot_services.copy_mem);
        ALLOCATOR.get_memory_map = Some(boot_services.get_memory_map);
    }
}

pub fn allocate_pages(mem_size: usize, address: efi::PHYSICAL_ADDRESS) -> Result<(), crate::Error> {
    if address % 0x1000 != 0 {
        return Err(crate::Error::new(
            efi::STATUS::NOT_FOUND,
            "Misaligned address for page allocation",
        ));
    }

    let mut address = address;

    unsafe {
        match ALLOCATOR.allocate_pages {
            None => Err(crate::Error::new(
                efi::STATUS::NOT_READY,
                "Allocator not setup",
            )),
            Some(allocate_pages) => {
                let status = allocate_pages(
                    efi::ALLOCATE_TYPE::AllocateAddress,
                    efi::MEMORY_TYPE::LoaderData,
                    (mem_size + 0xFFF) / 0x1000,
                    &mut address,
                );
                match status {
                    efi::STATUS::SUCCESS => Ok(()),
                    _ => Err(crate::Error::new(status, "Failed to allocate pages")),
                }
            }
        }
    }
}

pub fn copy_mem(destination: *mut c_void, source: *const c_void, length: usize) {
    unsafe {
        match ALLOCATOR.copy_mem {
            None => {}
            Some(copy_mem) => copy_mem(destination, source, length),
        }
    }
}

pub fn get_memory_map() -> Result<MemoryMap, crate::Error> {
    unsafe {
        match ALLOCATOR.get_memory_map {
            None => Err(crate::Error::new(
                efi::STATUS::NOT_READY,
                "Failed to get memory map",
            )),
            Some(get_memory_map) => {
                let mut size = 0;
                let mut addr = null_mut();
                let mut key = 0;
                let mut desc_size = 0;
                let mut desc_version = 0;
                let status =
                    get_memory_map(&mut size, addr, &mut key, &mut desc_size, &mut desc_version);
                if status != efi::STATUS::BUFFER_TOO_SMALL {
                    return Err(crate::Error::new(status, "Failed to get memory map"));
                }

                let allocate_pool = ALLOCATOR.allocate.unwrap();
                let status = allocate_pool(
                    efi::MEMORY_TYPE::LoaderData,
                    size,
                    &mut addr as *mut *mut _ as *mut *const _,
                );
                if status != efi::STATUS::SUCCESS {
                    return Err(crate::Error::new(status, "Failed to get memory map"));
                }

                let status =
                    get_memory_map(&mut size, addr, &mut key, &mut desc_size, &mut desc_version);

                match status {
                    efi::STATUS::SUCCESS => Ok(MemoryMap {
                        size: size,
                        key: key,
                        address: addr,
                        desc_size: desc_size,
                        desc_version: desc_version,
                    }),
                    _ => Err(crate::Error::new(status, "Failed to get memory map")),
                }
            }
        }
    }
}

#[alloc_error_handler]
fn alloc_error_handler(layout: alloc::alloc::Layout) -> ! {
    panic!("Allocation error: {:?}", layout)
}

unsafe impl GlobalAlloc for UEFIAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        match self.allocate {
            None => null_mut(),
            Some(allocate) => {
                let mut ret: *const efi::VOID = null_mut();
                match allocate(
                    efi::MEMORY_TYPE::BootServicesData,
                    layout.size(),
                    &mut ret as *mut *const efi::VOID,
                ) {
                    efi::STATUS::SUCCESS => ret as *mut _,
                    _ => null_mut(),
                }
            }
        }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, _: Layout) {
        match self.free {
            None => {}
            Some(free) => {
                free(ptr as *const efi::VOID);
            }
        }
    }
}
