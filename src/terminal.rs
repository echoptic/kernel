use core::fmt::Write;

use stivale_boot::v2::StivaleStruct;

pub struct TermWriter<'a> {
    stivale_struct: Option<&'a StivaleStruct>,
}

impl<'a> Write for TermWriter<'a> {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        Ok(self
            .stivale_struct
            .unwrap()
            .terminal()
            .unwrap()
            .term_write()(s))
    }
}

pub static mut TERM_WRITER: TermWriter = TermWriter {
    stivale_struct: None,
};

pub fn init_terminal_writer(stivale_struct: &'static StivaleStruct) {
    unsafe {
        TERM_WRITER.stivale_struct = Some(stivale_struct);
    }
}

#[macro_export]
macro_rules! kprint {
	($($arg:tt)+) => ({
		use core::fmt::Write;
        use crate::terminal::TERM_WRITER;

        let _ = unsafe { TERM_WRITER.write_fmt(format_args!($($arg)+)) };

	});
}

#[macro_export]
macro_rules! kprintln {
	() => ({
		$crate::kprint!("\n");
	});
	($($arg:tt)+) => ({
		$crate::kprint!($($arg)+);
		$crate::kprint!("\n");
	});
}
