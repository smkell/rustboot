pub mod int;
pub mod memory;

pub static mut allocator: memory::BuddyAlloc = memory::BuddyAlloc {
    base: 0x110_000,
    order: 14,
    storage: memory::Bitv { storage: 0x100_000 as *mut [u32, ..0x8_000 / 4] }
};
