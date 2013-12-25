pub mod int;
pub mod memory;

pub static mut allocator: memory::BuddyAlloc = memory::BuddyAlloc {
    base: 0x100000,
    order: 12,
    storage: memory::Bitv { storage: 0x105000 as *mut [u32, ..2048] }
};
