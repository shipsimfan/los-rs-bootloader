use crate::efi;
use core::{
    alloc::{GlobalAlloc, Layout},
    ptr::null_mut,
};

struct UEFIAllocator {
    allocate: Option<efi::ALLOCATE_POOL>,
    free: Option<efi::FREE_POOL>,
}

#[global_allocator]
static mut ALLOCATOR: UEFIAllocator = UEFIAllocator {
    allocate: None,
    free: None,
};

pub fn initialize(boot_services: &efi::BOOT_SERVICES) {
    unsafe {
        ALLOCATOR.allocate = Some(boot_services.allocate_pool);
        ALLOCATOR.free = Some(boot_services.free_pool);
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
