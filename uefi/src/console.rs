use crate::efi;
use alloc::vec::Vec;
use core::fmt::{self, Write};

pub struct Console(
    &'static efi::SIMPLE_TEXT_OUTPUT_PROTOCOL,
    *const efi::SIMPLE_TEXT_OUTPUT_PROTOCOL,
);

static mut STANDARD_OUTPUT: Option<Console> = None;

pub fn initialize(system_table: &efi::SYSTEM_TABLE) -> Result<(), crate::Error> {
    let stdout = Console::new(system_table.console_out)?;

    unsafe { STANDARD_OUTPUT = Some(stdout) };

    Ok(())
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    match unsafe { &mut STANDARD_OUTPUT } {
        None => {}
        Some(console) => console.write_fmt(args).unwrap(),
    }
}

impl Console {
    pub fn new(
        simple_text_output_interface: *const efi::SIMPLE_TEXT_OUTPUT_PROTOCOL,
    ) -> Result<Self, crate::Error> {
        let console = Console(
            crate::from_pointer(simple_text_output_interface),
            simple_text_output_interface,
        );

        console.clear_screen()?;
        console.set_cursor_pos(0, 0)?;

        Ok(console)
    }

    pub fn clear_screen(&self) -> Result<(), crate::Error> {
        let status = unsafe { (self.0.clear_screen)(self.1) };
        match status {
            efi::STATUS::SUCCESS => Ok(()),
            _ => Err(crate::Error::new(status, "Failed to clear standard output")),
        }
    }

    pub fn set_cursor_pos(&self, column: usize, row: usize) -> Result<(), crate::Error> {
        let status = unsafe { (self.0.set_cursor_pos)(self.1, column, row) };
        match status {
            efi::STATUS::SUCCESS => Ok(()),
            _ => Err(crate::Error::new(status, "Failed to clear standard error")),
        }
    }
}

impl fmt::Write for Console {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        let mut str: Vec<efi::CHAR16> = Vec::with_capacity(s.len() + 1);

        for c in s.chars() {
            str.push(c as efi::CHAR16);
        }

        str.push(0);

        match unsafe { (self.0.output_string)(self.1, str.as_ptr()) } {
            efi::STATUS::SUCCESS => Ok(()),
            _ => Err(fmt::Error {}),
        }
    }
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::console::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\r\n"));
    ($($arg:tt)*) => ($crate::print!("{}\r\n", format_args!($($arg)*)));
}
