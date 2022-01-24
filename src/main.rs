#![no_std]
#![no_main]
#![feature(default_alloc_error_handler)]
#![feature(abi_x86_interrupt)]

extern crate alloc;

mod allocator;
mod boot;
mod interrupts;
mod terminal;

use alloc::vec::Vec;
use stivale_boot::v2::StivaleStruct;
use x86_64::instructions;

fn init(stivale_struct: &'static StivaleStruct) {
    terminal::init_writer(stivale_struct);
    interrupts::init_idt();
    allocator::init(stivale_struct);
}

#[no_mangle]
fn kmain(stivale_struct: &'static StivaleStruct) -> ! {
    init(stivale_struct);

    instructions::interrupts::int3();

    let mut v = Vec::new();
    for i in 0..100 {
        v.push(i);
    }
    kprintln!("{:?}", v);

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
