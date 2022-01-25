#![no_std]
#![no_main]
#![feature(default_alloc_error_handler)]
#![feature(abi_x86_interrupt)]

extern crate alloc;

mod allocator;
mod boot;
mod date_time;
mod drivers;
mod interrupts;
mod terminal;

use stivale_boot::v2::StivaleStruct;
use x86_64::instructions;

use crate::date_time::{Date, Time};

fn init(stivale_struct: &'static StivaleStruct) {
    interrupts::init_idt();
    terminal::init_writer(stivale_struct);
    allocator::init(stivale_struct);
}

#[no_mangle]
fn kmain(stivale_struct: &'static StivaleStruct) -> ! {
    init(stivale_struct);

    kprintln!("TIME: {}", Time::get_current());
    kprintln!("DATE: {}", Date::get_current());

    loop {
        instructions::hlt();
    }
}

#[panic_handler]
fn panic_handler(info: &core::panic::PanicInfo) -> ! {
    kprintln!("PANIC: {:#?}", info);
    loop {
        instructions::hlt();
    }
}
