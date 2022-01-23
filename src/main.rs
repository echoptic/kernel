#![no_std]
#![no_main]

use core::arch::asm;

use stivale_boot::v2::{
    StivaleFramebufferHeaderTag, StivaleHeader, StivaleStruct, StivaleTerminalHeaderTag,
};

static STACK: [u8; 4096] = [0; 4096];

static TERMINAL_HDR_TAG: StivaleTerminalHeaderTag = StivaleTerminalHeaderTag::new();

static FRAMEBUFFER_HDR_TAG: StivaleFramebufferHeaderTag = StivaleFramebufferHeaderTag::new()
    .next((&TERMINAL_HDR_TAG as *const StivaleTerminalHeaderTag).cast());

#[used]
#[link_section = ".stivale2hdr"]
static STIVALE_HDR: StivaleHeader = StivaleHeader::new()
    .stack(STACK[0] as *const u8)
    .tags((&FRAMEBUFFER_HDR_TAG as *const StivaleFramebufferHeaderTag).cast());

#[no_mangle]
fn kmain(stivale_struct: &StivaleStruct) -> ! {
    let term_str_tag = stivale_struct.terminal().unwrap();
    let term_write = term_str_tag.term_write();
    term_write("Hello, world!");

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
