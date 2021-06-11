use core::ffi::c_void;

use alloc::vec::Vec;

type Elf64Addr = u64;
type Elf64Half = u16;
type Elf64Off = u64;
type Elf64Word = u32;
type Elf64XWord = u64;

const EI_MAG0: usize = 0;
const EI_MAG1: usize = 1;
const EI_MAG2: usize = 2;
const EI_MAG3: usize = 3;
const EI_CLASS: usize = 4;
const EI_DATA: usize = 5;
const EI_VERSION: usize = 6;
const EI_NIDENT: usize = 16;

const ELFMAG0: u8 = 0x7F;
const ELFMAG1: u8 = 'E' as u8;
const ELFMAG2: u8 = 'L' as u8;
const ELFMAG3: u8 = 'F' as u8;

const ELFCLASS64: u8 = 2;

const ELFDATA2LSB: u8 = 1;

const EV_CURRENT: u8 = 1;

const ET_EXEC: Elf64Half = 2;

const EM_AMD64: Elf64Half = 62;

const PT_LOAD: Elf64Word = 1;

#[repr(C)]
struct Elf64Ehdr {
    pub e_ident: [u8; EI_NIDENT],
    pub e_type: Elf64Half,
    pub e_machine: Elf64Half,
    pub e_version: Elf64Word,
    pub e_entry: Elf64Addr,
    pub e_phoff: Elf64Off,
    pub e_shoff: Elf64Off,
    pub e_flags: Elf64Word,
    pub e_ehsize: Elf64Half,
    pub e_phentsize: Elf64Half,
    pub e_phnum: Elf64Half,
    pub e_shentsize: Elf64Half,
    pub e_shnum: Elf64Half,
    pub e_shstrndx: Elf64Half,
}

#[repr(C)]
struct Elf64Phdr {
    pub p_type: Elf64Word,
    pub p_flags: Elf64Word,
    pub p_offset: Elf64Off,
    pub p_vaddr: Elf64Addr,
    pub p_paddr: Elf64Addr,
    pub p_filesz: Elf64XWord,
    pub p_memsz: Elf64XWord,
    pub p_align: Elf64XWord,
}

pub fn load_executable(file: &Vec<u8>) -> Result<usize, uefi::Error> {
    let hdr = unsafe { &*(file.as_ptr() as *const Elf64Ehdr) };

    // Check ELF MAG
    if hdr.e_ident[EI_MAG0] != ELFMAG0
        || hdr.e_ident[EI_MAG1] != ELFMAG1
        || hdr.e_ident[EI_MAG2] != ELFMAG2
        || hdr.e_ident[EI_MAG3] != ELFMAG3
    {
        return Err(uefi::Error::new(
            uefi::Status::COMPROMISED_DATA,
            "Invalid ELF MAG",
        ));
    }

    // Verify class
    if hdr.e_ident[EI_CLASS] != ELFCLASS64 {
        return Err(uefi::Error::new(uefi::Status::UNSUPPORTED, "Invalid class"));
    }

    // Verify data order
    if hdr.e_ident[EI_DATA] != ELFDATA2LSB {
        return Err(uefi::Error::new(
            uefi::Status::UNSUPPORTED,
            "Invalid data order",
        ));
    }

    // Verify version
    if hdr.e_ident[EI_VERSION] != EV_CURRENT || hdr.e_version != EV_CURRENT as u32 {
        return Err(uefi::Error::new(
            uefi::Status::UNSUPPORTED,
            "Invalid version",
        ));
    }

    // Verify type
    if hdr.e_type != ET_EXEC {
        return Err(uefi::Error::new(uefi::Status::UNSUPPORTED, "Invalid type"));
    }

    // Verify machine
    if hdr.e_machine != EM_AMD64 {
        return Err(uefi::Error::new(
            uefi::Status::UNSUPPORTED,
            "Invalid machine",
        ));
    }

    // Load the execuatable
    let mut phdr_ptr = unsafe { file.as_ptr().offset(hdr.e_phoff as isize) };
    let mut phdr = unsafe { &*(phdr_ptr as *const Elf64Phdr) };
    let mut i = 0;
    while i < hdr.e_phnum {
        if phdr.p_type == PT_LOAD {
            uefi::memory::allocate_pages(phdr.p_memsz as usize, phdr.p_paddr)?;

            if phdr.p_filesz > 0 {
                uefi::memory::copy_mem(
                    phdr.p_paddr as *mut c_void,
                    unsafe { file.as_ptr().offset(phdr.p_offset as isize) as *const c_void },
                    phdr.p_filesz as usize,
                );
            }

            let diff = phdr.p_memsz - phdr.p_filesz;
            let start = (phdr.p_paddr + phdr.p_filesz) as *mut u8;
            let mut i = 0;
            while i < diff {
                unsafe { *(start.offset(i as isize)) = 0 };

                i += 1;
            }
        }

        i += 1;
        phdr_ptr = unsafe { phdr_ptr.offset(hdr.e_phentsize as isize) };
        phdr = unsafe { &*(phdr_ptr as *const Elf64Phdr) };
    }

    Ok(hdr.e_entry as usize)
}
