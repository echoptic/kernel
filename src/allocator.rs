use slab_allocator_rs::{LockedHeap, MIN_HEAP_SIZE};
use stivale_boot::v2::{StivaleMemoryMapEntryType, StivaleStruct};

#[global_allocator]
static ALLOCATOR: LockedHeap = LockedHeap::empty();

pub fn init(stivale_struct: &StivaleStruct) {
    for addr in stivale_struct.memory_map().unwrap().iter() {
        if addr.entry_type == StivaleMemoryMapEntryType::Usable {
            let heap_start_addr = addr.base as usize;
            unsafe {
                ALLOCATOR.init(heap_start_addr, MIN_HEAP_SIZE);
            }
            break;
        }
    }
    if ALLOCATOR.lock().is_none() {
        panic!("Error initializing allocator!");
    }
}
