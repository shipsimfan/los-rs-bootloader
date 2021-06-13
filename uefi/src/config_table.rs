use core::{ffi::c_void, ptr::null};

use crate::efi;

pub type GUID = efi::GUID;

struct ConfigurationTable {
    table: *const efi::CONFIGURATION_TABLE,
    num_tables: usize,
}

static mut CONFIGURATION_TABLE: ConfigurationTable = ConfigurationTable {
    table: null(),
    num_tables: 0,
};

pub const ACPI_20_RSDP_GUID: GUID = GUID {
    a: 0x8868E871,
    b: 0xE4F1,
    c: 0x11D3,
    d: [0xBC, 0x22, 0x00, 0x80, 0xC7, 0x3C, 0x88, 0x81],
};

pub fn initialize(system_table: &efi::SYSTEM_TABLE) {
    unsafe {
        CONFIGURATION_TABLE = ConfigurationTable {
            table: system_table.configuration_table,
            num_tables: system_table.number_of_table_entries,
        }
    }
}

pub fn get_config_table(guid: GUID) -> Result<*const c_void, crate::Error> {
    let mut ect = unsafe { CONFIGURATION_TABLE.table };
    let num_tables = unsafe { CONFIGURATION_TABLE.num_tables };
    let mut i = 0;
    while i < num_tables {
        if unsafe { (*ect).vendor_guid == guid } {
            return Ok(unsafe { (*ect).vendor_table });
        }

        i += 1;
        ect = unsafe { ect.offset(1) };
    }

    Err(crate::Error::new(
        efi::STATUS::NOT_FOUND,
        "Failed to get table",
    ))
}
