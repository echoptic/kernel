use core::fmt::Write;

use spin::Mutex;
use stivale_boot::v2::StivaleStruct;

pub fn init_writer(stivale_struct: &'static StivaleStruct) {
    WRITER.lock().stivale_struct = Some(stivale_struct);
}

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

pub static WRITER: Mutex<TermWriter> = Mutex::new(TermWriter {
    stivale_struct: None,
});

#[macro_export]
macro_rules! kprint {
	($($arg:tt)+) => ({
		use core::fmt::Write;
        use crate::terminal::WRITER;

        let _ = WRITER.lock().write_fmt(format_args!($($arg)+));

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
