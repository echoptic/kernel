#![no_std]
#![no_main]

mod boot;
mod terminal;

use core::arch::asm;

use stivale_boot::v2::StivaleStruct;

#[no_mangle]
fn kmain(stivale_struct: &'static StivaleStruct) -> ! {
    terminal::init_terminal_writer(stivale_struct);

    kprintln!("Hello, {}!", "world!");

    loop {
        unsafe {
            asm!("hlt");
        }
    }
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {
        unsafe {
            asm!("hlt");
        }
    }
}
