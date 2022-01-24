use stivale_boot::v2::{StivaleFramebufferHeaderTag, StivaleHeader, StivaleTerminalHeaderTag};

static STACK: [u8; 4096] = [0; 4096];

static TERMINAL_HDR_TAG: StivaleTerminalHeaderTag = StivaleTerminalHeaderTag::new();

static FRAMEBUFFER_HDR_TAG: StivaleFramebufferHeaderTag = StivaleFramebufferHeaderTag::new()
    .next((&TERMINAL_HDR_TAG as *const StivaleTerminalHeaderTag).cast());

#[used]
#[link_section = ".stivale2hdr"]
static STIVALE_HDR: StivaleHeader = StivaleHeader::new()
    .stack(STACK[0] as *const u8)
    .tags((&FRAMEBUFFER_HDR_TAG as *const StivaleFramebufferHeaderTag).cast());
