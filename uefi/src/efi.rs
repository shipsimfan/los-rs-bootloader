#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

/*
 * ================================================================
 * || 2.3.1 Data types
 * ================================================================
 */

pub const FALSE: BOOLEAN = 0;
pub const TRUE: BOOLEAN = 1;

pub type BOOLEAN = u8;
pub type INTN = isize;
pub type UINTN = usize;
pub type INT8 = i8;
pub type UINT8 = u8;
pub type INT16 = i16;
pub type UINT16 = u16;
pub type INT32 = i32;
pub type UINT32 = u32;
pub type INT64 = i64;
pub type UINT64 = u64;
pub type INT128 = i128;
pub type UINT128 = u128;
pub type CHAR8 = u8;
pub type CHAR16 = u16;
pub type VOID = core::ffi::c_void;
pub type HANDLE = *const VOID;
pub type EVENT = *const VOID;
pub type LBA = UINT64;
pub type TPL = UINTN;
pub type MAC_ADDRESS = [u8; 32];
pub type IPv4_ADDRESS = [u8; 4];
pub type IPv6_ADDRESS = [u8; 16];
pub type IP_ADDRESS = [u8; 16];

#[repr(C)]
pub struct GUID {
    a: u32,
    b: u16,
    c: u16,
    d: [u8; 8],
}

/*
 * ================================================================
 * || 4.2 EFI Table Header
 * ================================================================
 */

#[repr(C)]
pub struct TABLE_HEADER {
    pub signature: UINT64,
    pub revision: UINT32,
    pub header_size: UINT32,
    pub crc32: UINT32,
    pub reserved: UINT32,
}

/*
 * ================================================================
 * || 4.3 EFI System Table
 * ================================================================
 */

pub const SYSTEM_TABLE_SIGNATURE: UINT64 = 0x5453595320494249;
#[repr(C)]
pub struct SYSTEM_TABLE {
    pub header: TABLE_HEADER,
    pub firmware_vendor: *const CHAR16,
    pub firmware_revision: UINT32,
    pub console_in_handle: HANDLE,
    pub console_in: *const VOID,
    pub console_out_handle: HANDLE,
    pub console_out: *const SIMPLE_TEXT_OUTPUT_PROTOCOL,
    pub standard_error_handle: HANDLE,
    pub standard_error: *const SIMPLE_TEXT_OUTPUT_PROTOCOL,
    pub runtime_services: *const VOID,
    pub boot_services: *const BOOT_SERVICES,
    pub number_of_table_entries: UINTN,
    pub configuration_table: *const VOID,
}

/*
 * ================================================================
 * || 4.4 EFI Boot Services Table
 * ================================================================
 */

pub const BOOT_SERVICES_SIGNATURE: UINT64 = 0x56524553544F4F42;
#[repr(C)]
pub struct BOOT_SERVICES {
    pub header: TABLE_HEADER,
    // Task priority services
    pub raise_tpl: *const VOID,
    pub restore_tpl: *const VOID,
    // Memory services
    pub allocate_pages: ALLOCATE_PAGES,
    pub free_pages: FREE_PAGES,
    pub get_memory_map: GET_MEMORY_MAP,
    pub allocate_pool: ALLOCATE_POOL,
    pub free_pool: FREE_POOL,
    // Event and timer services
    pub create_event: *const VOID,
    pub set_timer: *const VOID,
    pub wait_for_timer: *const VOID,
    pub signal_event: *const VOID,
    pub close_event: *const VOID,
    pub check_event: *const VOID,
    // Protocol handler services
    pub install_protocol_interface: *const VOID,
    pub reinstall_protocol_interface: *const VOID,
    pub uninstall_protocol_interface: *const VOID,
    pub handle_protocol: HANDLE_PROTOCOL,
    pub reserved: *const VOID,
    pub register_protocol_notify: *const VOID,
    pub locate_handle: *const VOID,
    pub locate_device_path: *const VOID,
    pub install_configuration_table: *const VOID,
    // Image services
    pub load_image: *const VOID,
    pub start_image: *const VOID,
    pub exit: *const VOID,
    pub unload_image: *const VOID,
    pub exit_boot_services: *const VOID,
    // Miscellaneous Services
    pub get_next_montonic_count: *const VOID,
    pub stall: *const VOID,
    pub set_watchdog_timer: SET_WATCHDOG_TIMER,
    // DriverSupport services
    pub connect_controller: *const VOID,
    pub disconnect_controller: *const VOID,
    // Open and close protocol services
    pub open_protocol: *const VOID,
    pub close_protocol: *const VOID,
    pub open_protocol_information: *const VOID,
    // Library Services
    pub protocols_per_handle: *const VOID,
    pub locate_handle_buffer: *const VOID,
    pub locate_protocol: *const VOID,
    pub install_multiple_protocol_interfaces: *const VOID,
    pub uninstall_multiple_protocol_interfaces: *const VOID,
    // 32-Bit CRC services
    pub calculate_crc32: *const VOID,
    // Miscellaneous services
    pub copy_mem: COPY_MEM,
    pub set_mem: *const VOID,
    pub create_event_ex: *const VOID,
}

/*
 * ================================================================
 * || 7.2 Memory Allocation Services
 * ================================================================
 */

#[repr(C)]
pub enum ALLOCATE_TYPE {
    AllocateAnyPages,
    AllocateMaxAddress,
    AllocateAddress,
    MaxAllocateType,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum MEMORY_TYPE {
    ReservedMemoryType,
    LoaderCode,
    LoaderData,
    BootServicesCode,
    BootServicesData,
    RuntimeServicesCode,
    RuntimeServicesData,
    ConventionalMemory,
    UnusableMemory,
    ACPIReclaimMemory,
    ACPIMemoryNVS,
    MemoryMappedIO,
    MemoryMappedIOPortSpace,
    PalCode,
    PersistentMemory,
    MaxMemoryType,
}

pub const MEMORY_UC: UINT64 = 0x1;
pub const MEMORY_WC: UINT64 = 0x2;
pub const MEMORY_WT: UINT64 = 0x4;
pub const MEMORY_WB: UINT64 = 0x8;
pub const MEMORY_UCE: UINT64 = 0x10;
pub const MEMORY_WP: UINT64 = 0x1000;
pub const MEMORY_RP: UINT64 = 0x2000;
pub const MEMORY_XP: UINT64 = 0x4000;
pub const MEMORY_NV: UINT64 = 0x8000;
pub const MEMORY_MORE_RELIABLE: UINT64 = 0x10000;
pub const MEMORY_RO: UINT64 = 0x20000;
pub const MEMORY_SP: UINT64 = 0x40000;
pub const MEMORY_CPU_CRYPTO: UINT64 = 0x80000;
pub const MEMORY_RUNTIME: UINT64 = 0x8000000000000000;

#[repr(C)]
pub struct MEMORY_DESCRIPTOR {
    pub memory_type: UINT32,
    pub physical_start: PHYSICAL_ADDRESS,
    pub virtual_start: VIRTUAL_ADDRESS,
    pub number_of_pages: UINT64,
    pub attribute: UINT64,
}

pub type PHYSICAL_ADDRESS = UINT64;
pub type VIRTUAL_ADDRESS = UINT64;

pub type ALLOCATE_PAGES = unsafe extern "efiapi" fn(
    allocate_type: ALLOCATE_TYPE,
    memory_type: MEMORY_TYPE,
    pages: UINTN,
    memory: *mut PHYSICAL_ADDRESS,
) -> STATUS;
pub type FREE_PAGES = unsafe extern "efiapi" fn(memory: PHYSICAL_ADDRESS, pages: UINTN) -> STATUS;
pub type GET_MEMORY_MAP = unsafe extern "efiapi" fn(
    memory_map_size: *mut UINTN,
    memory_map: *mut MEMORY_DESCRIPTOR,
    map_key: *mut UINTN,
    descriptor_size: UINTN,
    descriptor_size: UINT32,
) -> STATUS;
pub type ALLOCATE_POOL = unsafe extern "efiapi" fn(
    pool_type: MEMORY_TYPE,
    size: UINTN,
    buffer: *mut *const VOID,
) -> STATUS;
pub type FREE_POOL = unsafe extern "efiapi" fn(buffer: *const VOID) -> STATUS;

/*
 * ================================================================
 * || 7.3 Protocol Handler Services
 * ================================================================
 */

pub type HANDLE_PROTOCOL = unsafe extern "efiapi" fn(
    handle: HANDLE,
    protocol: *const GUID,
    interface: *mut *const VOID,
) -> STATUS;

/*
 * ================================================================
 * || 7.5 Miscellaneous Boot Services
 * ================================================================
 */

pub type SET_WATCHDOG_TIMER = unsafe extern "efiapi" fn(
    timeout: UINTN,
    watchdog_code: UINT64,
    data_size: UINTN,
    watchdog_data: *const CHAR16,
) -> STATUS;

pub type COPY_MEM =
    unsafe extern "efiapi" fn(destination: *mut VOID, source: *const VOID, length: UINTN);

/*
 * ================================================================
 * || 8.3 Time Services
 * ================================================================
 */

#[repr(C)]
pub struct TIME {
    year: UINT16,
    month: UINT8,
    day: UINT8,
    hour: UINT8,
    minute: UINT8,
    second: UINT8,
    pad1: UINT8,
    nanosecond: UINT32,
    time_zone: INT16,
    daylight: UINT8,
    pad2: UINT8,
}

/*
 * ================================================================
 * || 9.1 EFI Loaded Image Protocol
 * ================================================================
 */

pub const LOADED_IMAGE_PROTOCOL_GUID: GUID = GUID {
    a: 0x5B1B31A1,
    b: 0x9562,
    c: 0x11D2,
    d: [0x8E, 0x3F, 0x00, 0xA0, 0xC9, 0x69, 0x72, 0x3B],
};

#[repr(C)]
pub struct LOADED_IMAGE_PROTOCOL {
    pub revision: UINT32,
    pub parent_handle: HANDLE,
    pub system_table: *const SYSTEM_TABLE,
    // Source location of the image
    pub device_handle: HANDLE,
    pub file_path: *const VOID,
    pub reserved: *const VOID,
    // Image's load options
    pub load_options_size: UINT32,
    pub load_options: *const VOID,
    // Location where image was loaded
    pub image_base: *const VOID,
    pub image_size: UINT64,
    pub image_code_type: MEMORY_TYPE,
    pub image_data_type: MEMORY_TYPE,
    pub unload: *const VOID,
}

/*
 * ================================================================
 * || 12.4 Simple Text Output Protocol
 * ================================================================
 */

const SIMPLE_TEXT_OUTPUT_PROTOCOL_GUID: GUID = GUID {
    a: 0x387477C2,
    b: 0x69C7,
    c: 0x11D2,
    d: [0x8E, 0x39, 0x00, 0xA0, 0xC9, 0x69, 0x72, 0x3B],
};

#[repr(C)]
pub struct SIMPLE_TEXT_OUTPUT_PROTOCOL {
    pub reset: *const VOID,
    pub output_string: TEXT_STRING,
    pub test_string: *const VOID,
    pub query_mode: *const VOID,
    pub set_mode: *const VOID,
    pub set_attribute: *const VOID,
    pub clear_screen: TEXT_CLEAR_SCREEN,
    pub set_cursor_pos: TEXT_SET_CURSOR_POSITION,
    pub enable_cursor: *const VOID,
    pub mode: *const VOID,
}

pub type TEXT_STRING = unsafe extern "efiapi" fn(
    this: *const SIMPLE_TEXT_OUTPUT_PROTOCOL,
    string: *const CHAR16,
) -> STATUS;
pub type TEXT_CLEAR_SCREEN =
    unsafe extern "efiapi" fn(this: *const SIMPLE_TEXT_OUTPUT_PROTOCOL) -> STATUS;
pub type TEXT_SET_CURSOR_POSITION = unsafe extern "efiapi" fn(
    this: *const SIMPLE_TEXT_OUTPUT_PROTOCOL,
    column: UINTN,
    row: UINTN,
) -> STATUS;

/*
 * ================================================================
 * || 13.4 Simple File System Protocol
 * ================================================================
 */

pub const SIMPLE_FILE_SYSTEM_PROTOCOL_GUID: GUID = GUID {
    a: 0x0964E5B22,
    b: 0x6459,
    c: 0x11D2,
    d: [0x8E, 0x39, 0x00, 0xA0, 0xC9, 0x69, 0x72, 0x3B],
};

pub struct SIMPLE_FILE_SYSTEM_PROTOCOL {
    pub revision: UINT64,
    pub open_volume: SIMPLE_FILE_SYSTEM_OPEN_VOLUME,
}

pub type SIMPLE_FILE_SYSTEM_OPEN_VOLUME = unsafe extern "efiapi" fn(
    this: *const SIMPLE_FILE_SYSTEM_PROTOCOL,
    root: *mut *const FILE_PROTOCOL,
) -> STATUS;

/*
 * ================================================================
 * || 13.5 File Protocol
 * ================================================================
 */

pub const FILE_MODE_READ: UINT64 = 1;
pub const FILE_MODE_WRITE: UINT64 = 2;
pub const FILE_MODE_CREATE: UINT64 = 0x8000000000000000;

pub const FILE_READ_ONLY: UINT64 = 0x1;
pub const FILE_HIDDEN: UINT64 = 0x2;
pub const FILE_SYSTEM: UINT64 = 0x4;
pub const FILE_RESERVED: UINT64 = 0x8;
pub const FILE_DIRECTORY: UINT64 = 0x10;
pub const FILE_ARCHIVE: UINT64 = 0x20;
pub const FILE_VALID_ATTR: UINT64 = 0x37;

pub const FILE_INFO_ID: GUID = GUID {
    a: 0x09576E92,
    b: 0x6D3F,
    c: 0x11D2,
    d: [0x8E, 0x39, 0x00, 0xA0, 0xC9, 0x69, 0x72, 0x3B],
};

#[repr(C)]
pub struct FILE_PROTOCOL {
    pub revision: UINT64,
    pub open: FILE_OPEN,
    pub close: FILE_CLOSE,
    pub delete: *const VOID,
    pub read: FILE_READ,
    pub write: *const VOID,
    pub get_position: *const VOID,
    pub set_position: *const VOID,
    pub get_info: FILE_GET_INFO,
    pub set_info: *const VOID,
    pub flush: *const VOID,
    pub open_ex: *const VOID,
    pub read_ex: *const VOID,
    pub write_ex: *const VOID,
    pub flush_ex: *const VOID,
}

#[repr(C)]
pub struct FILE_INFO {
    pub size: UINT64,
    pub file_size: UINT64,
    pub physical_size: UINT64,
    pub create_time: TIME,
    pub last_access_time: TIME,
    pub modification_time: TIME,
    pub attribute: UINT64,
    pub filename: CHAR16,
}

pub type FILE_OPEN = unsafe extern "efiapi" fn(
    this: *const FILE_PROTOCOL,
    new_handle: *mut *const FILE_PROTOCOL,
    file_name: *const CHAR16,
    open_mode: UINT64,
    attributes: UINT64,
) -> STATUS;
pub type FILE_CLOSE = unsafe extern "efiapi" fn(this: *const FILE_PROTOCOL) -> STATUS;
pub type FILE_GET_INFO = unsafe extern "efiapi" fn(
    this: *const FILE_PROTOCOL,
    information_type: *const GUID,
    buffer_size: *mut UINTN,
    buffer: *mut VOID,
) -> STATUS;
pub type FILE_READ = unsafe extern "efiapi" fn(
    this: *const FILE_PROTOCOL,
    buffer_size: *mut UINTN,
    buffer: *mut VOID,
) -> STATUS;

/*
 * ================================================================
 * || Appendix D - Status Codes
 * ================================================================
 */

pub const fn ERROR(code: UINTN) -> UINTN {
    0x8000000000000000 | code
}

#[repr(usize)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum STATUS {
    SUCCESS = 0,
    WARN_UNKOWN_GLYPH = 1,
    WARN_DELETE_FAILURE = 2,
    WARN_WRITE_FAILURE = 3,
    WARN_BUFFER_TOO_SMALL = 4,
    WARN_STALE_DATA = 5,
    WARN_FILE_SYSTEM = 6,
    WARN_RESET_REQUIRED = 7,
    LOAD_ERROR = ERROR(1),
    INVALID_PARAMETER = ERROR(2),
    UNSUPPORTED = ERROR(3),
    BAD_BUFFER_SIZE = ERROR(4),
    BUFFER_TOO_SMALL = ERROR(5),
    NOT_READY = ERROR(6),
    DEVICE_ERROR = ERROR(7),
    WRITE_PROTECTED = ERROR(8),
    OUT_OF_RESOURCES = ERROR(9),
    VOLUME_CORRUPTED = ERROR(10),
    VOLUME_FULL = ERROR(11),
    NO_MEDIA = ERROR(12),
    MEDIA_CHANGED = ERROR(13),
    NOT_FOUND = ERROR(14),
    ACCESS_DENIED = ERROR(15),
    NO_RESPONSE = ERROR(16),
    NO_MAPPING = ERROR(17),
    TIMEOUT = ERROR(18),
    NOT_STARTED = ERROR(19),
    ALREADY_STARTED = ERROR(20),
    ABORTED = ERROR(21),
    ICMP_ERROR = ERROR(22),
    TFTP_ERROR = ERROR(23),
    PROTOCOL_ERROR = ERROR(24),
    INCOMPATIBLE_VERSION = ERROR(25),
    SECURITY_VIOLATION = ERROR(26),
    CRC_ERROR = ERROR(27),
    END_OF_MEDIA = ERROR(28),
    END_OF_FILE = ERROR(31),
    INVALID_LANGUAGE = ERROR(32),
    COMPROMISED_DATA = ERROR(33),
    IP_ADDRESS_CONFLICT = ERROR(34),
    HTTP_ERROR = ERROR(35),
}

impl core::fmt::Display for STATUS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "{} ({:#X})",
            match self {
                STATUS::SUCCESS => "Success",
                STATUS::WARN_UNKOWN_GLYPH => "Unknown Glyph",
                STATUS::WARN_DELETE_FAILURE => "Delete Failure",
                STATUS::WARN_WRITE_FAILURE => "Write Failure",
                STATUS::WARN_BUFFER_TOO_SMALL => "Buffer Too Small",
                STATUS::WARN_STALE_DATA => "Stale Data",
                STATUS::WARN_FILE_SYSTEM => "File System",
                STATUS::WARN_RESET_REQUIRED => "Reset Required",
                STATUS::LOAD_ERROR => "Load Error",
                STATUS::INVALID_PARAMETER => "Invalid Parameter",
                STATUS::UNSUPPORTED => "Unsupported",
                STATUS::BAD_BUFFER_SIZE => "Bad Buffer Size",
                STATUS::BUFFER_TOO_SMALL => "Buffer Too Small",
                STATUS::NOT_READY => "Not Ready",
                STATUS::DEVICE_ERROR => "Device Error",
                STATUS::WRITE_PROTECTED => "Write Protected",
                STATUS::OUT_OF_RESOURCES => "Out of Resources",
                STATUS::VOLUME_CORRUPTED => "Volume Corrupted",
                STATUS::VOLUME_FULL => "Volume Full",
                STATUS::NO_MEDIA => "No Media",
                STATUS::MEDIA_CHANGED => "Media Changed",
                STATUS::NOT_FOUND => "Not Found",
                STATUS::ACCESS_DENIED => "Access Denied",
                STATUS::NO_RESPONSE => "No Response",
                STATUS::NO_MAPPING => "No Mapping",
                STATUS::TIMEOUT => "Timeout",
                STATUS::NOT_STARTED => "Not Started",
                STATUS::ALREADY_STARTED => "Already Started",
                STATUS::ABORTED => "Aborted",
                STATUS::ICMP_ERROR => "ICMP Error",
                STATUS::TFTP_ERROR => "TFTP Error",
                STATUS::PROTOCOL_ERROR => "Protocol Error",
                STATUS::INCOMPATIBLE_VERSION => "Incompatible Version",
                STATUS::SECURITY_VIOLATION => "Security Violation",
                STATUS::CRC_ERROR => "CRC Error",
                STATUS::END_OF_MEDIA => "End of Media",
                STATUS::END_OF_FILE => "End of File",
                STATUS::INVALID_LANGUAGE => "Invalid Language",
                STATUS::COMPROMISED_DATA => "Compromised Data",
                STATUS::IP_ADDRESS_CONFLICT => "IP Address Conflict",
                STATUS::HTTP_ERROR => "HTTP Error",
            },
            *self as usize
        )
    }
}
